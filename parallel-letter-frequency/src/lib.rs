use crossbeam::thread;
use std::collections::HashMap;

fn count_chars(map: &mut HashMap<char, usize>, line: &str) {
    // try flat_map
    for c in line.chars() {
        // did not check if this is actually more efficient
        // than lowercasing the whole string at once,
        // but I hope Rust is smart enough to do the iterator
        // overhead elimination magic here for me :)
        if c.is_alphabetic() {
            for lc in c.to_lowercase() {
                let count = map.entry(lc).or_insert(0);
                *count += 1;
            }
        }
    }
}

/// extends `to` with `from` calling `f` on conflicts
fn merge_into_map(
    to: &mut HashMap<char, usize>,
    from: &HashMap<char, usize>,
    mut f: impl FnMut(usize, usize) -> usize,
) {
    for (key, from_val) in from {
        to.entry(*key)
            .and_modify(|to_val| *to_val = f(*to_val, *from_val))
            .or_insert(*from_val);
    }
}

// The type of input clearly suggests to split work using lines.
// Surely, it's possible to have a giant line among short ones,
// but it's coded for fun anyway :)
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
        // iterators are lazy so we need to collect() them first
        let threads: Vec<_> = (0..worker_count)
            .map(|n| {
                s.spawn(move |_| {
                    let mut char_count = HashMap::with_capacity(256);
                    for &line in input.iter().skip(n).step_by(worker_count) {
                        count_chars(&mut char_count, line);
                    }
                    char_count
                })
            })
            .collect();

        threads
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .fold(HashMap::with_capacity(256), |mut acc, map| {
                merge_into_map(&mut acc, &map, |to, from| to + from);
                acc
            })
    })
    .unwrap()
}
