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
        println!("cargo:rustc-cfg=feature=\"cpu_clock\"");
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!(
        "cargo:KERNEL={}",
        manifest_dir.join("FreeRTOS-Kernel").to_str().unwrap()
    );
    println!(
        "cargo:SHIM={}",
        manifest_dir.join("src/freertos").to_str().unwrap()
    );

    let feature_define_map = HashMap::from([
        ("delete_task", "INCLUDE_vTaskDelete"),
        ("delay_until", "INCLUDE_vTaskDelayUntil"),
    ]);

    for (ft, def) in feature_define_map.iter() {
        if check_feature(ft) {
            println!("cargo:DEF_{}=1", def);
        }
    }
}

fn check_feature(s: &str) -> bool {
    let ft = "CARGO_FEATURE_".to_string() + &s.to_uppercase();
    env::var(ft).map_or(false, |v| v == "1")
}
