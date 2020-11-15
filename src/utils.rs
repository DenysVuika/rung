use log::error;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
    return a.len().cmp(&b.len());
}

pub fn get_top_lines(path: &Path, size: usize) -> Vec<String> {
    let input = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            error!("Error opening file {}", path.to_str().unwrap());
            return vec![];
        }
    };
    BufReader::new(input)
        .lines()
        .take(size)
        .map(|item| item.unwrap())
        .collect()
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
