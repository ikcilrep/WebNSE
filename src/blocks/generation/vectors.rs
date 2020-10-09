pub fn are_orthogonal(vector1: &[u16], vector2: &[i16]) -> bool {
    if vector1.len() != vector2.len() {
        panic!("vectors are of different size");
    }
    vector1
        .iter()
        .zip(vector2)
        .fold(0i64, |acc, (&e1, &e2)| acc + e1 as i64 * e2 as i64)
        == 0
}

pub fn vector_difference(vector1: &[i8], vector2: &[i8], output: &mut [i16]) {
    let vector_size = vector1.len();
    if vector_size != vector2.len() {
        panic!("vectors are of different size");
    }
    if vector_size != output.len() {
        panic!("output is of different size than vectors");
    }

    for i in 0..vector_size {
        output[i] = vector1[i] as i16 - vector2[i] as i16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn are_orthogonal_returns_true_when_orthogonal() {
        assert!(are_orthogonal(&[1, 1, 1, 1], &[-1, 1, -1, 1]));
        assert!(are_orthogonal(&[1, 1, 3, 1], &[-1, -2, 1, 0]));
    }

    #[test]
    fn are_orthogonal_returns_false_when_not_orthogonal() {
        assert!(!are_orthogonal(&[2, 3, 7, 1], &[14, 23, 54, 12]));
        assert!(!are_orthogonal(&[1, 2, 5, 1], &[-121, 2, 5, 1]));
    }

    #[test]
    #[should_panic]
    fn are_orthogonal_panics_when_vectors_are_different_size() {
        are_orthogonal(&[2, 3, 7], &[14, 23, 54, 12]);
    }

    #[test]
    fn vector_difference_returns_vector_difference() {
        let mut difference1 = [0; 3];
        vector_difference(&[2, -123, 43], &[5, 124, -128], &mut difference1);
        assert_eq!(difference1, [-3, -247, 171]);

        let mut difference2 = [0; 4];
        vector_difference(&[85, 92, -32, -76], &[-19, 12, 65, 43], &mut difference2);

        assert_eq!(difference2, [104, 80, -97, -119]);
    }

    #[test]
    #[should_panic(expected = "vectors are of different size")]
    fn vector_difference_panics_when_vectors_are_of_different_size() {
        let mut difference = [0; 3];
        vector_difference(&[2, -123, 43], &[5, 124, -128, 1], &mut difference);
    }

    #[test]
    #[should_panic(expected = "output is of different size than vectors")]
    fn vector_difference_panics_when_difference_is_of_different_size_than_vectors() {
        let mut difference = [0; 3];
        vector_difference(&[2, -123, 43, 12], &[5, 124, -128, 1], &mut difference);
    }
}
