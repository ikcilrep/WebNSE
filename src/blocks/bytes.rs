use crate::blocks::BlockSize;
use crate::blocks::ElementSize;

pub fn join_bytes(bytes: &[u8; ElementSize * BlockSize], data: &mut [i64; BlockSize]) {
    for i in (0..ElementSize * BlockSize).step_by(ElementSize) {
        let mut element = 0;
        for j in (i..i + ElementSize).rev() {
            element <<= 8;
            element += bytes[j] as u64;
        }

        let index = i / ElementSize;
        data[index] = element as i64;
    }
}

pub fn split_bytes<I>(data: &mut I, bytes: &mut [u8])
where
    I: Iterator<Item = i64>,
{
    for i in (0..bytes.iter().count()).step_by(ElementSize) {
        let mut e = data.next().unwrap() as u64;
        for j in i..i + ElementSize {
            bytes[j] = (e & 255) as u8;
            e >>= 8;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_bytes_can_be_reversed() {
        let data: [i64; BlockSize] = [274877906943; BlockSize];
        let mut bytes = [0; BlockSize * ElementSize];
        split_bytes(&mut data.iter().map(|&x| x as i64), &mut bytes);
        let mut joined_bytes = [0; BlockSize];
        join_bytes(&bytes, &mut joined_bytes);
    }
}
