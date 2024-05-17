# `Rust Embedded Intro`

Using the [`nrf8340-dk`](https://www.nordicsemi.com/Products/Development-hardware/nrf5340-dk)
board based on Cortex-M33 with the `thumbv8m.main-none-eabihf` ISA, with 1MB Flash and 256K RAM.

## Preparation

- [`rustup`](https://rustup.rs/) - is recommended to install rust and its components
- `cargo install cargo-binutils` - for `cargo size` and `cargo objdump -- --disassemble`, etc…
- `cargo install cargo-embed` - flash and debug using the [`probe-rs project`](https://probe.rs/)

## Examples

- `cargo embed --example rtt` - build, flash and open the rtt [debugger/monitor](https://probe.rs/docs/tools/cargo-embed/)
- `cargo embed --example embassy` - [embassy](https://embassy.dev/) demo with 2 concurent tasks, blinking 2 LEDs

## VS Code settings

The repo will also suggest common extensions for VS Code:
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [crates helper](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)

and some settings to instruct rust-analyzer to only run for the `thumbv8m.main-none-eabihf` target.
