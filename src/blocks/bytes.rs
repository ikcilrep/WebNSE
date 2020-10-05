use crate::blocks::BlockSize;
use crate::blocks::ElementSize;

fn u40_as_i40(u40: u64) -> i64 {
    let half = 1 << (8 * ElementSize - 1);
    if u40 < half {
        u40 as i64
    } else {
        u40 as i64 - 2 * half as i64
    }
}

fn i40_as_u40(i40: i64) -> u64 {
    let half = 1 << (8 * ElementSize - 1);
    if i40 < 0 {
        (i40 + 2 * half) as u64
    } else {
        i40 as u64
    }
}

pub fn join_bytes(bytes: &[u8], data: &mut [i64]) {
    for i in (0..bytes.iter().count()).step_by(ElementSize) {
        let mut element = 0;
        for j in (i..i + ElementSize).rev() {
            element <<= 8;
            element += bytes[j] as u64;
        }

        let index = i / ElementSize;
        data[index] = u40_as_i40(element);
    }
}

pub fn split_bytes<I>(data: &mut I, bytes: &mut [u8])
where
    I: Iterator<Item = i64>,
{
    for i in (0..bytes.iter().count()).step_by(ElementSize) {
        let mut e = i40_as_u40(data.next().unwrap());

        for j in i..i + ElementSize {
            bytes[j] = (e & 255) as u8;
            e >>= 8;
        }
        if e > 1 {
            panic!("overflow");
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

    #[test]
    fn u40_as_i40_can_be_reversed() {
        use rand::thread_rng;
        use rand::Rng;

        let mut rng = thread_rng();

        let u40 = rng.gen_range(0, 1 << 40);
        let i40 = u40_as_i40(u40);

        assert_eq!(u40, i40_as_u40(i40));
    }
}
