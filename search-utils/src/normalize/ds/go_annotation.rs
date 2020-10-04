use std::{
    collections::HashSet,
    iter::FromIterator,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoAnnotation {
    go_term_id: String,
    qualifier: String,
    go_name: String,
    assigned_by: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoAnnotationVec {
    go_term_id: HashSet<String>,
    qualifier: HashSet<String>,
    go_name: HashSet<String>,
    assigned_by: HashSet<String>,
}

impl Default for GoAnnotationVec {
    fn default() -> Self {
        Self {
            go_term_id: HashSet::new(),
            qualifier: HashSet::new(),
            go_name: HashSet::new(),
            assigned_by: HashSet::new(),
        }
    }
}

impl FromIterator<GoAnnotation> for GoAnnotationVec {
    fn from_iter<I: IntoIterator<Item = GoAnnotation>>(iter: I) -> Self {
        let mut value = GoAnnotationVec::default();

        for i in iter {
            value.go_term_id.insert(i.go_term_id);
            value.qualifier.insert(i.qualifier);
            value.go_name.insert(i.go_name);
            value.assigned_by.insert(i.assigned_by);
        }

        value
    }
}
