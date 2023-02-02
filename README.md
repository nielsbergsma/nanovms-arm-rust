# NanoVMs - Rust - ARM example


## Compile instructions
```shell
RUSTFLAGS='-C target-feature=+crt-static' cross build --release --target aarch64-unknown-linux-gnu

ops image create target/aarch64-unknown-linux-gnu/release/api -c deployment/config.json -i api-v1 -t gcp
ops instance create api-v1 -c deployment/config.json -t gcp 
```