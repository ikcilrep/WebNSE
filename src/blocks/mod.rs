mod bytes;
mod generation;

use crate::blocks::bytes::join_bytes;
use crate::blocks::bytes::split_bytes;
use crate::blocks::generation::derive_key;
use crate::blocks::generation::generate_iv;
use js_sys::Uint8Array;
use num_bigint::BigUint;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::*;

pub const BLOCK_SIZE: usize = 256;
const SALT_SIZE: usize = 16;

const ELEMENT_SIZE: usize = 5;

pub const ENCRYPTED_BLOCK_SIZE: usize = ELEMENT_SIZE * BLOCK_SIZE + BLOCK_SIZE + SALT_SIZE;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    fn randomBytes(size: u32) -> Uint8Array;
}

pub fn encrypt_block(block: &[i8], key: &BigUint, encrypted_block: &Uint8Array) {
    let salt = randomBytes(SALT_SIZE as u32);
    for i in 0..salt.length() {
        encrypted_block.set_index(i, salt.get_index(i));
    }

    let mut derived_key = [0; BLOCK_SIZE];
    derive_key(key, &salt.to_vec(), &mut derived_key);

    let mut iv = [0; BLOCK_SIZE];
    generate_iv(&derived_key, &block.to_vec(), &mut iv);
    for i in SALT_SIZE..SALT_SIZE + BLOCK_SIZE {
        encrypted_block.set_index(i as u32, iv[i - SALT_SIZE] as u8);
    }

    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..BLOCK_SIZE {
        sum1 += derived_key[i] as i64 * derived_key[i] as i64;
        sum2 += derived_key[i] as i64 * (block[i] as i64 - iv[i] as i64);
    }
    sum2 <<= 1;

    let mut encrypted_block_iter = block
        .iter()
        .zip(derived_key.iter())
        .map(|(&r, &p)| r as i64 * sum1 - (p as i64 * sum2));
    split_bytes(
        &mut encrypted_block_iter,
        &encrypted_block.subarray(
            SALT_SIZE as u32 + BLOCK_SIZE as u32,
            ENCRYPTED_BLOCK_SIZE as u32,
        ),
    );
}

fn decrypt_block(encrypted_block: &[u8], key: &BigUint, decrypted_block: &mut [i8; BLOCK_SIZE]) {
    let salt = &encrypted_block[0..SALT_SIZE];

    let mut derived_key = [0; BLOCK_SIZE];
    derive_key(key, &salt, &mut derived_key);

    let unsigned_iv = &encrypted_block[SALT_SIZE..SALT_SIZE + BLOCK_SIZE];
    let mut iv = [0; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        iv[i] = unsigned_iv[i] as i8;
    }

    let mut joined_encrypted_block: [i64; BLOCK_SIZE] = [0; BLOCK_SIZE];

    join_bytes(
        &encrypted_block[BLOCK_SIZE + SALT_SIZE..ENCRYPTED_BLOCK_SIZE],
        &mut joined_encrypted_block,
    );

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut sum3 = 0;

    for i in 0..BLOCK_SIZE {
        sum1 += derived_key[i] as i64 * iv[i] as i64;
        sum2 += derived_key[i] as i64 * derived_key[i] as i64;
        sum3 += derived_key[i] as i64 * joined_encrypted_block[i] as i64;
    }

    sum1 <<= 1;
    sum3 <<= 1;

    for i in 0..BLOCK_SIZE {
        let a = joined_encrypted_block[i] + derived_key[i] as i64 * sum1;
        let b = (derived_key[i] as i64 * (sum3 / sum2))
            + ((derived_key[i] as i64 * (sum3 % sum2)) / sum2);
        let c = (a - b) / sum2;
        decrypted_block[i] = c as i8;
    }
}

#[wasm_bindgen_test]
pub fn encrypt_block_can_be_reversed() {
    use std::str::FromStr;

    let unsigned_block: [u8; BLOCK_SIZE] = [
        237, 252, 84, 64, 120, 86, 39, 29, 40, 209, 77, 44, 108, 122, 150, 132, 46, 92, 98, 25,
        173, 186, 243, 142, 77, 145, 76, 71, 245, 118, 52, 172, 221, 109, 180, 222, 235, 18, 182,
        237, 67, 240, 184, 164, 150, 90, 193, 97, 89, 74, 204, 205, 185, 255, 80, 49, 97, 172, 213,
        235, 96, 46, 24, 104, 68, 32, 179, 110, 229, 157, 134, 6, 94, 199, 82, 118, 185, 155, 160,
        157, 240, 102, 63, 131, 3, 195, 152, 146, 202, 243, 217, 208, 3, 93, 180, 20, 164, 129,
        112, 207, 162, 16, 69, 220, 173, 220, 211, 162, 84, 14, 167, 182, 91, 110, 178, 214, 31,
        152, 103, 133, 191, 213, 244, 226, 49, 21, 15, 36, 21, 122, 54, 114, 121, 210, 134, 219, 7,
        220, 110, 12, 111, 66, 28, 104, 217, 18, 120, 177, 188, 145, 244, 194, 16, 187, 34, 147,
        164, 94, 247, 204, 192, 54, 143, 155, 66, 191, 225, 159, 88, 20, 25, 12, 72, 178, 212, 178,
        61, 85, 108, 223, 38, 187, 44, 187, 197, 138, 143, 180, 248, 48, 132, 157, 193, 104, 196,
        186, 26, 198, 214, 182, 119, 65, 187, 161, 108, 234, 95, 112, 36, 19, 42, 194, 95, 18, 45,
        154, 203, 218, 118, 20, 185, 197, 197, 85, 42, 147, 251, 18, 197, 192, 107, 156, 191, 115,
        194, 207, 162, 16, 69, 220, 173, 220, 211, 162, 84, 14, 167, 182, 91, 110, 178, 214, 31,
        152, 103, 133,
    ];
    let mut block = Vec::new();

    for i in 0..BLOCK_SIZE {
        block.push(unsigned_block[i] as i8);
    }

    let key = BigUint::from_str("110192826829776194000614388426091705128").unwrap();

    let mut encrypted_block = Uint8Array::new_with_length(ENCRYPTED_BLOCK_SIZE as u32);

    encrypt_block(&block, &key, &mut encrypted_block);

    let mut decrypted_block = [1; BLOCK_SIZE];

    decrypt_block(&encrypted_block.to_vec(), &key, &mut decrypted_block);

    for (e1, e2) in block.iter().zip(decrypted_block.iter()) {
        assert_eq!(e1, e2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypted_block_size_is_1552() {
        assert_eq!(ENCRYPTED_BLOCK_SIZE, 1552);
    }
}
