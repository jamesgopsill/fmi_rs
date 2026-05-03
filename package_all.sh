set -e

(cd "examples/add_fmi2" && bash package.sh)
(cd "examples/add_fmi3" && bash package.sh)
(cd "examples/counter" && bash package.sh)
(cd "examples/mul" && bash package.sh)
(cd "examples/sub" && bash package.sh)
(cd "examples/spring_mass_damper" && bash package.sh)
