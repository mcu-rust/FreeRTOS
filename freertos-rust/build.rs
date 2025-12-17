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

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:SHIM={}",
        PathBuf::from(manifest_dir)
            .join("src/freertos")
            .to_str()
            .unwrap()
    );
}
