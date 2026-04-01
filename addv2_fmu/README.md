# Counter FMU

An example implementation of a counter FMU in Rust conforming to the [FMI2 specification](https://github.com/modelica/fmi-standard/releases/download/v2.0.5/FMI-Specification-2.0.5.pdf).

# Build and bundle

To build and bundle the built artefacts into a `.fmu` zip run the following bash script in the root directory of the project. The build targets windows for now.

```bash
bash ./build_fmu.sh
```

You will find the fmu at `./target/counter.fmu`.
