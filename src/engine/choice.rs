use std::fmt;
use std::cmp::Ordering;

use super::score::Score;

/// Wrapper around String so it orders strings by len and not bytes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct OrderlyString(String);

impl Ord for OrderlyString {
    fn cmp(&self, other: &OrderlyString) -> Ordering {
        self.0.len().cmp(&other.0.len())
    }
}

impl PartialOrd for OrderlyString {
    fn partial_cmp(&self, other: &OrderlyString) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for OrderlyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for OrderlyString {
    fn from(text: String) -> Self {
        OrderlyString(text)
    }
}

/// A choice represents an element in the list that matches against the current user's query.
///
/// It has the original text, the score (how good is the match) and where the match starts and
/// ends.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Choice {
    match_start: usize,
    match_end: usize,
    score: Score,
    text: OrderlyString,
}

impl Choice {
    /// Build a new Choice.
    pub fn new(text: String, match_start: usize, match_end: usize) -> Self {
        Self {
            match_start,
            match_end,
            score: (match_start, match_end).into(),
            text: text.into(),
        }
    }

    /// The character index where the matching starts
    ///
    /// The character under this index is included in the match.
    pub fn start(&self) -> usize {
        self.match_start
    }

    /// The character index where the matching ends
    ///
    /// The character under this index is NOT included in the match.
    /// It's an open ending.
    pub fn end(&self) -> usize {
        self.match_end
    }
}

impl Ord for Choice {
    fn cmp(&self, other: &Choice) -> Ordering {
        let by_score = self.score.cmp(&other.score);

        by_score.then_with(|| self.text.cmp(&other.text))
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Choice) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<(String, usize, usize)> for Choice {
    fn from(tuple: (String, usize, usize)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl From<String> for Choice {
    fn from(text: String) -> Self {
        Self::new(text, 0, 0)
    }
}