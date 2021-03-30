# Domris

[![Deploy GitHub Pages](https://github.com/eduidl/domris/actions/workflows/gh-pages.yml/badge.svg)](https://github.com/eduidl/domris/actions/workflows/gh-pages.yml)
[![License](https://img.shields.io/github/license/eduidl/domris)](https://github.com/eduidl/domris/blob/main/LICENSE)

You can play [here](https://eduidl.github.io/domris/) (keyboard is needed).

## Requirements

- Rust
- Node.js

Execute below commands if needed.

```terminal
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-pack
```

## Run

```terminal
$ npm i
$ npm run build:release
$ npm run start
```
