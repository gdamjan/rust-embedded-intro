# `Rust Embedded Intro`

Using the [`nrf8340-dk`](https://www.nordicsemi.com/Products/Development-hardware/nrf5340-dk)
board based on Cortex-M33 with the `thumbv8m.main-none-eabihf` ISA, with 1MB Flash and 256K RAM.

![nRF5340-DK](https://github.com/gdamjan/rust-embedded-intro/assets/81654/a3cbe0e1-5f82-40d7-b5aa-fc6759a978c5)

## Preparation

- [`rustup`](https://rustup.rs/) - is recommended to install rust and its components
- `cargo install cargo-binutils` - for `cargo size` and `cargo objdump -- --disassemble`, etc…
- `cargo install probe-rs-tools` - flash and debug using the [`probe-rs project`](https://probe.rs/)

## Examples

- `cargo embed --example rtt` - build, flash and open the rtt [debugger/monitor](https://probe.rs/docs/tools/cargo-embed/)
- `cargo embed --example embassy` - [embassy](https://embassy.dev/) demo with 2 concurent tasks, blinking 2 LEDs

## VS Code settings

The repo will also suggest common extensions for VS Code:
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [crates helper](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)

and some settings to instruct rust-analyzer to only run for the `thumbv8m.main-none-eabihf` target.

## Infrastructure files

Apart from the rust source code files, the following files are involved in the build and debug process:

- `Cargo.toml`, `Cargo.lock` - [Cargo is the Rust package manager](https://doc.rust-lang.org/cargo/index.html).
  Dependencies are specified here.
- `rust-toolchain.toml` - [rustup overrides](https://rust-lang.github.io/rustup/overrides.html).
  Make sure we use `stable` rust, and have the `thumbv8m.main-none-eabihf` target, and the `llvm-tools` component.
- `memory.x` - the [memory layout](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/#memoryx) of the nRF5340/Cortex-M33 core.
- [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) - this sets the default build target (`thumbv8m.main-none-eabihf`).
- [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html) - linker flags, track changes to memory.x.
- `Embed.toml` - [cargo-embed](https://probe.rs/docs/tools/cargo-embed/#configuration) config file. Specifies the chip,
  and enables rtt.
