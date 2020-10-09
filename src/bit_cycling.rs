use num_bigint::BigUint;
use num_traits::ToPrimitive;

pub fn cycle_left(data: &[i8], bits_to_shift: &BigUint, cycled_data: &mut [u8]) {
    let l1 = bits_to_shift % (8u64 * data.len() as u64);
    let l2 = (&l1 % 8usize).to_usize().unwrap();
    let l3 = (l1 / 8usize).to_usize().unwrap();
    if l2 == 0 {
        for k in 0..data.len() - l3 - 1 {
            cycled_data[k] = data[k + l3] as u8;
        }
        cycled_data[data.len() - 1 - l3] = data[data.len() - 1] as u8;
        for k in data.len() - l3..data.len() {
            cycled_data[k] = data[k + l3 - data.len()] as u8;
        }
    } else {
        for k in 0..data.len() - l3 - 1 {
            cycled_data[k] = exchange_bits_left(data[k + l3], data[k + l3 + 1], l2);
        }
        cycled_data[data.len() - 1 - l3] = exchange_bits_left(data[data.len() - 1], data[0], l2);
        for k in data.len() - l3..data.len() {
            cycled_data[k] =
                exchange_bits_left(data[k + l3 - data.len()], data[k + l3 - data.len() + 1], l2);
        }
    }
}

#[inline]
fn exchange_bits_left(a: i8, b: i8, bits_to_exchange: usize) -> u8 {
    ((a as u8) << bits_to_exchange) | ((b as u8) >> (8 - bits_to_exchange))
}

#[inline]
fn exchange_bits_right(a: u8, b: u8, bits_to_exchange: usize) -> i8 {
    ((a << (8 - bits_to_exchange)) | (b >> bits_to_exchange)) as i8
}

pub fn cycle_right(data: &[u8], bits_to_shift: &BigUint, cycled_data: &mut [i8]) {
    let l1 = bits_to_shift % (8u64 * data.len() as u64);
    let l2 = (&l1 % 8usize).to_usize().unwrap();
    let l3 = (l1 / 8usize).to_usize().unwrap();

    if l2 == 0 {
        for k in 0..l3 {
            cycled_data[k] = data[data.len() + k - l3] as i8;
        }

        cycled_data[l3] = data[0] as i8;

        for k in l3 + 1..data.len() {
            cycled_data[k] = data[k - l3] as i8;
        }
    } else {
        for k in 0..l3 {
            cycled_data[k] =
                exchange_bits_right(data[data.len() + k - l3 - 1], data[data.len() + k - l3], l2);
        }

        cycled_data[l3] = exchange_bits_right(data[data.len() - 1], data[0], l2);

        for k in l3 + 1..data.len() {
            cycled_data[k] = exchange_bits_right(data[k - l3 - 1], data[k - l3], l2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::RandBigInt;
    use rand::thread_rng;
    use rand::RngCore;

    #[test]
    #[should_panic]
    fn cycle_left_panics_if_input_is_bigger_than_output() {
        let mut rng = thread_rng();

        let bits_to_shift = rng.gen_biguint(256);
        let mut unsigned_data = [0; 1441];
        rng.fill_bytes(&mut unsigned_data);

        let mut data = [0; 1441];
        for i in 0..1441 {
            data[i] = unsigned_data[i] as i8;
        }

        let mut cycled_left_data = [0; 1440];
        cycle_left(&data, &bits_to_shift, &mut cycled_left_data);
    }

    #[test]
    #[should_panic]

    fn cycle_right_panics_if_input_is_bigger_than_output() {
        let mut rng = thread_rng();

        let bits_to_shift = rng.gen_biguint(256);
        let mut data = [0; 1441];
        rng.fill_bytes(&mut data);

        let mut cycled_right_data = [0; 1440];
        cycle_right(&data, &bits_to_shift, &mut cycled_right_data);
    }

    #[test]
    fn cycle_left_can_be_reversed_with_cycle_right() {
        let mut rng = thread_rng();

        let bits_to_shift = rng.gen_biguint(256);
        let mut unsigned_data = [0; 1441];
        rng.fill_bytes(&mut unsigned_data);

        let mut data = [0; 1441];
        for i in 0..1441 {
            data[i] = unsigned_data[i] as i8;
        }

        let mut cycled_left_data = [0; 1441];
        cycle_left(&data, &bits_to_shift, &mut cycled_left_data);

        let mut uncycled_data = [0; 1441];
        cycle_right(&cycled_left_data, &bits_to_shift, &mut uncycled_data);

        for (d, u) in data.iter().zip(uncycled_data.iter()) {
            assert_eq!(d, u);
        }
    }

    #[test]
    fn cycle_right_can_be_reversed_with_cycle_left() {
        let mut rng = thread_rng();

        let bits_to_shift = rng.gen_biguint(256);
        let mut data = [0; 1441];
        rng.fill_bytes(&mut data);

        let mut cycled_right_data = [0; 1441];
        cycle_right(&data, &bits_to_shift, &mut cycled_right_data);

        let mut uncycled_data = [0; 1441];
        cycle_left(&cycled_right_data, &bits_to_shift, &mut uncycled_data);

        for (d, u) in data.iter().zip(uncycled_data.iter()) {
            assert_eq!(d, u);
        }
    }
}
