# `fmi_rs`

A lightweight trait and ffi generation tool for creating Functional Model Units in Rust.

## Structure

The repo uses cargo workspaces with the `fmi_rs` containing the library with the fmi trait and macros for the FMI2 spec (more specs coming). The `counter_fmu` and `multiplier_fmu` provide examples of how the use the crate as well as example build scripts that build and bundle the code into `.fmu` archives to be used on windows.

## Want more automation?

The [`fmi`](https://github.com/jondo2010/rust-fmi) crate provides greater automation and integration that can be useful for larger FMUs where the `modelDescription.xml` and bundling can be performed automatically using `fmi-export`.
