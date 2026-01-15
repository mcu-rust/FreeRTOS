use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

// See: https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-check-cfg=cfg(cortex_m)");
    let target = env::var("TARGET").unwrap();
    if target.starts_with("thumbv") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=feature=\"cpu-clock\"");
        println!("cargo:DEF___IS_CORTEX_M=1");
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:CRATE_DIR={}", manifest_dir.to_str().unwrap());
    println!(
        "cargo:KERNEL={}",
        manifest_dir.join("FreeRTOS-Kernel").to_str().unwrap()
    );
    println!(
        "cargo:SHIM={}",
        manifest_dir.join("src/freertos").to_str().unwrap()
    );

    let feature_define_map = HashMap::from([
        ("delete-task", "INCLUDE_vTaskDelete"),
        ("delay-until", "INCLUDE_vTaskDelayUntil"),
        ("stack-high-water", "INCLUDE_uxTaskGetStackHighWaterMark"),
        ("heap-free-size", "INCLUDE_HeapFreeSize"),
        ("task-suspend", "INCLUDE_vTaskSuspend"),
        ("recursive-mutex", "configUSE_RECURSIVE_MUTEXES"),
        ("counting-semaphore", "configUSE_COUNTING_SEMAPHORES"),
        ("trace-facility", "configUSE_TRACE_FACILITY"),
    ]);

    for (ft, def) in feature_define_map.iter() {
        if check_feature(ft) {
            println!("cargo:DEF_{}=1", def);
        }
    }
}

fn check_feature(name: &str) -> bool {
    let s = name.replace("-", "_");
    let ft = "CARGO_FEATURE_".to_string() + &s.to_uppercase();
    env::var(ft).map_or(false, |v| v == "1")
}
