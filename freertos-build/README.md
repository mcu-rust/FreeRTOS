# freertos-build

[![CI](https://github.com/mcu-rust/freertos/workflows/CI/badge.svg)](https://github.com/mcu-rust/freertos/actions)
[![Crates.io](https://img.shields.io/crates/v/freertos-build.svg)](https://crates.io/crates/freertos-build)
[![Docs.rs](https://docs.rs/freertos-build/badge.svg)](https://docs.rs/freertos-build)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](./LICENSE)
[![Downloads](https://img.shields.io/crates/d/freertos-build.svg)](https://crates.io/crates/freertos-build)

As part of the [freertos-next](https://crates.io/crates/freertos-next) ecosystem, `freertos-build` provides a simple and reliable way to build FreeRTOS applications using Cargo.  
It integrates FreeRTOS into your Rust project through a `build.rs`, automatically compiling the kernel sources and applying your custom `FreeRTOSConfig.h`.

See also [freertos README](https://github.com/mcu-rust/FreeRTOS/tree/main/freertos).

## 📦 Usage

```sh
cargo add --build freertos-build
```

Add the following snippet to your application's `build.rs`:
```rust
fn main() {
    let mut b = freertos_build::Builder::new();
    b.freertos_config("src_c"); // Location of `FreeRTOSConfig.h`
    b.compile().unwrap();
}
```
