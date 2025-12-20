# FreeRTOS

[![Crates.io](https://img.shields.io/crates/v/freertos-next.svg)](https://crates.io/crates/freertos-next)

Wrapper library to use FreeRTOS API in Rust.

- It includes the source code of [FreeRTOS-Kernel](https://github.com/FreeRTOS/FreeRTOS-Kernel). Current version is `V11.2.0`.
  - If you have any specific requirements, please prepare your source code and call `b.freertos("path/to/your/kernel");` in your `build.rs` file.
- It implements some useful traits:
  - [os-trait](https://crates.io/crates/os-trait)
  - [critical-section](https://crates.io/crates/critical-section).
  - [mutex-traits](https://crates.io/crates/mutex-traits).

It's named **freertos-next** on crates.io right now, because we can't choose a shorter name.

## Usage

1. Add dependencies to your Rust APP
```shell
cargo add freertos-next
cargo add --build freertos-build
```

2. Add this snippet to your APP's `build.rs`:
```rust
fn main() {
    let mut b = freertos_build::Builder::new();
    b.freertos_config("src_c"); // Location of `FreeRTOSConfig.h`
    b.compile().unwrap();
}
```
3. Optional:
```rust
// If you want to use you own source code.
b.freertos("path/to/FreeRTOS-Kernel");
// Path relative to 'FreeRTOS-Kernel/portable'.
// If the default path is not what you want.
b.freertos_port("GCC/ARM_CM3");
// Set the heap_?.c allocator to use from 'FreeRTOS-Kernel/portable/MemMang'
// (Default: heap_4.c)
b.heap("heap_4.c");
```


It needs [freertos-build](https://crates.io/crates/freertos-build) to work with. [stm32f1-FreeRTOS-example](https://github.com/mcu-rust/stm32f1-FreeRTOS-example) shows how to use this crate with [stm32f1-hal](https://crates.io/crates/stm32f1-hal) together.

### Used C compiler
`freertos-build` depends on the [cc crate](https://docs.rs/crate/cc). So the C compiler
used can be set by using the `CC` enviroment variable or otherwise defined by internal
defaults. For the ARM architecture this is the `arm-none-eabi-gcc` which can be found [here](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).

Install:
```shell
# on Ubuntu
sudo apt-get install -y gcc-arm-none-eabi
# on Windows
scoop install gcc-arm-none-eabi
```

See also [repository](https://github.com/mcu-rust/FreeRTOS).
