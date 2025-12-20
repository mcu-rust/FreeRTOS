# FreeRTOS

[![Crates.io](https://img.shields.io/crates/v/freertos-next.svg)](https://crates.io/crates/freertos-next)
[![Downloads](https://img.shields.io/crates/d/freertos-next.svg)](https://crates.io/crates/freertos-next)
[![Docs.rs](https://docs.rs/freertos-next/badge.svg)](https://docs.rs/freertos-next)

`freertos-next` is a Rust wrapper for the FreeRTOS API.

- It bundles the official [FreeRTOS-Kernel](https://github.com/FreeRTOS/FreeRTOS-Kernel) sources (currently `V11.2.0`).  
  If you need a customized setup, you can supply your own kernel tree and call `b.freertos("path/to/your/kernel");` in your `build.rs`.
- It implements several traits to ensure smooth integration with Rust tooling:
  - [`os-trait`](https://crates.io/crates/os-trait)
  - [`critical-section`](https://crates.io/crates/critical-section)
  - [`mutex-traits`](https://crates.io/crates/mutex-traits)

The crate is published as **freertos-next** on crates.io because the more obvious names (`freertos`, `freertos-rust`) are already taken.

## 📦 Usage

1. Add the dependencies to your application:

```sh
cargo add freertos-next
cargo add --build freertos-build
```
2. Add the following snippet to your application's `build.rs`:
```rust
fn main() {
    let mut b = freertos_build::Builder::new();
    b.freertos_config("src_c"); // Path to your FreeRTOSConfig.h
    b.compile().unwrap();
}
```
3. Optional configuration:
```rust
// Use your own FreeRTOS-Kernel source tree
b.freertos("path/to/FreeRTOS-Kernel");

// Override the default port (relative to FreeRTOS-Kernel/portable)
b.freertos_port("GCC/ARM_CM3");

// Select the heap allocator from FreeRTOS-Kernel/portable/MemMang
// Default: heap_4.c
b.heap("heap_4.c");
```
[freertos-next](https://github.com/mcu-rust/FreeRTOS/tree/main/freertos) works together with [freertos-build](https://crates.io/crates/freertos-build). A complete example using [freertos-next](https://github.com/mcu-rust/FreeRTOS/tree/main/freertos) with [stm32f1-hal](https://crates.io/crates/stm32f1-hal) is available here: [stm32f1-FreeRTOS-example](https://github.com/mcu-rust/stm32f1-FreeRTOS-example)

## 📘 C Compiler

`freertos-build` uses the [`cc`](https://docs.rs/crate/cc) crate to compile the FreeRTOS kernel.  
The C compiler can be configured via the `CC` environment variable, or it will fall back to the defaults provided by `cc`.

For ARM targets, the expected compiler is `arm-none-eabi-gcc`, which can be obtained from the [ARM GNU toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).

### Install

```sh
# Ubuntu
sudo apt-get install -y gcc-arm-none-eabi

# Windows (Scoop)
scoop install gcc-arm-none-eabi
```
For more details, see the main [repository](https://github.com/mcu-rust/FreeRTOS).

## 🔖 Keywords

freertos · rtos · rust · embedded · embedded-hal · no-std · arm · cortex-m · scheduler · multitasking · bindings · wrapper
