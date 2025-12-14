import subprocess

subprocess.run("cargo publish --manifest-path freertos-rust/Cargo.toml".split())
subprocess.run("cargo publish --manifest-path freertos-cargo-build/Cargo.toml".split())
