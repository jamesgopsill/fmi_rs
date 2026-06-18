use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::Command,
};

use cargo_metadata::MetadataCommand;
use chrono::Utc;
use minidom::{Element, Node};
use uuid::Uuid;
use zip::{ZipWriter, write::SimpleFileOptions};

fn main() {
    // Notes.
    // - Should be run from the root dir
    build("x86_64-pc-windows-gnu");
    build("x86_64-unknown-linux-gnu");

    let path = Path::new("target/sub.fmu");
    let file = File::create(path).expect("Error initialising archive.");
    let mut zip = ZipWriter::new(file);

    let opts = SimpleFileOptions::default();

    zip.start_file("binaries/win64/sub.dll", opts).unwrap();
    let mut dll = File::open("target/x86_64-pc-windows-gnu/release/sub.dll").unwrap();
    let mut buffer = Vec::new();
    dll.read_to_end(&mut buffer).unwrap();
    zip.write_all(&buffer).unwrap();

    zip.start_file("binaries/linux64/sub.so", opts).unwrap();
    let mut lib = File::open("target/x86_64-unknown-linux-gnu/release/libsub.so").unwrap();
    let mut buffer = Vec::new();
    lib.read_to_end(&mut buffer).unwrap();
    zip.write_all(&buffer).unwrap();

    zip.start_file("documentation/README.md", opts).unwrap();
    let mut lib = File::open("README.md").unwrap();
    let mut buffer = Vec::new();
    lib.read_to_end(&mut buffer).unwrap();
    zip.write_all(&buffer).unwrap();

    let meta = metadata();

    let ns = "http://fmi-standard.org/fmi-model-description.xml";

    let xml = fs::read_to_string("modelDescription.xml").unwrap();
    let mut root: Element = xml.parse().unwrap();

    root.set_attr("".into(), "generationTool".try_into().unwrap(), "fmi_rs");
    root.set_attr(
        "".into(),
        "modelName".try_into().unwrap(),
        format!("fmu--{}", meta.name),
    );
    root.set_attr(
        "".into(),
        "description".try_into().unwrap(),
        meta.description,
    );
    root.set_attr("".into(), "version".try_into().unwrap(), meta.version);

    let uuid = Uuid::now_v7().to_string();
    root.set_attr("".into(), "guid".try_into().unwrap(), uuid);

    let now = Utc::now().to_rfc3339();
    root.set_attr("".into(), "generationDateAndTime".try_into().unwrap(), now);

    // Find Model Exchange
    let me = root
        .get_child_mut("ModelExchange", ns)
        .expect("Could not find ModelExchange");
    me.set_attr("".into(), "modelIdentifier".try_into().unwrap(), meta.name);

    // Add model annotations
    let mut vendor_annotation = Element::builder("VendorAnnotations", ns).build();
    let child = Element::builder("Tool", ns)
        .attr("name".try_into().unwrap(), "SourceControl")
        .build();
    vendor_annotation.append_child(child);
    let child = Element::builder("Repository", ns)
        .attr("url".try_into().unwrap(), meta.repo)
        .build();
    vendor_annotation.append_child(child);

    let mut nodes = root.take_nodes();
    let pos = nodes
        .iter()
        .position(|n| {
            if let Node::Element(el) = n {
                el.is("ModelVariables", ns)
            } else {
                false
            }
        })
        .unwrap();
    nodes.insert(pos, vendor_annotation.into());

    for n in nodes {
        root.append_node(n);
    }

    let mut writer = Vec::new();
    root.write_to(&mut writer).unwrap();
    // fs::write("out.xml", writer).unwrap();

    zip.start_file("modelDescription.xml", opts).unwrap();
    zip.write_all(&writer).unwrap();
}

fn build(target: &str) {
    let cargo = env::var("CARGO").expect("Cargo should be present in your environment variables");
    let status = Command::new(cargo)
        .current_dir("sub")
        .args([
            "build",
            "--release",
            format!("--target={}", target).as_str(),
        ])
        .status()
        .expect("Error building.");

    if !status.success() {
        panic!("Cargo build failed.");
    }
}

struct Metadata {
    name: String,
    version: String,
    description: String,
    repo: String,
}

fn metadata() -> Metadata {
    // 1. Fetch metadata for the entire workspace
    let metadata = MetadataCommand::new()
        .no_deps() // We only care about our own workspace members, not all dependencies
        .exec()
        .expect("Failed to get cargo metadata");

    let target_package_name = "sub";

    let package = metadata
        .workspace_packages()
        .into_iter()
        .find(|p| p.name == target_package_name)
        .expect("Could not find the target package in the workspace");

    // 3. Access the information
    let name = package.name.to_string();
    let version = package.version.to_string();
    let description = package.description.to_owned().unwrap_or_default();

    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .unwrap();
    let repo = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Metadata {
        name,
        version,
        description,
        repo,
    }
}
