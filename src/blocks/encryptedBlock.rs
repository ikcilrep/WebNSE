use crate::blocks::BlockSize;
use crate::blocks::SaltSize;

pub struct EncryptedBlock<'lifetime> {
    pub encrypted_data: &'lifetime [i64; BlockSize],
    pub IV: &'lifetime [i8; BlockSize],
    pub Salt: &'lifetime [u8; SaltSize]
}