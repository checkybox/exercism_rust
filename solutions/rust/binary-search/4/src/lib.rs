use std::cmp::Ordering;

pub fn find<T: Ord, H: AsRef<[T]>>(array: H, key: T) -> Option<usize> {
    let array = array.as_ref();

    let mut low = 0;
    let mut high = array.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }

    None
}