#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self(scores)
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().copied()
    }
    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().max().copied()
    }
    pub fn personal_top_three(&self) -> Vec<u32> {
        // using a small fixed size vector
        // instead of a heap or a tree for simplicity
        self.0.iter().fold(Vec::with_capacity(4), |mut acc, x| {
            acc.push(*x);
            // reverse sort
            acc.sort_unstable_by(|a, b| b.cmp(a));
            if acc.len() == 4 {
                acc.pop();
            }
            acc
        })
    }
}
