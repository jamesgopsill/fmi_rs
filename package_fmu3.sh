set -e

FMU_DIR="fmu"
PKG_NAME=$(cargo read-manifest | jq -r '.name')

echo "Building ${PKG_NAME}..."

rm -rf "{$FMU_DIR}"
mkdir -p "${FMU_DIR}/binaries/x86_64-windows"

echo "Copying files..."

# Look up one as in workspace.
cp "../target/x86_64-pc-windows-gnu/release/${PKG_NAME}.dll" "${FMU_DIR}/binaries/x86_64-windows/"
cp ./modelDescription.xml "${FMU_DIR}/"

# Creating a new GUID on build
NEW_GUID="{$(uuidgen)}"
echo "Build GUID: ${NEW_GUID}"
sed -i "s/guid=\"[^\"]*\"/guid=\"${NEW_GUID}\"/g" "${FMU_DIR}/modelDescription.xml"

echo "Packaging into fmu..."
(cd "${FMU_DIR}" && zip -r "../../target/${PKG_NAME}.fmu" .)

echo "Cleaning up..."

rm -rf "${FMU_DIR}"
