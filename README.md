
# Term Rewriting

This repository holds the Rust source code that powers the
WASM demos that are a part of this [blog post](https://irreducible.io/experiments/2023/12/term-rewriting)

## Building

Build the native executable
```shell
cargo build --release
```

Build the WASM package
```shell
wasm-pack build --target web
```
