use super::*;
use crate::blocks::BLOCK_SIZE;
use crate::blocks::ELEMENT_SIZE;
use js_sys::Uint8Array;

fn u40_as_i40(u40: u64) -> i64 {
    let half = 1 << (8 * ELEMENT_SIZE - 1);
    if u40 < half {
        u40 as i64
    } else {
        u40 as i64 - 2 * half as i64
    }
}

fn i40_as_u40(i40: i64) -> u64 {
    let half = 1 << (8 * ELEMENT_SIZE - 1);
    if i40 < 0 {
        (i40 + 2 * half) as u64
    } else {
        i40 as u64
    }
}

pub fn join_bytes(bytes: &[u8], data: &mut [i64]) {
    for i in (0..bytes.iter().count()).step_by(ELEMENT_SIZE) {
        let mut element = 0;
        for j in (i..i + ELEMENT_SIZE).rev() {
            element <<= 8;
            element += bytes[j] as u64;
        }

        let index = i / ELEMENT_SIZE;
        data[index] = u40_as_i40(element);
    }
}

pub fn split_bytes<I>(data: &mut I, bytes: &Uint8Array)
where
    I: Iterator<Item = i64>,
{
    for i in (0..bytes.length()).step_by(ELEMENT_SIZE) {
        let mut e = i40_as_u40(data.next().unwrap());

        for j in i..i + ELEMENT_SIZE as u32 {
            bytes.set_index(j, (e & 255) as u8);
            e >>= 8;
        }
        if e > 1 {
            panic!("overflow");
        }
    }
}

#[wasm_bindgen_test]
pub fn join_bytes_can_be_reversed() {
    let data: [i64; BLOCK_SIZE] = [274877906943; BLOCK_SIZE];
    let mut bytes = Uint8Array::new_with_length(BLOCK_SIZE as u32 * ELEMENT_SIZE as u32);
    split_bytes(&mut data.iter().map(|&x| x as i64), &mut bytes);
    let mut joined_bytes = [0; BLOCK_SIZE];
    join_bytes(&bytes.to_vec(), &mut joined_bytes);
}

#[wasm_bindgen_test]
pub fn u40_as_i40_can_be_reversed() {
    let u40 = 560608505551;
    let i40 = u40_as_i40(u40);

    assert_eq!(u40, i40_as_u40(i40));
}
