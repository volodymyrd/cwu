use crate::wallet::Wallet;
use crate::wasm::{Result, instance::WasmInstance};
use anyhow::Context;
use rand::RngCore;
use std::fs;
use wasmtime::{Caller, Engine, Linker, Module, Store};

pub struct Host {
    instance: WasmInstance,
}

const SANDBOX: &str = "sandbox/target/wasm32-unknown-unknown/release/cwu_sandbox.wasm";

impl Host {
    pub(crate) fn set_up() -> Result<Self> {
        let engine = Engine::default();
        let module_bytes = fs::read(SANDBOX).context("Failed to read WASM file")?;
        let module = Module::from_binary(&engine, &module_bytes)?;

        // The Store is the container for all isolated Wasm state
        let mut store = Store::new(&engine, ());
        let mut linker = Linker::new(&engine);

        // Register the Host function `fill_random_bytes_host` with the Wasm module
        linker.func_wrap("env", "fill_random_bytes", fill_random_bytes_host)?;

        // Instantiate the module, creating the isolated sandbox process
        let instance = linker
            .instantiate(&mut store, &module)
            .context("Failed to instantiate Wasm module")?;

        // Get the Wasm memory object for manual reading/writing of data
        let memory = instance
            .get_memory(&mut store, "memory")
            .context("Wasm module must export 'memory'")?;

        let instance = WasmInstance::new(store, instance, memory);

        Ok(Self { instance })
    }

    pub(crate) fn create_wallet(&mut self, word_count: i32, language: &str) -> Result<Wallet> {
        let (input_ptr, input_len) = self
            .instance
            .write_string(language)
            .context("Failed to write string to Wasm memory")?;

        // 2. Call the sandboxed function
        let generate_mnemonic_func = self
            .instance
            .get_typed_func::<(i32, i32, i32), u64>("generate_mnemonic")?;

        let result_packed_u64 = generate_mnemonic_func
            .call(self.instance.store(), (word_count, input_ptr, input_len))
            .context("Failed to call Wasm 'generate_mnemonic' function")?;

        // 3. Process result: read the output string from the Guest's memory
        let output_string = self
            .instance
            .read_string(result_packed_u64)
            .context("Failed to read result string from Wasm memory")?;

        Ok(Wallet::new(output_string))
    }
}

// This function is implemented by the Host (native OS) but callable by the Wasm Guest (Sandbox).
fn fill_random_bytes_host(mut caller: Caller<'_, ()>, ptr: i32, len: i32) -> anyhow::Result<()> {
    // 1. Get mutable access to the Wasm module's memory
    let memory = caller
        .get_export("memory")
        .context("Wasm module must export 'memory'")?
        .into_memory()
        .context("Export 'memory' is not a memory object")?;

    // 2. Get a mutable slice corresponding to the Wasm memory region
    let dest_slice = memory
        .data_mut(&mut caller)
        .get_mut(ptr as usize..ptr as usize + len as usize)
        .ok_or_else(|| anyhow::anyhow!("Memory access out of bounds for fill_random_bytes"))?;

    // 3. Use the Host's native, Cryptographically Secure RNG (CSPRNG)
    let mut rng = rand::rng();
    rng.fill_bytes(dest_slice);

    Ok(())
}
