use num_bigint::BigUint;
use js_sys::Uint8Array;

mod tests;
mod blocks;
mod bit_cycling;

fn encrypt(data: &Uint8Array, key: &BigUint) -> Uint8Array {
    return Uint8Array::new_with_length(data.length() as u32);
}

fn decrypt(data: &Uint8Array, key: &BigUint) -> Uint8Array {
    return Uint8Array::new_with_length(data.length() as u32);
}