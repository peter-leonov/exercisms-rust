use crossbeam::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
        let result = s
            .spawn(|_| {
                let mut char_count = HashMap::with_capacity(256);
                for line in input {
                    // try flat_map
                    for c in line.chars() {
                        // did not check if this is actually more efficient
                        // than lowercasing the whole string at once,
                        // but I hope Rust is smart enough to do the iterator
                        // overhead elimination magic here for me :)
                        if c.is_alphabetic() {
                            for lc in c.to_lowercase() {
                                let count = char_count.entry(lc).or_insert(0);
                                *count += 1;
                            }
                        }
                    }
                }
                char_count
            })
            .join()
            .unwrap();

        result
    })
    .unwrap()
}
