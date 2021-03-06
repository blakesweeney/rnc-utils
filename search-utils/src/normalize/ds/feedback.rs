use std::{
    collections::HashSet,
    iter::FromIterator,
};

use serde::{
    Deserialize,
    Serialize,
};

// use serde_with::CommaSeparator;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feedback {
    overlaps_with: Vec<String>,
    no_overlaps_with: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackVec {
    overlaps_with: HashSet<String>,
    no_overlaps_with: HashSet<String>,
}

impl Default for FeedbackVec {
    fn default() -> Self {
        Self {
            overlaps_with: HashSet::new(),
            no_overlaps_with: HashSet::new(),
        }
    }
}

impl FromIterator<Feedback> for FeedbackVec {
    fn from_iter<I: IntoIterator<Item = Feedback>>(iter: I) -> Self {
        let mut value = FeedbackVec::default();

        for i in iter {
            value.overlaps_with.extend(i.overlaps_with);
            value.no_overlaps_with.extend(i.no_overlaps_with);
        }

        value
    }
}
