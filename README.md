# FreeRTOS-Next
[![Crates.io](https://img.shields.io/crates/v/freertos-next.svg)](https://crates.io/crates/freertos-next)

This project is based on code from [freertos.rs](https://github.com/hashmismatch/freertos.rs) and [FreeRTOS-rust](https://github.com/lobaro/FreeRTOS-rust) and some additions to
 simplify the usage of [FreeRTOS](https://github.com/FreeRTOS/FreeRTOS-Kernel) in embedded applications written in Rust.

Project Crates:
- [freertos-next](freertos-rust) The runtime dependency for you FreeRTOS Rust application.
- [freertos-build](freertos-cargo-build) A tool to build the FreeRTOS-Kernel.

## How it works

The `freertos-build` build-dependency compiles the FreeRTOS code from its original "C" sources files into an
archive to be linked against your Rust app. Internally it uses the [cc crate](https://docs.rs/crate/cc) and some meta
info provided by your apps `build.rs`: A path to the app specific `FreeRTOSConfig.h`. You can copy one from [examples](freertos-rust-examples/examples).

 The `freertos-next` dependency provides an interface to access all FreeRTOS functionality from your (embedded) Rust app.

## Usage

See [freertos-next README](freertos-rust/README.md)

## Examples
To get started there are examples in [freertos-rust-examples](freertos-rust-examples) for:

* Cortex M33 (nRF9160)
* Cortex M3 (STM32F103C8)
* Cortex M4 (STM32F411CE)
* Windows
* ...more to come...


# License
This repository is using the MIT License. Some parts might state different licenses that need to be respected when used.

* The [Linux port](https://github.com/michaelbecker/freertos-addons) is licensed under GPLv2
