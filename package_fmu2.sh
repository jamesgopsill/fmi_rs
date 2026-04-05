set -e

FMU_DIR="fmu"
PKG_NAME=$(cargo read-manifest | jq -r '.name')
PKG_VERSION=$(cargo read-manifest | jq -r '.version')
PKG_DESCRIPTION=$(cargo read-manifest | jq -r '.description')

echo "Building ${PKG_NAME}..."

# Remove old dir if exists
rm -rf "{$FMU_DIR}"
mkdir -p "${FMU_DIR}/binaries/win64"

echo "Copying files..."

# Look up one as in workspace.
cp "../target/x86_64-pc-windows-gnu/release/${PKG_NAME}.dll" "${FMU_DIR}/binaries/win64/"
cp ./modelDescription.xml "${FMU_DIR}/"

# Creating a new GUID on build
BUILD_GUID="{$(uuidgen)}"
BUILD_DATE="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
echo "Build GUID: ${BUILD_GUID}"
echo "Updating xml..."
sed -i "s/guid=\"[^\"]*\"/guid=\"${BUILD_GUID}\"/g" "${FMU_DIR}/modelDescription.xml"
sed -i "s/version=\"[^\"]*\"/version=\"${PKG_VERSION}\"/g" "${FMU_DIR}/modelDescription.xml"
sed -i "s/description=\"[^\"]*\"/description=\"${PKG_DESCRIPTION}\"/g" "${FMU_DIR}/modelDescription.xml"
sed -i "s/modelIdentifier=\"[^\"]*\"/modelIdentifier=\"${PKG_NAME}\"/g" "${FMU_DIR}/modelDescription.xml"
sed -i "s/generationDateAndTime=\"[^\"]*\"/generationDateAndTime=\"${BUILD_DATE}\"/g" "${FMU_DIR}/modelDescription.xml"

cat "${FMU_DIR}/modelDescription.xml"

# Packaging
echo "Packaging into fmu..."
(cd "${FMU_DIR}" && zip -r "../../target/${PKG_NAME}.fmu" .)

echo "Cleaning up..."
rm -rf "${FMU_DIR}"
