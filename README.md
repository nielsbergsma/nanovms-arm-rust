# NanoVMs - Rust - ARM example

This repository contains the code accompanying the medium article [found here]().

In short, it demonstrates running NanoVM unikernel on Google Cloud T2A (arm) instances with Rust.
## Install the dependencies
### Rust compiler via Rustup
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Protobuf compiler (optional)
```shell
# on Mac OS X
brew install protobuf
# or on Linux, APT-based distros
apt install protobuf-compiler
```

### Rust cross-compile toolkit
```shell
cargo install cross --git https://github.com/cross-rs/cross
```

### OPS from NanoVMs
```shell
curl https://ops.city/get.sh -sSfL | sh
```

## Deploy instructions
### Step 1. Cross-compile the project for Aarch64 (arm64) architecture
```shell
RUSTFLAGS='-C target-feature=+crt-static' cross build --release --target aarch64-unknown-linux-gnu
```

### Step 2. Create the VM image on GCP
```shell
ops image create target/aarch64-unknown-linux-gnu/release/api -c deployment/config.json -i api-v1 -t gcp
```

### Step 3. Create an VM instance on GCP with that VM image
```shell
ops instance create api-v1 -c deployment/config.json -t gcp 
```

The last step targets an instance group, if you wish to deploy just a single instance, and expose it directly with a public ip-address. Remove `RunConfig.InstanceGroup` property from `deployment/config.json` and execute:
```shell
ops instance create api-v1 -c deployment/config.json p 80 -t gcp
```