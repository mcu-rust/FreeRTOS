# freertos-build
Helper crate for building FreeRTOS applications with Cargo and Rust using a `build.rs`.

To build an embedded application with FreeRTOS please refer
to [home](https://crates.io/crates/freertos-next).


## Usage

```shel
cargo add --build freertos-build
```

Add this snippet to your apps `build.rs`:
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
