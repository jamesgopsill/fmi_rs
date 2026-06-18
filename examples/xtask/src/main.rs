use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    process::{Command, exit},
};

use cargo_metadata::MetadataCommand;
use chrono::Utc;
use minidom::{Element, Node};
use uuid::Uuid;
use zip::{ZipWriter, write::SimpleFileOptions};

fn main() {
    let fmu_metadata = workspace_package_data();

    build_all("x86_64-pc-windows-gnu");
    build_all("x86_64-unknown-linux-gnu");

    for fmu in fmu_metadata {
        println!("Packaging {}", fmu.name);
        let path = format!("target/{}.fmu", fmu.name);
        let path = Path::new(path.as_str());
        let file = File::create(path).expect("Error initialising archive.");
        let mut zip = ZipWriter::new(file);

        let opts = SimpleFileOptions::default();
        let path = format!("binaries/win64/{}.dll", fmu.name);
        zip.start_file(path, opts).unwrap();
        let path = format!("target/x86_64-pc-windows-gnu/release/{}.dll", fmu.name);
        let dll = fs::read(path).unwrap();
        zip.write_all(&dll).unwrap();

        let path = format!("binaries/linux64/{}.so", fmu.name);
        zip.start_file(path, opts).unwrap();
        let path = format!("target/x86_64-unknown-linux-gnu/release/lib{}.so", fmu.name);
        let lib = fs::read(path).unwrap();
        zip.write_all(&lib).unwrap();

        zip.start_file("documentation/README.md", opts).unwrap();
        let path = format!("{}/README.md", fmu.name);
        let lib = fs::read(path).unwrap();
        zip.write_all(&lib).unwrap();

        let path = format!("{}/model.png", fmu.name);
        if fs::exists(&path).unwrap() {
            zip.start_file("model.png", opts).unwrap();
            let lib = fs::read(path).unwrap();
            zip.write_all(&lib).unwrap();
        }

        let ns = "http://fmi-standard.org/fmi-model-description.xml";

        let path = format!("{}/modelDescription.xml", fmu.name);
        println!("{}", path);
        let xml = fs::read_to_string(path).unwrap();
        let mut root: Element = xml.parse().unwrap();

        root.set_attr("".into(), "generationTool".try_into().unwrap(), "fmi_rs");
        root.set_attr(
            "".into(),
            "modelName".try_into().unwrap(),
            format!("fmu--{}", fmu.name),
        );
        root.set_attr(
            "".into(),
            "description".try_into().unwrap(),
            fmu.description,
        );
        root.set_attr("".into(), "version".try_into().unwrap(), fmu.version);

        let Some(version) = root.attr("fmiVersion") else {
            println!("[ERROR] No fmi version found");
            exit(1);
        };

        match version {
            "3.0" => {
                let uuid = Uuid::now_v7().to_string();
                root.set_attr("".into(), "instantiationToken".try_into().unwrap(), uuid);
            }
            "2.0" => {
                let uuid = Uuid::now_v7().to_string();
                root.set_attr("".into(), "guid".try_into().unwrap(), uuid);
            }
            _ => {
                println!("[ERROR] No fmi version found");
                exit(1);
            }
        }

        let now = Utc::now().to_rfc3339();
        root.set_attr("".into(), "generationDateAndTime".try_into().unwrap(), now);

        // Find Model Exchange and/or CoSimulation
        // TODO: add a check if neither exist.
        if let Some(me) = root.get_child_mut("ModelExchange", ns) {
            me.set_attr(
                "".into(),
                "modelIdentifier".try_into().unwrap(),
                fmu.name.clone(),
            );
        }
        if let Some(cs) = root.get_child_mut("CoSimulation", ns) {
            cs.set_attr(
                "".into(),
                "modelIdentifier".try_into().unwrap(),
                fmu.name.clone(),
            );
        }

        // Add model annotations
        let mut vendor_annotation = Element::builder("VendorAnnotations", ns).build();
        let child = Element::builder("Tool", ns)
            .attr("name".try_into().unwrap(), "SourceControl")
            .build();
        vendor_annotation.append_child(child);
        let child = Element::builder("Repository", ns)
            .attr("url".try_into().unwrap(), fmu.repo)
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
        zip.start_file("modelDescription.xml", opts).unwrap();
        zip.write_all(&writer).unwrap();
    }
}

struct FmuMetadata {
    name: String,
    version: String,
    description: String,
    repo: String,
}

fn workspace_package_data() -> Vec<FmuMetadata> {
    // 1. Fetch metadata for the entire workspace
    let metadata = MetadataCommand::new()
        .no_deps() // We only care about our own workspace members, not all dependencies
        .exec()
        .expect("Failed to get cargo metadata");

    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .unwrap();

    let repo = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let mut fmu_metadata: Vec<FmuMetadata> = Vec::new();

    for package in metadata.packages {
        let name = package.name.to_string();
        if name == "xtask" {
            continue;
        }
        let version = package.version.to_string();
        let description = package.description.to_owned().unwrap_or_default();
        let fmu_meta = FmuMetadata {
            name,
            version,
            description,
            repo: repo.clone(),
        };
        fmu_metadata.push(fmu_meta)
    }

    fmu_metadata
}

fn build_all(target: &str) {
    let cargo = env::var("CARGO").expect("Cargo should be present in your environment variables");
    let status = Command::new(cargo)
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

/*

fn main() {
    let meta = MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("Failed to fetch cargo metadata");

    let workspace_package_names: Vec<String> = meta
        .workspace_packages()
        .into_iter()
        .map(|p| p.name.to_string())
        .collect();

    if !workspace_package_names.contains(&fmu) {
        println!("[ERROR] arg does not match a package in the workspace.");
        exit(1);
    }

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

fn build(fmu: &str, target: &str) {
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
*/
