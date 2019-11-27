* download cargo from your package repository
* install rustc
* `cargo init`
* `rustup target add wasm32-unknown-unknown`
* `cargo build --target=wasm32-unknown-unknown`

your exported .wasm file will be in /target/wasm32