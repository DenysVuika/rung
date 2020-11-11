use std::cmp::Ordering;

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
