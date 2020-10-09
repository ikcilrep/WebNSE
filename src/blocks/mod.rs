mod bytes;
mod generation;

use crate::blocks::bytes::join_bytes;
use crate::blocks::bytes::split_bytes;
use crate::blocks::generation::derive_key;
use crate::blocks::generation::generate_iv;
use num_bigint::BigUint;
use rand::rngs::OsRng;
use rand::RngCore;

const BlockSize: usize = 256;
const SaltSize: usize = 16;

const ElementSize: usize = 5;

const EncryptedBlockSize: usize = ElementSize * BlockSize + BlockSize + SaltSize;

fn encrypt_block(block: &[i8], key: &BigUint, encrypted_block: &mut [u8; EncryptedBlockSize]) {
    let salt = &mut encrypted_block[0..SaltSize];
    OsRng.fill_bytes(salt);

    let mut derived_key = [0; BlockSize];
    derive_key(key, &salt, &mut derived_key);

    let mut iv = [0; BlockSize];
    generate_iv(&derived_key, &block.to_vec(), &mut iv);
    for i in SaltSize..SaltSize + BlockSize {
        encrypted_block[i] = iv[i - SaltSize] as u8;
    }

    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..BlockSize {
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
        &mut encrypted_block[SaltSize + BlockSize..EncryptedBlockSize],
    );
}

fn decrypt_block(encrypted_block: &[u8], key: &BigUint, decrypted_block: &mut [i8; BlockSize]) {
    let salt = &encrypted_block[0..SaltSize];

    let mut derived_key = [0; BlockSize];
    derive_key(key, &salt, &mut derived_key);

    let unsigned_iv = &encrypted_block[SaltSize..SaltSize + BlockSize];
    let mut iv = [0; BlockSize];
    for i in 0..BlockSize {
        iv[i] = unsigned_iv[i] as i8;
    }

    let mut joined_encrypted_block: [i64; BlockSize] = [0; BlockSize];

    join_bytes(
        &encrypted_block[BlockSize + SaltSize..EncryptedBlockSize],
        &mut joined_encrypted_block,
    );

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut sum3 = 0;

    for i in 0..BlockSize {
        sum1 += derived_key[i] as i64 * iv[i] as i64;
        sum2 += derived_key[i] as i64 * derived_key[i] as i64;
        sum3 += derived_key[i] as i64 * joined_encrypted_block[i] as i64;
    }

    sum1 <<= 1;
    sum3 <<= 1;

    for i in 0..BlockSize {
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
    fn encryptedBlockSize_is_1552() {
        assert_eq!(EncryptedBlockSize, 1552);
    }

    #[test]
    fn encrypt_block_can_be_reversed() {
        use num_bigint::RandBigInt;
        use rand::{thread_rng, RngCore};

        let mut rng = thread_rng();
        let mut unsigned_block = [0; BlockSize];
        rng.fill_bytes(&mut unsigned_block);
        let mut block = Vec::new();

        for i in 0..BlockSize {
            block.push(unsigned_block[i] as i8);
        }

        let key = rng.gen_biguint(128);

        let mut encrypted_block = [0; EncryptedBlockSize];

        encrypt_block(&block, &key, &mut encrypted_block);

        let mut decrypted_block = [1; BlockSize];

        decrypt_block(&encrypted_block.to_vec(), &key, &mut decrypted_block);

        for (e1, e2) in block.iter().zip(decrypted_block.iter()) {
            assert_eq!(e1, e2);
        }
    }
}
