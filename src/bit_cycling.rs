use num_bigint::BigUint;

pub fn cycle_left(data: &[i8], bits_to_shift: &BigUint, cycled_data: &mut [u8]) {}

pub fn cycle_right(data: &[u8], bits_to_shift: &BigUint, cycled_data: &mut [i8]) {}

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
