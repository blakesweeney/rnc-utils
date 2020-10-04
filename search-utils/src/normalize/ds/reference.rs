use std::{
    collections::HashSet,
    iter::FromIterator,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reference {
    authors: String,
    journal: String,
    pub_title: String,
    pub_id: String,
    pubmed_id: String,
    doi: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceVec {
    authors: HashSet<String>,
    journal: HashSet<String>,
    pub_title: HashSet<String>,
    pub_id: HashSet<String>,
    pubmed_id: HashSet<String>,
    doi: HashSet<String>,
}

impl Default for ReferenceVec {
    fn default() -> Self {
        Self {
            authors: HashSet::new(),
            journal: HashSet::new(),
            pub_title: HashSet::new(),
            pub_id: HashSet::new(),
            pubmed_id: HashSet::new(),
            doi: HashSet::new(),
        }
    }
}

impl FromIterator<Reference> for ReferenceVec {
    fn from_iter<I: IntoIterator<Item = Reference>>(iter: I) -> Self {
        let mut value = ReferenceVec::default();

        for i in iter {
            value.authors.insert(i.authors);
            value.journal.insert(i.journal);
            value.pub_title.insert(i.pub_title);
            value.pub_id.insert(i.pub_id);
            value.pubmed_id.insert(i.pubmed_id);
            value.doi.insert(i.doi);
        }

        value
    }
}
