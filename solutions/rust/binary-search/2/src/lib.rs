pub fn find<T: Ord, H: AsRef<[T]>>(array: H, key: T) -> Option<usize> {
    let array = array.as_ref();
    if array.is_empty() {
        return None
    }
    if key < array[0] {
        return None
    }

    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if array[mid] == key {
            return Some(mid)
        } else if array[mid] < key {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}
