// O(N*M) memory solution with ~O(N) computational

// use std::collections::HashSet;

// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//     let mut seen = HashSet::new();
//     let mut sum = 0;
//     for &factor in factors {
//         if factor == 0 {
//             continue;
//         }
//         for num in (factor..limit).step_by(factor as usize) {
//             if !seen.contains(&num) {
//                 seen.insert(num);
//                 sum += num;
//             }
//         }
//     }

//     return sum;
// }

// O(N) memory solution with O(N*log(N)) computational

use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::BinaryHeap;

// partially copying Peekable because it's peek() cannot
// be used in cmp() as the self must be immutable for cmp().

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Peekable<I: Iterator> {
    iter: I,
    /// Remember a peeked value, even if it was None.
    peeked: Option<I::Item>,
}

impl<I: Iterator> Peekable<I> {
    pub fn new(mut iter: I) -> Peekable<I> {
        let peeked = iter.next();
        Peekable { iter, peeked }
    }

    fn is_empty(&self) -> bool {
        self.peeked.is_none()
    }
}

impl<It: Ord, I: Iterator<Item = It>> PartialEq for Peekable<I> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl<It: Ord, I: Iterator<Item = It>> Eq for Peekable<I> {}

impl<It: Ord, I: Iterator<Item = It>> PartialOrd for Peekable<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<It: Ord, I: Iterator<Item = It>> Ord for Peekable<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        // sort accending
        other.peeked.cmp(&self.peeked)
    }
}

fn merge_sorted_iters<Item: Ord, I: Iterator<Item = Item>>(
    iters: Vec<I>,
) -> impl std::iter::Iterator<Item = Item> {
    let mut pq = BinaryHeap::new();
    for iter in iters.into_iter() {
        let p = Peekable::new(iter);
        if !p.is_empty() {
            pq.push(p);
        }
    }

    std::iter::from_fn(move || {
        if let Some(peekable) = pq.pop() {
            let p = Peekable::new(peekable.iter);
            if !p.is_empty() {
                pq.push(p);
            }
            return peekable.peeked;
        }
        return None;
    })
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let factors_iters = factors
        .iter()
        .filter(|&&factor| factor > 0)
        .map(|&factor| (factor..limit).step_by(factor as usize))
        .collect();

    // for num in merge_sorted_iters(factors_iters) {
    //     dbg!(num);
    // }

    // 0
    merge_sorted_iters(factors_iters).dedup().sum()
}
