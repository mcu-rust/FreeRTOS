# FreeRTOS-Next

[![Crates.io](https://img.shields.io/crates/v/freertos-next.svg)](https://crates.io/crates/freertos-next)

Wrapper library to use FreeRTOS API in Rust.

To build an embedded application with FreeRTOS please refer
to [repository](https://github.com/mcu-rust/FreeRTOS).
It needs [freertos-build](https://crates.io/crates/freertos-build) to work with.

 [stm32f1-FreeRTOS-example](https://github.com/mcu-rust/stm32f1-FreeRTOS-example) shows how to use this crate with [stm32f1-hal](https://crates.io/crates/stm32f1-hal) together.

It implements some useful traits:
- [os-trait](https://crates.io/crates/os-trait)
- [critical-section](https://crates.io/crates/critical-section).
- [mutex-traits](https://crates.io/crates/mutex-traits).

## Usage

1. Checkout FreeRTOS: https://github.com/FreeRTOS/FreeRTOS-Kernel

1. Add dependencies to your Rust apps `Cargo.toml`
```shell
cargo add freertos-next
cargo add --build freertos-build
```

1. Add this snippet to your apps `build.rs`:
```rust
fn main() {
    let mut b = freertos_build::Builder::new();

    // Path to FreeRTOS kernel or set ENV "FREERTOS_SRC" instead
    b.freertos("path/to/FreeRTOS-Kernel");
    b.freertos_config("src");       // Location of `FreeRTOSConfig.h`
    b.freertos_port("GCC/ARM_CM3"); // Port dir relativ to 'FreeRTOS-Kernel/portable'
    b.heap("heap_4.c");             // Set the heap_?.c allocator to use from
                                    // 'FreeRTOS-Kernel/portable/MemMang' (Default: heap_4.c)

    // b.get_cc().file("More.c");   // Optional additional C-Code to be compiled

    b.compile().unwrap_or_else(|e| { panic!("{}", e.to_string()) });
}
```
