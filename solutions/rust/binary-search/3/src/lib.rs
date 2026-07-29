use std::cmp::Ordering;

pub fn find<T: Ord, H: AsRef<[T]>>(array: H, key: T) -> Option<usize> {
    let array = array.as_ref();
    if array.is_empty() {
        return None
    }

    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => {
                if mid == 0 {
                    return None
                }
                high = mid - 1
            },
        }
    }

    None
}
