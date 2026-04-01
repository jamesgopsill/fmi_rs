#!/bin/bash

# Exit if a command exits with a non-zero status
set -e

cargo build --release --target=x86_64-pc-windows-gnu

FMU_DIR="fmu"
PKG_NAME=$(cargo read-manifest | jq -r '.name')

echo "Building ${PKG_NAME}..."

rm -rf "{$FMU_DIR}"
mkdir -p "${FMU_DIR}/binaries/win64"

echo "Copying files..."

cp "./target/x86_64-pc-windows-gnu/release/${PKG_NAME}.dll" "${FMU_DIR}/binaries/win64/"
cp ./modelDescription.xml "${FMU_DIR}/"

echo "Packaging into fmu..."

(cd "${FMU_DIR}" && zip -r "../target/${PKG_NAME}.fmu" .)

echo "Cleaning up..."

rm -rf "${FMU_DIR}"

echo "Done"
