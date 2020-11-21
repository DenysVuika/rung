use std::cmp::Ordering;

// Compares two vectors
pub fn compare<T: Ord>(a: &[T], b: &[T]) -> Ordering {
    let mut iter_b = b.iter();

    for v in a {
        match iter_b.next() {
            Some(w) => match v.cmp(w) {
                Ordering::Equal => continue,
                ord => return ord,
            },
            None => break,
        }
    }

    a.len().cmp(&b.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_works_as_expected() {
        assert_eq!(Ordering::Equal, compare(&[1, 2, 3], &[1, 2, 3]));
        assert_eq!(Ordering::Less, compare(&[1, 0], &[1, 2]));
        assert_eq!(Ordering::Less, compare(&[], &[1, 2, 3]));
        assert_eq!(Ordering::Greater, compare(&[1, 2, 3], &[1, 2]));
        assert_eq!(Ordering::Greater, compare(&[1, 3], &[1, 2]));
    }
}
