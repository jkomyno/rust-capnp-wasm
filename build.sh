#!/bin/sh

# 1. Build the Rust library / update the Rust capnp bindings
(
  cd ./rust \
    && cargo build -p event-capnp --release --target wasm32-unknown-unknown
)

# 2. Build the WebAssembly modules
wasm-bindgen --target deno \
  --out-dir ./src/wasm \
  ./rust/target/wasm32-unknown-unknown/release/event_capnp.wasm

# 3. Update the TypeScript capnp bindings
capnpc -o ts ./src/capnp/event.capnp
