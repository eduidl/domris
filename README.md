# Tetris (仮)

## Requirements

- Rust
- Node.js

if needed, please execute below commands

```sh
rustup install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo +nightly install wasm-bindgen-cli
```

## Run

```sh
npm install
npm run build:release
npm run serve
```
