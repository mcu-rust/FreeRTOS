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
use freertos_build::prelude::*;

fn main() {
    let mut b = freertos_build::Builder::new();
    b.cpu_clock(72.MHz());
    b.heap_size(10 * 1024);
    b.minimal_stack_size(80);
    b.interrupt_priority_bits(4, 5, 15);
    b.compile().unwrap();
}
```
Optional configuration:
```rust
// Use your own FreeRTOS-Kernel source tree
b.freertos_kernel("path/to/FreeRTOS-Kernel");

// Override the default port (relative to FreeRTOS-Kernel/portable)
b.freertos_port("GCC/ARM_CM3");

// Use UserConfig.h
b.user_config_dir("path/to/config/directory");

// Override the internal FreeRTOSConfig.h
b.freertos_config_dir("path/to/config/directory");

// Select the heap allocator from FreeRTOS-Kernel/portable/MemMang
// Default: heap_4.c
b.heap("heap_4.c");

b.use_timer_task(1, 10, 200);
b.max_task_priorities(5);
b.use_preemption(true);
b.idle_should_yield(true);
b.max_task_name_len(16);
b.queue_registry_size(8);
b.check_for_stack_overflow(2);
```
