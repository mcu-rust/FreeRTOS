import subprocess

subprocess.run("cargo publish --manifest-path freertos/Cargo.toml".split())
subprocess.run("cargo publish --manifest-path freertos-build/Cargo.toml".split())
