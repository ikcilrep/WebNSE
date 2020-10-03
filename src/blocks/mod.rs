use num_bigint::{BigUint};

mod encryptedBlock;

const BlockSize: usize = 256;
const SaltSize: usize = 16;

const ElementSize: usize = 5;

const EncryptedBlockSize: usize = ElementSize * BlockSize + BlockSize + SaltSize;

fn encrypt_block(block: &[u8; BlockSize], key: &BigUint, encrypted_block: &mut [u8; EncryptedBlockSize]) {

}

fn decrypt_block(encrypted_block: &[u8; EncryptedBlockSize], key: &BigUint, decrypted_block: &mut [u8; BlockSize]) {

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
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let mut block = [0; BlockSize];
        for i in 0..BlockSize {
            block[i] = rng.gen::<u8>();
        }

        let key = thread_rng().gen_biguint(128);

        let mut encrypted_block = [0; EncryptedBlockSize];

        encrypt_block(&block, &key, &mut encrypted_block);

        let mut decrypted_block = [1; BlockSize];

        decrypt_block(&encrypted_block, &key, &mut decrypted_block);

        for (e1, e2) in block.iter().zip(decrypted_block.iter()) {
            assert_eq!(e1, e2);
        }
    }
}
