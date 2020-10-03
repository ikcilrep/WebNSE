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
    block: &[u8; BlockSize],
    key: &BigUint,
    encrypted_block: &mut [u8; EncryptedBlockSize],
) {
}

fn decrypt_block(
    encrypted_block: &[u8; EncryptedBlockSize],
    key: &BigUint,
    decrypted_block: &mut [u8; BlockSize],
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
        let mut block = [0; BlockSize];
        rng.fill_bytes(&mut block);

        let key = rng.gen_biguint(128);

        let mut encrypted_block = [0; EncryptedBlockSize];

        encrypt_block(&block, &key, &mut encrypted_block);

        let mut decrypted_block = [1; BlockSize];

        decrypt_block(&encrypted_block, &key, &mut decrypted_block);

        for (e1, e2) in block.iter().zip(decrypted_block.iter()) {
            assert_eq!(e1, e2);
        }
    }

    #[test]
    fn derive_key_fills_output_with_primes() {
        use num_bigint::RandBigInt;
        use rand::{thread_rng, RngCore};
        let mut rng = thread_rng();

        let key = rng.gen_biguint(128);
        let mut salt = [0; SaltSize];
        rng.fill_bytes(&mut salt);

        let mut derived_key = [0; BlockSize];
        derive_key(&key, &salt, &mut derived_key);
        for e in derived_key.iter() {
            assert_eq!(Primes.iter().any(|p| p == e), true);
        }
    }
}
