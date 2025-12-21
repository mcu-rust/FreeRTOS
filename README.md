# FreeRTOS Workspace
[![Crates.io](https://img.shields.io/crates/v/freertos-next.svg)](https://crates.io/crates/freertos-next)

This project is based on code from [freertos.rs](https://github.com/hashmismatch/freertos.rs) and [FreeRTOS-rust](https://github.com/lobaro/FreeRTOS-rust) and some additions to
 simplify the usage of [FreeRTOS](https://github.com/FreeRTOS/FreeRTOS-Kernel) in embedded applications written in Rust.

Crate sub projects:
- [freertos](freertos) The runtime dependency for you FreeRTOS Rust application.
- [freertos-build](freertos-build) A tool to build the FreeRTOS-Kernel.

## How it works

The `freertos-build` build-dependency compiles the FreeRTOS code from its original "C" sources files into an
archive to be linked against your Rust app. Internally it uses the [cc crate](https://docs.rs/crate/cc) and some meta
info provided by your apps `build.rs`: A path to the app specific `FreeRTOSConfig.h`. You can copy one from [examples](freertos-rust-examples/examples).

 The `freertos` dependency provides an interface to access all FreeRTOS functionality from your (embedded) Rust app.

## Usage

See [freertos README](freertos/README.md)

## Examples
[stm32f1-FreeRTOS-example](https://github.com/mcu-rust/stm32f1-FreeRTOS-example) is a more complete example for STM32F1.

And there are some simple examples in [freertos-rust-examples](freertos-rust-examples) for:

* Cortex M33 (nRF9160)
* Cortex M3 (STM32F103C8)
* Cortex M4 (STM32F411CE)
* Windows
* Linux


# License
This repository is using the MIT License. Some parts might state different licenses that need to be respected when used.

* The [Linux port](https://github.com/michaelbecker/freertos-addons) is licensed under GPLv2

## 🔖 Keywords

freertos · rtos · rust · embedded · embedded-hal · no-std · scheduler · multitasking · bindings · wrapper
