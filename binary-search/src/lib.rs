// // Deliberetely trying not to use explicit pointers,
// // using slices and some fancy ptr arith instead.
// pub fn find<T: PartialOrd, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
//     let array = array.as_ref();
//     let mut slice = array.as_ref();

//     while !slice.is_empty() {
//         let mid = slice.len() / 2;
//         let x = &slice[mid];
//         slice = if key < *x {
//             &slice[..mid]
//         } else if key > *x {
//             &slice[mid + 1..]
//         } else {
//             let start = &array[0] as *const T as usize;
//             let found = &slice[mid] as *const T as usize;
//             let size = std::mem::size_of::<T>();
//             return Some((found - start) / size);
//         }
//     }
//     None
// }

// Primitive version with no overflow checks.
// Still have no idea how to avoid bound checks :|
pub fn find<T: PartialOrd, U: AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut l = 0;
    let mut r = array.len();
    while l < r {
        let mid = l + (r - l - 1) / 2;
        let x = &array[mid];
        if key < *x {
            r = mid;
        } else if key > *x {
            l = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}
