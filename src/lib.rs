use crate::bit_cycling::cycle_left;
use crate::bit_cycling::cycle_right;
use crate::blocks::decrypt_block;
use crate::blocks::encrypt_block;
use crate::blocks::BLOCK_SIZE;
use crate::blocks::ENCRYPTED_BLOCK_SIZE;
use js_sys::Uint8Array;
use num_bigint::BigUint;
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

fn encrypt(data: Uint8Array, key: &BigUint) -> Uint8Array {
    let bytes_to_pad = (BLOCK_SIZE - (data.length() as usize % BLOCK_SIZE)) % BLOCK_SIZE;
    let mut padded_data = vec![0; data.length() as usize + BLOCK_SIZE + bytes_to_pad];

    for i in 0..data.length() {
        padded_data[i as usize] = data.get_index(i);
    }

    let padding_bytes = randomBytes(padded_data.len() as u32 - data.length());

    for i in 0..padding_bytes.length() {
        padded_data[i as usize + data.length() as usize] = padding_bytes.get_index(i);
    }

    let last_index = padded_data.len() - 1;
    padded_data[last_index] = bytes_to_pad as u8;
    let bits_to_shift = hash_number(key);
    let mut cycled_data = Vec::with_capacity(padded_data.len() as usize);

    cycle_right(&padded_data, &bits_to_shift, &mut cycled_data);

    let result = Uint8Array::new_with_length(
        (cycled_data.len() as u32 / BLOCK_SIZE as u32) * ENCRYPTED_BLOCK_SIZE as u32,
    );

    for i in (0..padded_data.len()).step_by(BLOCK_SIZE) {
        let encrypted_block_start = i as u32 / BLOCK_SIZE as u32 * ENCRYPTED_BLOCK_SIZE as u32;
        encrypt_block(
            &cycled_data[i..i + BLOCK_SIZE],
            key,
            &result.subarray(
                encrypted_block_start,
                encrypted_block_start + ENCRYPTED_BLOCK_SIZE as u32,
            ),
        );
    }
    return result;
}

fn decrypt(encrypted_data: Uint8Array, key: &BigUint) -> Uint8Array {
    let mut cycled_data =
        vec![0; encrypted_data.length() as usize / ENCRYPTED_BLOCK_SIZE * BLOCK_SIZE];
    for i in (0..encrypted_data.length()).step_by(ENCRYPTED_BLOCK_SIZE) {
        let decrypted_block_start = i as usize / ENCRYPTED_BLOCK_SIZE * BLOCK_SIZE;
        decrypt_block(
            &encrypted_data.subarray(i, i + ENCRYPTED_BLOCK_SIZE as u32),
            key,
            &mut cycled_data[decrypted_block_start..decrypted_block_start + BLOCK_SIZE],
        );
    }

    let bits_to_shift = hash_number(key);
    let mut padded_data = Vec::with_capacity(cycled_data.len());
    cycle_left(&cycled_data, &bits_to_shift, &mut padded_data);
    let last_index = padded_data.len() - 1;

    let data = Uint8Array::new_with_length(
        padded_data.len() as u32 - padded_data[last_index] as u32 - BLOCK_SIZE as u32,
    );

    for i in 0..data.length() {
        data.set_index(i, padded_data[i as usize]);
    }

    return data;
}
