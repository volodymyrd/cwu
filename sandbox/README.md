1.

```cargo install wasm-bindgen-cli```

The wasm-bindgen-cli is a command-line tool that performs post-processing on your Wasm binary. When you compile your
Rust code to the wasm32-unknown-unknown target, it produces a standard .wasm file.
The wasm-bindgen-cli then takes that .wasm file and does two key things:

- Generates Glue Code: It creates the necessary Rust (or JavaScript) code that manages the low-level details of passing
  complex types like strings (String) and structs across the memory boundary between the Host and the Wasm Guest.
- Optimizes the Wasm: It can also perform some final optimizations specific to the binding process.

2. Add the Wasm target

```rustup target add wasm32-unknown-unknown```

3. Build the Wasm module

```
cd sandbox
cargo build --target wasm32-unknown-unknown --release
```

4. Generate the WASM binary and required glue code

```
wasm-bindgen ../target/wasm32-unknown-unknown/release/cwu_sandbox.wasm --out-dir pkg --target web
```

The resulting WASM binary is 'pkg/cwu_sandbox_bg.wasm'
