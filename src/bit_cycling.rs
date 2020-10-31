use num_bigint::BigUint;
use num_traits::ToPrimitive;
use wasm_bindgen_test::*;

pub fn cycle_left(data: &[i8], bits_to_shift: &BigUint, cycled_data: &mut Vec<u8>) {
    let l1 = bits_to_shift % (8u64 * data.len() as u64);
    let l2 = (&l1 % 8usize).to_usize().unwrap();
    let l3 = (l1 / 8usize).to_usize().unwrap();
    if l2 == 0 {
        for k in 0..data.len() - l3 - 1 {
            cycled_data.push(data[k + l3] as u8);
        }
        cycled_data.push(data[data.len() - 1] as u8);
        for k in data.len() - l3..data.len() {
            cycled_data.push(data[k + l3 - data.len()] as u8);
        }
    } else {
        for k in 0..data.len() - l3 - 1 {
            cycled_data.push(exchange_bits_left(data[k + l3], data[k + l3 + 1], l2));
        }
        cycled_data.push(exchange_bits_left(data[data.len() - 1], data[0], l2));
        for k in data.len() - l3..data.len() {
            cycled_data.push(exchange_bits_left(
                data[k + l3 - data.len()],
                data[k + l3 - data.len() + 1],
                l2,
            ));
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

pub fn cycle_right(data: &[u8], bits_to_shift: &BigUint, cycled_data: &mut Vec<i8>) {
    let l1 = bits_to_shift % (8u64 * data.len() as u64);
    let l2 = (&l1 % 8usize).to_usize().unwrap();
    let l3 = (l1 / 8usize).to_usize().unwrap();

    if l2 == 0 {
        for k in 0..l3 {
            cycled_data.push(data[data.len() + k - l3] as i8);
        }

        cycled_data.push(data[0] as i8);

        for k in l3 + 1..data.len() {
            cycled_data.push(data[k - l3] as i8);
        }
    } else {
        for k in 0..l3 {
            cycled_data.push(exchange_bits_right(
                data[data.len() + k - l3 - 1],
                data[data.len() + k - l3],
                l2,
            ));
        }

        cycled_data.push(exchange_bits_right(
            data[data.len() - 1],
            data[0],
            l2,
        ));

        for k in l3 + 1..data.len() {
            cycled_data.push(exchange_bits_right(
                data[k - l3 - 1],
                data[k - l3],
                l2,
            ));
        }
    }
}

#[wasm_bindgen_test]
fn cycle_left_can_be_reversed_with_cycle_right() {
    use std::str::FromStr;
    let bits_to_shift = BigUint::from_str("110192826829776194000614388426091705128").unwrap();
    let unsigned_data = [
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
        194, 207, 152, 67, 182, 159, 250, 27, 212, 187, 51, 79, 55, 174, 207, 75, 74, 121, 82, 14,
        216, 48, 95, 108, 91, 186, 24, 81, 186, 197, 196, 45, 242, 193, 130, 15, 134, 17, 227, 215,
        185, 169, 246, 121, 149, 191, 168, 232, 101, 34, 221, 132, 160, 19, 216, 133, 17, 220, 255,
        132, 167, 220, 212, 83, 110, 21, 219, 35, 52, 22, 169, 59, 69, 104, 172, 201, 253, 5, 44,
        198, 194, 107, 155, 205, 99, 84, 104, 68, 117, 56, 38, 53, 105, 213, 85, 7, 118, 77, 155,
        63, 75, 218, 126, 24, 7, 248, 215, 122, 44, 165, 140, 177, 143, 186, 205, 119, 7, 101, 220,
        134, 158, 124, 213, 169, 178, 134, 86, 145, 1, 106, 133, 90, 184, 143, 70, 148, 5, 76, 114,
        161, 20, 220, 157, 52, 175, 27, 149, 210, 246, 92, 235, 75, 89, 122, 207, 104, 188, 39,
        176, 187, 96, 216, 21, 185, 228, 192, 48, 135, 199, 214, 208, 58, 4, 166, 115, 112, 13,
        150, 105, 236, 148, 106, 179, 196, 103, 35, 123, 99, 209, 129, 55, 167, 180, 144, 212, 138,
        252, 238, 183, 225, 166, 31, 223, 216, 33, 176, 183, 115, 250, 241, 42, 163, 89, 7, 63,
        185, 162, 197, 61, 127, 25, 82, 169, 232, 34, 175, 122, 97, 20, 87, 91, 31, 47, 233, 249,
        0, 67, 136, 186, 59, 148, 41, 134, 210, 230, 91, 192, 225, 198, 51, 19, 36, 234, 243, 76,
        33, 17, 69, 249, 71, 244, 139, 250, 248, 167, 1, 163, 72, 173, 92, 252, 132, 249, 231, 236,
        12, 100, 190, 227, 192, 16, 150, 244, 251, 190, 33, 179, 26, 125, 51, 119, 31, 137, 149,
        109, 84, 67, 238, 155, 87, 243, 105, 127, 240, 225, 230, 137, 165, 10, 53, 133, 156, 9,
        116, 48, 135, 171, 243, 247, 134, 113, 255, 17, 168, 32, 215, 125, 79, 35, 24, 104, 216,
        213, 93, 22, 211, 96, 234, 111, 134, 5, 236, 163, 80, 1, 103, 39, 96, 79, 168, 18, 11, 88,
        7, 165, 48, 48, 243, 247, 34, 30, 240, 98, 232, 7, 238, 32, 36, 102, 202, 145, 235, 184,
        36, 139, 142, 74, 209, 12, 28, 92, 233, 97, 240, 232, 120, 156, 46, 191, 79, 73, 160, 149,
        68, 49, 102, 247, 230, 94, 40, 162, 100, 40, 19, 232, 127, 214, 196, 12, 50, 54, 233, 88,
        246, 162, 55, 75, 205, 100, 90, 238, 4, 110, 118, 231, 195, 239, 109, 220, 193, 128, 223,
        97, 122, 233, 236, 43, 56, 233, 129, 125, 237, 131, 231, 171, 34, 234, 44, 209, 146, 24,
        61, 90, 173, 58, 121, 146, 13, 11, 133, 173, 235, 26, 136, 82, 186, 6, 32, 69, 172, 173, 9,
        207, 182, 110, 244, 74, 111, 87, 198, 70, 213, 230, 56, 77, 102, 145, 143, 49, 69, 179,
        209, 243, 179, 69, 187, 34, 71, 66, 65, 76, 168, 244, 151, 186, 137, 167, 151, 54, 39, 38,
        207, 126, 123, 213, 167, 164, 198, 216, 91, 132, 238, 6, 88, 153, 67, 83, 248, 110, 220,
        174, 193, 41, 65, 165, 243, 58, 121, 241, 210, 138, 247, 164, 169, 238, 99, 29, 37, 91, 92,
        90, 189, 23, 171, 223, 152, 56, 39, 121, 184, 78, 95, 151, 46, 136, 253, 214, 145, 233,
        253, 222, 7, 29, 180, 94, 222, 77, 249, 200, 78, 160, 117, 72, 94, 128, 89, 253, 115, 108,
        88, 223, 231, 147, 244, 108, 182, 103, 173, 55, 7, 159, 135, 110, 215, 14, 227, 223, 110,
        168, 222, 28, 107, 122, 194, 151, 217, 251, 89, 136, 59, 235, 165, 206, 150, 175, 71, 20,
        156, 72, 128, 134, 128, 158, 22, 167, 48, 36, 90, 252, 212, 192, 142, 132, 13, 36, 247,
        153, 217, 249, 108, 108, 89, 18, 134, 12, 118, 137, 47, 237, 155, 12, 121, 226, 42, 168,
        22, 115, 74, 210, 199, 96, 201, 102, 246, 222, 77, 179, 74, 64, 119, 120, 241, 241, 145,
        235, 179, 221, 254, 217, 117, 231, 133, 183, 147, 123, 26, 149, 146, 90, 227, 61, 119, 69,
        74, 210, 2, 191, 96, 70, 101, 47, 150, 36, 109, 143, 146, 190, 17, 205, 71, 27, 30, 102,
        203, 246, 26, 242, 241, 17, 11, 222, 229, 165, 191, 130, 124, 250, 103, 26, 151, 189, 200,
        104, 53, 143, 37, 83, 238, 140, 5, 240, 83, 106, 210, 83, 251, 65, 38, 249, 119, 143, 81,
        190, 81, 3, 153, 109, 27, 236, 183, 143, 190, 243, 131, 86, 226, 128, 182, 80, 70, 170, 67,
        9, 18, 162, 147, 70, 120, 82, 97, 85, 234, 80, 245, 220, 49, 186, 245, 152, 24, 4, 33, 69,
        13, 199, 212, 184, 215, 8, 21, 123, 131, 25, 95, 64, 156, 43, 51, 123, 215, 192, 181, 153,
        198, 236, 184, 2, 20, 50, 208, 32, 21, 227, 48, 250, 211, 113, 208, 247, 112, 228, 228, 52,
        247, 50, 168, 186, 86, 224, 236, 246, 160, 100, 124, 176, 65, 82, 231, 235, 183, 206, 117,
        96, 51, 163, 255, 14, 145, 62, 181, 112, 117, 2, 215, 36, 113, 104, 240, 135, 211, 170,
        225, 41, 229, 214, 78, 129, 77, 15, 128, 68, 155, 105, 212, 130, 142, 107, 31, 8, 37, 92,
        235, 202, 114, 189, 185, 253, 186, 98, 102, 33, 229, 148, 15, 138, 148, 48, 62, 54, 59, 10,
        107, 170, 40, 244, 176, 26, 220, 110, 80, 108, 32, 204, 152, 40, 13, 36, 181, 182, 176, 43,
        116, 172, 87, 102, 74, 30, 6, 38, 29, 45, 239, 165, 208, 85, 13, 44, 65, 229, 120, 110,
        250, 112, 70, 11, 203, 67, 93, 84, 4, 47, 43, 11, 150, 16, 41, 205, 204, 157, 111, 253, 26,
        76, 241, 37, 206, 255, 3, 171, 255, 106, 147, 140, 114, 95, 188, 188, 210, 67, 230, 226,
        122, 37, 157, 253, 88, 124, 86, 99, 135, 222, 129, 187, 166, 185, 168, 130, 135, 57, 222,
        119, 105, 55, 92, 180, 195, 110, 43, 120, 42, 176, 219, 81, 57, 159, 68, 190, 221, 76, 190,
        124, 207, 174, 134, 7, 203, 181, 81, 144, 194, 62, 219, 230, 119, 17, 147, 10, 53, 69, 117,
        204, 11, 246, 185, 108, 180, 251, 114, 160, 169, 45, 10, 49, 86, 240, 10, 139, 51, 161,
        233, 216, 56, 199, 118, 242, 65, 163, 5, 215, 194, 97, 45, 3, 239, 98, 132, 151, 199, 89,
        33, 204, 167, 158, 34, 148, 173, 195, 51, 96, 203, 109, 54, 44, 195, 23, 178, 55, 8, 208,
        223, 193, 32, 3, 143, 220, 242, 72, 112, 60, 29, 106, 246, 184, 209, 102, 95, 28, 43, 159,
        178, 102, 27, 247, 27, 48, 85, 168, 15, 14, 243, 203, 204, 103, 118, 124, 244, 185, 24,
        108, 147, 172, 219, 248, 40, 29, 169, 206, 3, 23, 198, 190, 125, 37, 139, 124, 2, 39, 107,
        171, 38, 13, 35, 194, 103, 147, 249, 250, 252, 26, 149, 238, 211, 236, 53, 77, 244, 22, 55,
        208, 236, 97, 19, 24, 220, 0, 208, 19,
    ];

    let mut data = vec![0; unsigned_data.len()];
    for i in 0..unsigned_data.len() {
        data[i] = unsigned_data[i] as i8;
    }

    let mut cycled_left_data = Vec::new();
    cycle_left(&data, &bits_to_shift, &mut cycled_left_data);

    let mut uncycled_data = Vec::new();
    cycle_right(&cycled_left_data, &bits_to_shift, &mut uncycled_data);

    for (d, u) in data.iter().zip(uncycled_data.iter()) {
        assert_eq!(d, u);
    }
}

#[wasm_bindgen_test]
fn cycle_right_can_be_reversed_with_cycle_left() {
    use std::str::FromStr;
    let bits_to_shift = BigUint::from_str("110192826829776194000614388426091705128").unwrap();
    let raw_data = [
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
        194, 207, 152, 67, 182, 159, 250, 27, 212, 187, 51, 79, 55, 174, 207, 75, 74, 121, 82, 14,
        216, 48, 95, 108, 91, 186, 24, 81, 186, 197, 196, 45, 242, 193, 130, 15, 134, 17, 227, 215,
        185, 169, 246, 121, 149, 191, 168, 232, 101, 34, 221, 132, 160, 19, 216, 133, 17, 220, 255,
        132, 167, 220, 212, 83, 110, 21, 219, 35, 52, 22, 169, 59, 69, 104, 172, 201, 253, 5, 44,
        198, 194, 107, 155, 205, 99, 84, 104, 68, 117, 56, 38, 53, 105, 213, 85, 7, 118, 77, 155,
        63, 75, 218, 126, 24, 7, 248, 215, 122, 44, 165, 140, 177, 143, 186, 205, 119, 7, 101, 220,
        134, 158, 124, 213, 169, 178, 134, 86, 145, 1, 106, 133, 90, 184, 143, 70, 148, 5, 76, 114,
        161, 20, 220, 157, 52, 175, 27, 149, 210, 246, 92, 235, 75, 89, 122, 207, 104, 188, 39,
        176, 187, 96, 216, 21, 185, 228, 192, 48, 135, 199, 214, 208, 58, 4, 166, 115, 112, 13,
        150, 105, 236, 148, 106, 179, 196, 103, 35, 123, 99, 209, 129, 55, 167, 180, 144, 212, 138,
        252, 238, 183, 225, 166, 31, 223, 216, 33, 176, 183, 115, 250, 241, 42, 163, 89, 7, 63,
        185, 162, 197, 61, 127, 25, 82, 169, 232, 34, 175, 122, 97, 20, 87, 91, 31, 47, 233, 249,
        0, 67, 136, 186, 59, 148, 41, 134, 210, 230, 91, 192, 225, 198, 51, 19, 36, 234, 243, 76,
        33, 17, 69, 249, 71, 244, 139, 250, 248, 167, 1, 163, 72, 173, 92, 252, 132, 249, 231, 236,
        12, 100, 190, 227, 192, 16, 150, 244, 251, 190, 33, 179, 26, 125, 51, 119, 31, 137, 149,
        109, 84, 67, 238, 155, 87, 243, 105, 127, 240, 225, 230, 137, 165, 10, 53, 133, 156, 9,
        116, 48, 135, 171, 243, 247, 134, 113, 255, 17, 168, 32, 215, 125, 79, 35, 24, 104, 216,
        213, 93, 22, 211, 96, 234, 111, 134, 5, 236, 163, 80, 1, 103, 39, 96, 79, 168, 18, 11, 88,
        7, 165, 48, 48, 243, 247, 34, 30, 240, 98, 232, 7, 238, 32, 36, 102, 202, 145, 235, 184,
        36, 139, 142, 74, 209, 12, 28, 92, 233, 97, 240, 232, 120, 156, 46, 191, 79, 73, 160, 149,
        68, 49, 102, 247, 230, 94, 40, 162, 100, 40, 19, 232, 127, 214, 196, 12, 50, 54, 233, 88,
        246, 162, 55, 75, 205, 100, 90, 238, 4, 110, 118, 231, 195, 239, 109, 220, 193, 128, 223,
        97, 122, 233, 236, 43, 56, 233, 129, 125, 237, 131, 231, 171, 34, 234, 44, 209, 146, 24,
        61, 90, 173, 58, 121, 146, 13, 11, 133, 173, 235, 26, 136, 82, 186, 6, 32, 69, 172, 173, 9,
        207, 182, 110, 244, 74, 111, 87, 198, 70, 213, 230, 56, 77, 102, 145, 143, 49, 69, 179,
        209, 243, 179, 69, 187, 34, 71, 66, 65, 76, 168, 244, 151, 186, 137, 167, 151, 54, 39, 38,
        207, 126, 123, 213, 167, 164, 198, 216, 91, 132, 238, 6, 88, 153, 67, 83, 248, 110, 220,
        174, 193, 41, 65, 165, 243, 58, 121, 241, 210, 138, 247, 164, 169, 238, 99, 29, 37, 91, 92,
        90, 189, 23, 171, 223, 152, 56, 39, 121, 184, 78, 95, 151, 46, 136, 253, 214, 145, 233,
        253, 222, 7, 29, 180, 94, 222, 77, 249, 200, 78, 160, 117, 72, 94, 128, 89, 253, 115, 108,
        88, 223, 231, 147, 244, 108, 182, 103, 173, 55, 7, 159, 135, 110, 215, 14, 227, 223, 110,
        168, 222, 28, 107, 122, 194, 151, 217, 251, 89, 136, 59, 235, 165, 206, 150, 175, 71, 20,
        156, 72, 128, 134, 128, 158, 22, 167, 48, 36, 90, 252, 212, 192, 142, 132, 13, 36, 247,
        153, 217, 249, 108, 108, 89, 18, 134, 12, 118, 137, 47, 237, 155, 12, 121, 226, 42, 168,
        22, 115, 74, 210, 199, 96, 201, 102, 246, 222, 77, 179, 74, 64, 119, 120, 241, 241, 145,
        235, 179, 221, 254, 217, 117, 231, 133, 183, 147, 123, 26, 149, 146, 90, 227, 61, 119, 69,
        74, 210, 2, 191, 96, 70, 101, 47, 150, 36, 109, 143, 146, 190, 17, 205, 71, 27, 30, 102,
        203, 246, 26, 242, 241, 17, 11, 222, 229, 165, 191, 130, 124, 250, 103, 26, 151, 189, 200,
        104, 53, 143, 37, 83, 238, 140, 5, 240, 83, 106, 210, 83, 251, 65, 38, 249, 119, 143, 81,
        190, 81, 3, 153, 109, 27, 236, 183, 143, 190, 243, 131, 86, 226, 128, 182, 80, 70, 170, 67,
        9, 18, 162, 147, 70, 120, 82, 97, 85, 234, 80, 245, 220, 49, 186, 245, 152, 24, 4, 33, 69,
        13, 199, 212, 184, 215, 8, 21, 123, 131, 25, 95, 64, 156, 43, 51, 123, 215, 192, 181, 153,
        198, 236, 184, 2, 20, 50, 208, 32, 21, 227, 48, 250, 211, 113, 208, 247, 112, 228, 228, 52,
        247, 50, 168, 186, 86, 224, 236, 246, 160, 100, 124, 176, 65, 82, 231, 235, 183, 206, 117,
        96, 51, 163, 255, 14, 145, 62, 181, 112, 117, 2, 215, 36, 113, 104, 240, 135, 211, 170,
        225, 41, 229, 214, 78, 129, 77, 15, 128, 68, 155, 105, 212, 130, 142, 107, 31, 8, 37, 92,
        235, 202, 114, 189, 185, 253, 186, 98, 102, 33, 229, 148, 15, 138, 148, 48, 62, 54, 59, 10,
        107, 170, 40, 244, 176, 26, 220, 110, 80, 108, 32, 204, 152, 40, 13, 36, 181, 182, 176, 43,
        116, 172, 87, 102, 74, 30, 6, 38, 29, 45, 239, 165, 208, 85, 13, 44, 65, 229, 120, 110,
        250, 112, 70, 11, 203, 67, 93, 84, 4, 47, 43, 11, 150, 16, 41, 205, 204, 157, 111, 253, 26,
        76, 241, 37, 206, 255, 3, 171, 255, 106, 147, 140, 114, 95, 188, 188, 210, 67, 230, 226,
        122, 37, 157, 253, 88, 124, 86, 99, 135, 222, 129, 187, 166, 185, 168, 130, 135, 57, 222,
        119, 105, 55, 92, 180, 195, 110, 43, 120, 42, 176, 219, 81, 57, 159, 68, 190, 221, 76, 190,
        124, 207, 174, 134, 7, 203, 181, 81, 144, 194, 62, 219, 230, 119, 17, 147, 10, 53, 69, 117,
        204, 11, 246, 185, 108, 180, 251, 114, 160, 169, 45, 10, 49, 86, 240, 10, 139, 51, 161,
        233, 216, 56, 199, 118, 242, 65, 163, 5, 215, 194, 97, 45, 3, 239, 98, 132, 151, 199, 89,
        33, 204, 167, 158, 34, 148, 173, 195, 51, 96, 203, 109, 54, 44, 195, 23, 178, 55, 8, 208,
        223, 193, 32, 3, 143, 220, 242, 72, 112, 60, 29, 106, 246, 184, 209, 102, 95, 28, 43, 159,
        178, 102, 27, 247, 27, 48, 85, 168, 15, 14, 243, 203, 204, 103, 118, 124, 244, 185, 24,
        108, 147, 172, 219, 248, 40, 29, 169, 206, 3, 23, 198, 190, 125, 37, 139, 124, 2, 39, 107,
        171, 38, 13, 35, 194, 103, 147, 249, 250, 252, 26, 149, 238, 211, 236, 53, 77, 244, 22, 55,
        208, 236, 97, 19, 24, 220, 0, 208, 19,
    ];

    let mut cycled_right_data = Vec::new();
    cycle_right(&raw_data, &bits_to_shift, &mut cycled_right_data);

    let mut uncycled_data = Vec::new();
    cycle_left(&cycled_right_data, &bits_to_shift, &mut uncycled_data);

    for (d, u) in raw_data.iter().zip(uncycled_data.iter()) {
        assert_eq!(d, u);
    }
}
