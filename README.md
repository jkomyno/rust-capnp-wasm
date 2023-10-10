# Towards Zero-Copy Binary (De)Serialization: TypeScript ↔ Rust

---

Slides for this talk are also available [here](https://jkomyno-trivago-rust-meetup-2023.vercel.app/).

## Abstract

JSON is the de facto standard for sharing data between services, but its neither type-safe nor cheap to parse. Enter the world of zero-copy binary protocols for Rust and TypeScript applications. Starting from commonly found - but non trivial - datatypes and their JSON schema definition, we will discuss how to model them in Cap'n Proto: a proven, zero-copy binary protocol used at Cloudflare, and crafted by the creator of Protobuf. Ready to level up your cross-language data handling prowess?

### Requirements

- [Rust 1.73.0](https://www.rust-lang.org/tools/install) or superior*
- [Deno 1.36.0](https://docs.deno.com/runtime/manual/getting_started/installation) or superior*

(*) These are the versions used to develop this repository. Older versions might work as well, but they haven't been tested.

Furthermore, you'll need to install:

- `wasm-bindgen`, via:
  ```sh
  cargo install -f wasm-bindgen-cli@0.2.87
  ```
- `capnp`, via:
  ```sh
  brew install capnp
  ```
  
  or

  ```sh
  apt-get install capnproto
  ```
- `capnpc-ts`, the TypeScript bindings for Cap'n Proto, via:
  ```sh
  deno install --allow-env --allow-read --allow-write -n capnpc-ts "https://deno.land/x/capnpc/mod.ts"
  ```

You'll also need to enable to `wasm32-unknown-unknown` target for Rust, via:

```sh
rustup target add wasm32-unknown-unknown
```

### How to Run

- Compile the [Rust codebase](./rust) and generate bindings for Cap'n Proto and WebAssembly via:
  ```sh
  ./build.sh
  ```

- Run the [`./src/event.ts`](./src/event.ts) demo via:
  ```sh
  deno task event
  ```

### Additional

You can use `capnp` to convert a Cap'n Proto binary to JSON.
For instance, given the `./src/capnp/event.capnp` Cap'n Proto schema containing the `Event` struct definition, and the example binary message at `./bin/event.bin`, you can run

```sh
capnp convert binary:json ./src/capnp/event.capnp Event < ./bin/event.bin | jq
```

which should yield

```json
{
  "name": "trivago",
  "year": 2023
}
```
