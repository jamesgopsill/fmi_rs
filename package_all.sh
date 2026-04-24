set -e

(cd "examples/add_fmi2" && bash package.sh)
(cd "examples/add_fmi3" && bash package.sh)
(cd "examples/counter" && bash package.sh)
(cd "examples/multiplier" && bash package.sh)
