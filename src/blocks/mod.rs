mod generation;

use hkdf::Hkdf;
use num_bigint::BigUint;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::Sha256;

const BlockSize: usize = 256;
const SaltSize: usize = 16;

const ElementSize: usize = 5;

const EncryptedBlockSize: usize = ElementSize * BlockSize + BlockSize + SaltSize;

fn encrypt_block(
    block: &[i8; BlockSize],
    key: &BigUint,
    encrypted_block: &mut [u8; EncryptedBlockSize],
) {
}

fn decrypt_block(
    encrypted_block: &[u8; EncryptedBlockSize],
    key: &BigUint,
    decrypted_block: &mut [i8; BlockSize],
) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::generation::derive_key;
    use crate::blocks::generation::Primes;

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
        let mut block = [0; BlockSize];

        for i in 0..BlockSize {
            block[i] = unsigned_block[i] as i8;
        }

        let key = rng.gen_biguint(128);

        let mut encrypted_block = [0; EncryptedBlockSize];

        encrypt_block(&block, &key, &mut encrypted_block);

        let mut decrypted_block = [1; BlockSize];

        decrypt_block(&encrypted_block, &key, &mut decrypted_block);

        for (e1, e2) in block.iter().zip(decrypted_block.iter()) {
            assert_eq!(e1, e2);
        }
    }
}
