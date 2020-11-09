mod vectors;

use crate::blocks::generation::vectors::are_orthogonal;
use crate::blocks::generation::vectors::vector_difference;
use crate::blocks::BLOCK_SIZE;
use hkdf::Hkdf;
use js_sys::Uint8Array;
use num_bigint::BigUint;
use sha2::Sha256;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "crypto"])]
    fn getRandomValues(typedArray: Uint8Array) -> Uint8Array;
}

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
    let mut unsigned_iv;
    let mut difference = [0; BLOCK_SIZE];
    while {
        unsigned_iv = getRandomValues(Uint8Array::new_with_length(BLOCK_SIZE as u32));

        for i in 0..BLOCK_SIZE {
            iv[i] = unsigned_iv.get_index(i as u32) as i8;
        }

        vector_difference(block, iv, &mut difference);

        are_orthogonal(derived_key, &difference)
    } {}
}

#[wasm_bindgen_test]
pub fn generate_iv_derived_key_is_not_orthogonal_with_block_and_key_difference() {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    let derived_key = [1; BLOCK_SIZE];

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

    let mut iv = [0; BLOCK_SIZE];
    generate_iv(&derived_key, &block.to_vec(), &mut iv);

    let mut difference = [0; BLOCK_SIZE];
    vector_difference(&block, &iv, &mut difference);

    assert!(!are_orthogonal(&derived_key, &difference));
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
