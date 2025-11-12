use anyhow::{Context, Result};
use wasmtime::*;

// A Host-side struct to manage the Wasm runtime state
pub(super) struct WasmInstance {
    store: Store<()>,
    instance: Instance,
    memory: Memory,
}

impl WasmInstance {
    pub(super) fn new(store: Store<()>, instance: Instance, memory: Memory) -> Self {
        Self {
            store,
            instance,
            memory,
        }
    }

    pub(super) fn store(&mut self) -> &mut Store<()> {
        &mut self.store
    }

    /// Helper to find a typed function export from the Wasm module.
    pub(super) fn get_typed_func<Args, Ret>(&mut self, name: &str) -> Result<TypedFunc<Args, Ret>>
    where
        Args: WasmParams,
        Ret: WasmResults,
    {
        self.instance
            .get_typed_func(&mut self.store, name)
            .with_context(|| format!("Failed to find exported function '{}'", name))
    }

    /// Allocates memory inside the Wasm guest by calling the `allocate` export.
    pub(super) fn allocate(&mut self, size: usize) -> Result<i32> {
        let allocate_func = self.get_typed_func::<i32, i32>("allocate")?;
        allocate_func.call(&mut self.store, size as i32)
    }

    /// Writes a string from native memory into the Wasm Guest's isolated memory.
    pub(super) fn write_string(&mut self, s: &str) -> Result<(i32, i32)> {
        let len = s.len();
        // 1. Allocate space in the Wasm sandbox
        let ptr = self.allocate(len)?;

        // 2. Write the string's bytes to the allocated Wasm memory region
        let slice = self.memory.data_mut(&mut self.store);
        let dest = &mut slice[ptr as usize..ptr as usize + len];
        dest.copy_from_slice(s.as_bytes());

        Ok((ptr, len as i32))
    }

    /// Reads a string from the Wasm Guest memory based on the packed pointer/length return.
    pub(super) fn read_string(&self, result: u64) -> Result<String> {
        let ptr = (result >> 32) as usize;
        let len = (result & 0xFFFFFFFF) as usize;

        // 1. Access the isolated Wasm memory
        let slice = self.memory.data(&self.store);

        // 2. Copy the bytes out of the sandbox
        let data = slice[ptr..ptr + len].to_vec();

        // 3. Convert bytes to a Rust string
        String::from_utf8(data).context("Wasm module returned invalid UTF-8")
    }
}
