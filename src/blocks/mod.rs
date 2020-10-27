mod bytes;
mod generation;

use crate::blocks::bytes::join_bytes;
use crate::blocks::bytes::split_bytes;
use crate::blocks::generation::derive_key;
use crate::blocks::generation::generate_iv;
use num_bigint::BigUint;
use rand::rngs::OsRng;
use rand::RngCore;

pub const BLOCK_SIZE: usize = 256;
const SALT_SIZE: usize = 16;

const ELEMENT_SIZE: usize = 5;

pub const ENCRYPTED_BLOCK_SIZE: usize = ELEMENT_SIZE * BLOCK_SIZE + BLOCK_SIZE + SALT_SIZE;

pub fn encrypt_block(block: &[i8], key: &BigUint, encrypted_block: &mut [u8; ENCRYPTED_BLOCK_SIZE]) {
    let salt = &mut encrypted_block[0..SALT_SIZE];
    OsRng.fill_bytes(salt);

    let mut derived_key = [0; BLOCK_SIZE];
    derive_key(key, &salt, &mut derived_key);

    let mut iv = [0; BLOCK_SIZE];
    generate_iv(&derived_key, &block.to_vec(), &mut iv);
    for i in SALT_SIZE..SALT_SIZE + BLOCK_SIZE {
        encrypted_block[i] = iv[i - SALT_SIZE] as u8;
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
        &mut encrypted_block[SALT_SIZE + BLOCK_SIZE..ENCRYPTED_BLOCK_SIZE],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypted_block_size_is_1552() {
        assert_eq!(ENCRYPTED_BLOCK_SIZE, 1552);
    }

    #[test]
    fn encrypt_block_can_be_reversed() {
        use num_bigint::RandBigInt;
        use rand::{thread_rng, RngCore};

        let mut rng = thread_rng();
        let mut unsigned_block = [0; BLOCK_SIZE];
        rng.fill_bytes(&mut unsigned_block);
        let mut block = Vec::new();

        for i in 0..BLOCK_SIZE {
            block.push(unsigned_block[i] as i8);
        }

        let key = rng.gen_biguint(128);

        let mut encrypted_block = [0; ENCRYPTED_BLOCK_SIZE];

        encrypt_block(&block, &key, &mut encrypted_block);

        let mut decrypted_block = [1; BLOCK_SIZE];

        decrypt_block(&encrypted_block.to_vec(), &key, &mut decrypted_block);

        for (e1, e2) in block.iter().zip(decrypted_block.iter()) {
            assert_eq!(e1, e2);
        }
    }
}
