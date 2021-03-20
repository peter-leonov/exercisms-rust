// Deliberetely trying not to use explicit pointers,
// using slices and some fancy ptr arith instead.
pub fn find<T: PartialOrd, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut slice = array.as_ref();

    while !slice.is_empty() {
        let mid = slice.len() / 2;
        let x = &slice[mid];
        slice = if key < *x {
            &slice[..mid]
        } else if key > *x {
            &slice[mid + 1..]
        } else {
            let start = &array[0] as *const T as usize;
            let found = &slice[mid] as *const T as usize;
            let size = std::mem::size_of::<T>();
            return Some((found - start) / size);
        }
    }
    None
}
