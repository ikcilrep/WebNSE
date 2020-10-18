mod vectors;

use crate::blocks::generation::vectors::are_orthogonal;
use crate::blocks::generation::vectors::vector_difference;
use crate::blocks::BLOCK_SIZE;
use hkdf::Hkdf;
use num_bigint::BigUint;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::Sha256;

pub const PRIMES: [u16; 256] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039,
    1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153,
    1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279,
    1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409,
    1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499,
    1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613,
    1619,
];

pub fn derive_key(key: &BigUint, salt: &[u8], derived_key: &mut [u16; BLOCK_SIZE]) {
    let hkdf = Hkdf::<Sha256>::new(Some(salt), &key.to_bytes_be());
    let mut okm = [0; BLOCK_SIZE];
    hkdf.expand(&[], &mut okm).unwrap();
    for i in 0..BLOCK_SIZE {
        derived_key[i] = PRIMES[okm[i] as usize];
    }
}

pub fn generate_iv(derived_key: &[u16; BLOCK_SIZE], block: &[i8], iv: &mut [i8; BLOCK_SIZE]) {
    let mut unsigned_iv = [0; BLOCK_SIZE];
    let mut difference = [0; BLOCK_SIZE];
    while {
        OsRng.fill_bytes(&mut unsigned_iv);

        for i in 0..BLOCK_SIZE {
            iv[i] = unsigned_iv[i] as i8;
        }

        vector_difference(block, iv, &mut difference);

        are_orthogonal(derived_key, &difference)
    } {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::generation::vectors::are_orthogonal;
    use crate::blocks::generation::vectors::vector_difference;
    use crate::blocks::SALT_SIZE;

    #[test]
    fn derive_key_fills_output_with_primes() {
        use num_bigint::RandBigInt;
        use rand::{thread_rng, RngCore};
        let mut rng = thread_rng();

        let key = rng.gen_biguint(128);
        let mut salt = [0; SALT_SIZE];
        rng.fill_bytes(&mut salt);

        let mut derived_key = [0; BLOCK_SIZE];
        derive_key(&key, &salt, &mut derived_key);
        for e in derived_key.iter() {
            assert_eq!(PRIMES.iter().any(|p| p == e), true);
        }
    }

    #[test]
    fn generate_iv_derived_key_is_not_orthogonal_with_block_and_key_difference() {
        use rand::{thread_rng, RngCore};
        let mut rng = thread_rng();

        let derived_key = [1; BLOCK_SIZE];

        let mut unsigned_block = [0; BLOCK_SIZE];
        rng.fill_bytes(&mut unsigned_block);
        let mut block = Vec::new();
        for i in 0..BLOCK_SIZE {
            block.push(unsigned_block[i] as i8);
        }

        let mut iv = [0; BLOCK_SIZE];
        generate_iv(&derived_key, &block.to_vec(), &mut iv);

        let mut difference = [0; BLOCK_SIZE];
        vector_difference(&block, &iv, &mut difference);

        assert!(!are_orthogonal(&derived_key, &difference));
    }
}
