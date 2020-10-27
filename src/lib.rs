use crate::bit_cycling::cycle_right;
use crate::blocks::encrypt_block;
use crate::blocks::BLOCK_SIZE;
use crate::blocks::ENCRYPTED_BLOCK_SIZE;
use js_sys::Uint8Array;
use num_bigint::BigUint;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::wasm_bindgen;

mod bit_cycling;
mod blocks;
mod tests;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    fn randomBytes(size: u32) -> Uint8Array;
}

fn hash_number(number: &BigUint) -> BigUint {
    let mut hasher = Sha256::default();
    hasher.update(number.to_bytes_be());

    let output = hasher.finalize();

    return BigUint::from_bytes_be(&output);
}

fn encrypt(data: &Uint8Array, key: &BigUint) -> Uint8Array {
    let mut padded_data =
        vec![0; data.length() as usize + 2 * BLOCK_SIZE - data.length() as usize % BLOCK_SIZE];

    for i in 0..data.length() {
        padded_data[i as usize] = data.get_index(i);
    }

    let padding_bytes = randomBytes(padded_data.len() as u32 - data.length());

    for i in 0..padding_bytes.length() {
        padded_data[i as usize + data.length() as usize] = padding_bytes.get_index(i);
    }

    let bits_to_shift = hash_number(key);
    let mut cycled_data = vec![0; data.length() as usize];

    cycle_right(data, &bits_to_shift, &mut cycled_data);

    let result = Uint8Array::new_with_length(ENCRYPTED_BLOCK_SIZE as u32);

    for i in (0..padded_data.len()).step_by(BLOCK_SIZE) {
        encrypt_block(
            &cycled_data[i..i + BLOCK_SIZE],
            key,
            &result.subarray(i as u32, i as u32 + BLOCK_SIZE as u32),
        );
    }
    return result;
}

fn decrypt(data: &Uint8Array, key: &BigUint) -> Uint8Array {
    return Uint8Array::new_with_length(data.length() as u32);
}
