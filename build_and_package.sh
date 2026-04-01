#!/bin/bash

# Exit if a command exits with a non-zero status
set -e

cargo build --release --target=x86_64-pc-windows-gnu

(cd "counter_fmu" && bash ../package_fmu2.sh)
(cd "multiplier_fmu" && bash ../package_fmu2.sh)
(cd "addv2_fmu" && bash ../package_fmu2.sh)
(cd "addv3_fmu" && bash ../package_fmu3.sh)
