# Domris

## Requirements

- Rust
- Node.js

if needed, please execute below commands

```sh
rustup install beta 
rustup target add wasm32-unknown-unknown --toolchain beta
cargo +beta install wasm-bindgen-cli
```

## Run

```sh
npm install
npm run build:release
npm run serve
```
