Binary file for wasm module, has to be built before you can install node_modules in parent directory.

Pre-requesites:
* Install [rustup](https://github.com/rust-lang/rustup.rs). This should also install `cargo`, a package manager for Rust.
* `cargo install wasm-pack`


1. `cargo build`
2. `wasm-pack build`
