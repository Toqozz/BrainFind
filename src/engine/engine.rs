use regex::Regex;
use std::time::Instant;
//use rayon::prelude::*;

use super::pattern::Pattern;
use super::refine::refine;
use super::errors::Error;
use super::choice::Choice;

/// This struct does the fuzzy search over a list of strings
///
/// You create a struct instance with all the list items and then you use that instance to filter
/// the list with different queries (list of chars).
///
/// # Example
///
/// ```
/// use scout::Scout;
///
/// let list = vec!["d/e/f.rs", "a/a/b/c.rs", "a/b/c.rs"];
/// let scout = Scout::new(list);
///
/// let query = ['a', 'b', 'c'];
/// let choices = scout.explore(&query);
///
/// let expected = vec!["a/b/c.rs", "a/a/b/c.rs"];
/// let actual: Vec<String> = choices.into_iter().map(|choice| choice.to_string()).collect();
///
/// assert_eq!(expected, actual);
/// ```
pub struct Scout<'a> {
    list: Vec<&'a str>,
}

impl<'a> Scout<'a> {
    /// Create a new Scout instance with a list of strings
    pub fn new(list: Vec<&'a str>) -> Self {
        Self { list }
    }

    /// Search for the choices that match a query, sorted by best match first.
    ///
    /// If the query is empty, it returns all the choices with the original order of the items.
    pub fn explore<'b>(&self, query: &'b [char]) -> Vec<Choice> {
        if query.is_empty() {
            return self.list
                .iter()
                .map(|text| text.to_string().into())
                .collect::<Vec<Choice>>();
        }

        let re = match self.regex(query) {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        };

        let mut choices: Vec<Choice> = self.list
            .iter()
            .map(|line| refine(&re, line))
            .filter(|choice| choice.is_some())
            .map(|choice| choice.unwrap())
            .collect();

        choices.sort();

        choices
    }

    /// Get a Regex from a list of chars.
    fn regex<'b>(&self, query: &'b [char]) -> Result<Regex, Error> {
        let pattern: Pattern = query.into();
        let regex = Regex::new(&pattern.to_string())?;

        Ok(regex)
    }
}
