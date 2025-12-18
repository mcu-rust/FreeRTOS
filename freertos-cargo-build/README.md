# freertos-build

[![Crates.io](https://img.shields.io/crates/v/freertos-build.svg)](https://crates.io/crates/freertos-build)


Helper crate for building FreeRTOS applications with Cargo and Rust using a `build.rs`.

See also [freertos-next README](https://crates.io/crates/freertos-next/freertos-rust/README.md).

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
