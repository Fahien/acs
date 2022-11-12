<div align="center">

# Acs 🖥️

**Acs is an experimental project for learning computer science by building a general-purpose computer system from the ground up.**

</div>

## Build

Acs should work on Linux, MacOS, and Windows without issues, but if you need any help please do not hesitate to contact me.

1. Install [Rust](https://rustup.rs/).
2. Compile with `cargo build`.
3. Run tests with `cargo test`.

### Windows

#### Acsim

On Windows you will need to install `SDL2.dll` and `SDL2.lib`:

1. Download these files from an [SDL2-devel VC release](https://github.com/libsdl-org/SDL/releases/).
2. Put them in the correct rustup lib path.
   ```sh
   .rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/x86_64-pc-windows-msvc/lib
   ```

#### Wasm

If you want to use wasm-pack, install it following [these instructions](https://rustwasm.github.io/wasm-pack/installer/).
Then run `wasm-pack build` to generate a wasm pkg in the `pkg` folder.

### Cal Language Server

You can build the acs library for the WebAssembly target enabling the `wasm` feature.

1. Make sure you have installed a wasm32 rust toolchain
   ```sh
   rustup target add wasm32-unknown-unknown
   ```
2. Build for this target enabling the `wasm` feature
