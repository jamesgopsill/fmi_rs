import os
import shutil
import subprocess
import json
import time
import uuid
from pathlib import Path
import datetime
import xml.etree.ElementTree as ET

FMU_DIR = Path("target") / "fmu"


def uuid7() -> str:
    ms = int(time.time() * 1000)
    rand = os.urandom(10)
    uuid_int = (
        (ms << 80)
        | (7 << 76)
        | ((rand[0] & 0x0F) << 72)
        | (rand[1] << 64)
        | (2 << 62)
        | (int.from_bytes(rand[2:], "big"))
    )
    return str(uuid.UUID(int=uuid_int))


def shell_cmd(cmd: str) -> str:
    result = subprocess.run(cmd, capture_output=True, shell=True)
    if result.returncode != 0:
        print(f"[ERROR] (cmd) {result.stderr}")
        exit(1)
    return result.stdout.decode().strip()


def stream_shell_cmd(cmd: str):
    process = subprocess.Popen(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, shell=True, text=True
    )
    if process.stdout is None:
        print("[ERROR] no stdout")
        exit(1)
    for line in process.stdout:
        print(line.strip())
    process.wait()
    if process.returncode != 0:
        print(f"[ERROR] Command failed with exit code {process.returncode}")
        exit(1)


def update_model_description_xml(
    pkg_name: str,
    pkg_description: str,
    pkg_version: str,
):
    git_remote_url = shell_cmd("git remote get-url origin")
    print(f"[INFO] GIT URL: {git_remote_url}")

    guid = uuid7()
    print(f"[INFO] GUID: {guid}")
    build_date = datetime.datetime.now(datetime.UTC).strftime("%Y-%m-%dT%H:%M:%SZ")

    tree = ET.parse("modelDescription.xml")
    root = tree.getroot()

    root.set("modelName", "fmu--" + pkg_name)
    root.set("description", pkg_description)
    root.set("guid", guid)
    root.set("version", pkg_version)
    root.set("generationTool", "fmi_rs")
    root.set("generationDateAndTime", build_date)

    me = root.find("ModelExchange")
    if me is not None:
        me.set("modelIdentifier", pkg_name)

    # Vendor Annotations
    annotations = ET.Element("VendorAnnotations")
    tool = ET.SubElement(annotations, "Tool")
    tool.set("name", "SourceControl")
    repo = ET.SubElement(tool, "Repository")
    repo.set("url", git_remote_url)

    # Insert the vendor annotations above the ModelVariables
    model_variables = root.find("ModelVariables")
    for index, child in enumerate(root):
        if child == model_variables:
            root.insert(index, annotations)
            break

    # Pretty print xml
    for el in root.iter():
        if el.text is not None and not el.text.strip():
            el.text = None
        if el.tail is not None and not el.tail.strip():
            el.tail = None
    ET.indent(root, space="  ")

    print("[INFO] Writing the XML")
    xml_out = FMU_DIR / "modelDescription.xml"
    tree.write(xml_out, encoding="utf-8", xml_declaration=True)


def copy_binaries(pkg_name: str):
    # Windows
    dll = Path("target") / "x86_64-pc-windows-gnu" / "release" / f"{pkg_name}.dll"
    if dll.exists():
        print("[INFO] Found Windows Binary")
        bin_path = FMU_DIR / "binaries" / "win64"
        bin_path.mkdir(parents=True)
        shutil.copy(dll, bin_path)
    # Linux
    so = Path("target") / "x86_64-unknown-linux-gnu" / "release" / f"lib{pkg_name}.so"
    if so.exists():
        print("[INFO] Found Linux binary")
        bin_path = FMU_DIR / "binaries" / "linux64"
        bin_path.mkdir(parents=True)
        # renaming binary
        bin_path = bin_path / f"{pkg_name}.so"
        shutil.copy(so, bin_path)
    # MacOS (TODO)


def copy_docs():
    doc_path = FMU_DIR / "documentation"
    doc_path.mkdir(parents=True)
    if not Path("README.md").exists():
        print("[ERROR] README.md does not exist.")
        exit(1)

    icon = Path("model.png")
    if icon.exists():
        shutil.copy("model.png", FMU_DIR)
    else:
        print(
            "[WARNING] No model.png found. If you want a icon for your fmu then add a model.png file to the root of your project."
        )

    readme = Path("README.md")
    if not readme.exists():
        print("[ERROR] No README.md found.")
        exit(1)


def make_archive(pkg_name: str) -> Path:
    fmu_out = Path("target") / f"{pkg_name}.fmu"
    if fmu_out.exists():
        fmu_out.unlink()

    shutil.make_archive(str(fmu_out.with_suffix("")), "zip", FMU_DIR)
    zip_tmp = fmu_out.with_suffix(".zip")
    zip_tmp.rename(fmu_out)
    return fmu_out


def main():
    if FMU_DIR.exists():
        print("[INFO] Removing old fmu temp directory.")
        shutil.rmtree(FMU_DIR)
    FMU_DIR.mkdir(parents=True)

    print("[INFO] Building FMU")
    stream_shell_cmd("cargo build --release --target=x86_64-pc-windows-gnu")
    stream_shell_cmd("cargo build --release --target=x86_64-unknown-linux-gnu")

    print("[INFO] Fetching metadata from cargo")
    manifest_raw = shell_cmd("cargo read-manifest")
    manifest_json = json.loads(manifest_raw)

    pkg_name = manifest_json["name"]
    print(f"[INFO] PKG NAME: {pkg_name}")
    pkg_version = manifest_json["version"]
    print(f"[INFO] PKG VERSION: {pkg_version}")
    pkg_description = manifest_json["description"]
    print(f"[INFO] PKG DESCRIPTION: {pkg_description}")

    update_model_description_xml(
        pkg_name=pkg_name, pkg_description=pkg_description, pkg_version=pkg_version
    )

    # Copying the files
    copy_binaries(pkg_name=pkg_name)

    copy_docs()

    fmu_path = make_archive(pkg_name=pkg_name)

    print("[INFO] Cleaning up.")

    shutil.rmtree(FMU_DIR)

    print(f"[SUCCESS] FMU can be found at {fmu_path}.")


if __name__ == "__main__":
    main()
