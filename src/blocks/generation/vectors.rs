use crate::blocks::BlockSize;
use std::ops::Sub;

fn are_orthogonal(vector1: &[u16], vector2: &[i16]) -> bool {
    if (vector1.iter().count() != vector2.iter().count()) {
        panic!("vectors have different sizes");
    }
    vector1
        .iter()
        .zip(vector2)
        .fold(0i64, |acc, (&e1, &e2)| acc + e1 as i64 * e2 as i64)
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::SaltSize;

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
}
