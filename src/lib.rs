use js_sys::Uint8Array;
use num_bigint::BigUint;
use sha2::{Digest, Sha256};

mod bit_cycling;
mod blocks;
mod tests;

fn hash_number(number: &BigUint) -> BigUint {
    let mut hasher = Sha256::default();
    hasher.update(number.to_bytes_be());

    let output = hasher.finalize();

    return BigUint::from_bytes_be(&output);
}

fn encrypt(data: &Uint8Array, key: &BigUint) -> Uint8Array {
    return Uint8Array::new_with_length(data.length() as u32);
}

fn decrypt(data: &Uint8Array, key: &BigUint) -> Uint8Array {
    return Uint8Array::new_with_length(data.length() as u32);
}
