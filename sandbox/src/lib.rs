use bip39::Mnemonic;
use std::mem;
use std::slice;

// We declare a function that MUST be provided by the Host (Wasmtime application).
// This is the *only* way the Wasm module can get random bytes.
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    // This function signature is matched by the Host application.
    fn fill_random_bytes(ptr: *mut u8, len: usize);
}

// --- EXPORTED MEMORY UTILITIES ---
// The Host calls these to manage the Guest's isolated memory.

/// Allocates memory in the Wasm linear memory.
/// Returns a pointer (i32) to the start of the new buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allocate(size: usize) -> *mut u8 {
    let mut vec = Vec::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    // Prevents Rust from deallocating the memory when `vec` goes out of scope.
    // The Host is now responsible for handling this memory.
    mem::forget(vec);
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn generate_mnemonic(ptr: *mut u8, len: usize) -> u64 {
    // SAFETY: We trust the Host passed a valid pointer/length from a
    // previously allocated block. The Wasm sandbox prevents unauthorized
    // access outside of the module's memory.
    let input_slice = unsafe { slice::from_raw_parts(ptr, len) };
    let input_string = String::from_utf8_lossy(input_slice);

    let mut entropy_buffer = [0u8; 32];

    // REQUEST RANDOMNESS FROM THE HOST
    // SAFETY: We trust the Host implements the `fill_random_bytes` function
    // correctly and securely. The pointer passed is valid Wasm memory.
    unsafe {
        fill_random_bytes(entropy_buffer.as_mut_ptr(), entropy_buffer.len());
    }

    // 3. Generate Mnemonic from the secure entropy bytes
    let mnemonic = generate(&entropy_buffer);

    let result_string = mnemonic.to_string();

    // 4. Prepare the result for the Host (using allocation logic)
    let result_bytes = result_string.into_bytes();
    let ptr = result_bytes.as_ptr() as u64;
    let len = result_bytes.len() as u64;

    // Prevent deallocation and return the packed pointer and length.
    mem::forget(result_bytes);

    // Pack pointer (u32) and length (u32) into a single u64
    (ptr << 32) | len
}

fn generate(entropy_buffer: &[u8]) -> Mnemonic {
    Mnemonic::from_entropy(entropy_buffer).expect("BIP39 generation failed unexpectedly")
}

#[cfg(test)]
mod tests {
    use crate::generate;
    use bip39::{Language, Mnemonic};
    use bitcoin_hashes::hex::FromHex;

    #[test]
    fn test_vectors_english() {
        // These vectors are tuples of
        // (entropy, mnemonic, seed)
        let test_vectors = [
            (
                "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
                "legal winner thank year wave sausage worth useful legal winner thank yellow",
                "2e8905819b8723fe2c1d161860e5ee1830318dbf49a83bd451cfb8440c28bd6fa457fe1296106559a3c80937a1c1069be3a3a5bd381ee6260e8d9739fce1f607",
            ),
            (
                "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
                "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal will",
                "f2b94508732bcbacbcc020faefecfc89feafa6649a5491b8c952cede496c214a0c7b3c392d168748f2d4a612bada0753b52a1c7ac53c1e93abd5c6320b9e95dd",
            ),
            (
                "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
                "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth title",
                "bc09fca1804f7e69da93c2f2028eb238c227f2e9dda30cd63699232578480a4021b146ad717fbb7e451ce9eb835f43620bf5c514db0f8add49f5d121449d3e87",
            ),
        ];

        for vector in &test_vectors {
            let entropy = Vec::<u8>::from_hex(&vector.0).unwrap();
            let mnemonic_str = vector.1;
            let seed = Vec::<u8>::from_hex(&vector.2).unwrap();

            let mnemonic = generate(&entropy);

            assert_eq!(
                mnemonic,
                Mnemonic::parse_in_normalized(Language::English, mnemonic_str).unwrap(),
                "failed vector: {}",
                mnemonic_str
            );
            assert_eq!(
                mnemonic,
                Mnemonic::parse_normalized(mnemonic_str).unwrap(),
                "failed vector: {}",
                mnemonic_str
            );
            assert_eq!(
                &seed[..],
                &mnemonic.to_seed_normalized("TREZOR")[..],
                "failed vector: {}",
                mnemonic_str
            );
        }
    }
}
