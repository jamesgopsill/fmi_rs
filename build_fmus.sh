#!/bin/bash

# Exit if a command exits with a non-zero status
set -e

(cd "counter_fmu" && bash build_fmu.sh)
(cd "multiplier_fmu" && bash build_fmu.sh)
