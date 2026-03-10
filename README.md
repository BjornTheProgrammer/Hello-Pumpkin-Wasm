# Hello, Pumpkin!
This repository is home to the Example Plugin from the [Pumpkin Documentation](https://pumpkinmc.org/plugin-dev/wasm-plugin-template/introduction).

## Steps to Build

1. Install [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
2. Install the `wasm32-wasip2` target by running `rustup target add wasm32-wasip2` in a terminal
3. Run `cargo build` inside of the root directory of this repository.

## Running on a Pumpkin Server

You can run this code on a Pumpkin Server by simply copying the resultant
`./target/wasm32-wasip2/debug/hello_pumpkin_wasm.wasm` file into your server's `plugin` directory.
