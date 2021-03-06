use std::{
    collections::HashSet,
    iter::FromIterator,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractingProtein {
    interacting_protein_id: String,
    synonyms: Vec<String>,
    label: String,
    relationship: String,
    methods: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractingProteinVec {
    interacting_protein_id: HashSet<String>,
    synonyms: HashSet<String>,
    label: HashSet<String>,
    relationship: HashSet<String>,
    methods: HashSet<String>,
}

impl Default for InteractingProteinVec {
    fn default() -> Self {
        Self {
            interacting_protein_id: HashSet::new(),
            synonyms: HashSet::new(),
            label: HashSet::new(),
            relationship: HashSet::new(),
            methods: HashSet::new(),
        }
    }
}

impl FromIterator<InteractingProtein> for InteractingProteinVec {
    fn from_iter<I: IntoIterator<Item = InteractingProtein>>(iter: I) -> Self {
        let mut value = InteractingProteinVec::default();

        for i in iter {
            value.interacting_protein_id.insert(i.interacting_protein_id);
            value.synonyms.extend(i.synonyms);
            value.label.insert(i.label);
            value.relationship.insert(i.relationship);
            value.methods.extend(i.methods);
        }

        value
    }
}
