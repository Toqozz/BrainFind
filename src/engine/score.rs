/// Points assigned to a Choice. The smaller the better.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Score {
    match_length: usize,
    index: usize,
}

impl Score {
    pub fn new(match_start: usize, match_end: usize) -> Self {
        Self {
            index: match_start,
            match_length: match_end - match_start,
        }
    }
}

impl From<(usize, usize)> for Score {
    fn from(tuple: (usize, usize)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}
