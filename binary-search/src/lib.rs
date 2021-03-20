// Deliberetely trying not to use explicit pointers,
// using slices and some fancy ptr arith instead.
pub fn find<T: Eq + Ord>(array: &[T], key: T) -> Option<usize> {
    let mut slice = array;

    while !slice.is_empty() {
        let mid = slice.len() / 2;
        let x = &slice[mid];
        slice = if key < *x {
            &slice[..mid]
        } else if key > *x {
            &slice[mid + 1..]
        } else {
            let start = array.as_ptr() as usize;
            let found = &slice[mid] as *const T as usize;
            let size = std::mem::size_of::<i32>();
            return Some((found - start) / size);
        }
    }
    None
}
