# freertos-build

[![Crates.io](https://img.shields.io/crates/v/freertos-build.svg)](https://crates.io/crates/freertos-build)


Helper crate for [freertos-next](https://crates.io/crates/freertos-next) by building FreeRTOS applications with Cargo and Rust using a `build.rs`.

See also [freertos README](https://github.com/mcu-rust/FreeRTOS/tree/main/freertos).

## Usage

```shel
cargo add --build freertos-build
```

Add this snippet to your apps `build.rs`:
```rust
fn main() {
    let mut b = freertos_build::Builder::new();
    b.freertos_config("src_c"); // Location of `FreeRTOSConfig.h`
    b.compile().unwrap();
}
```
