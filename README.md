# RMON

Remote MONitoring, a system monitoring tool written in Rust.

## Scope of this project

- [x] Client monitoring tool
- [ ] Remote collection server
- [ ] Web interface

## RMON-Client

This is the client-side application which functions as a collector for the hardware statistics.

### Run
```bash
cd client
cargo build --release
cargo run --release        # Or simply run ./target/release/rmon-client
```
