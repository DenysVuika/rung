use log::error;
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Verifies that all files exist
pub fn verify_files(paths: &[&Path]) -> bool {
    paths.iter().all(|path| {
        let exists = path.exists();
        if !exists {
            error!("`{}` not found", path.to_str().unwrap());
        }
        exists
    })
}

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

pub fn get_lines(path: &Path) -> Vec<String> {
    match read_to_string(&path) {
        Ok(content) => content.lines().map(|line| line.to_string()).collect(),
        Err(err) => {
            let path_str = path.to_str().unwrap();

            error!("Error loading `{}`. {}", path_str, err);
            vec![]
        }
    }
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
