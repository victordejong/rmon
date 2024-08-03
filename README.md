# RMON

Remote MONitoring, a system monitoring tool written in Rust.

# Installation

Run `rmon-installer.sh` with supported arguments:

```bash
./rmon-installer.sh [install | uninstall | install-src] [client | server]
```

(Currently only `install-src server`) is supported.

## Scope of this project

- [x] Client monitoring tool
- [x] Remote collection server
- [ ] Web interface

## RMON-Client

This is the client-side application which functions as a collector for the hardware statistics.

### Features

- Collect CPU, Memory metrics in a configurable interval
- Collect disk info using a user provided list
- Read configuration from disk, commandline or environment variables
- Print collected metrics to console

### Run
```bash
cd client
cargo build --release
cargo run --release        # Or simply run ./target/release/rmon-client
```

## RMON-Server

TODO
