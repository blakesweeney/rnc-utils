use std::{
    collections::HashSet,
    iter::FromIterator,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractingRna {
    interacting_rna_id: String,
    urs: String,
    methods: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractingRnaVec {
    interacting_rna_id: HashSet<String>,
    urs: HashSet<String>,
    methods: HashSet<String>,
}

impl Default for InteractingRnaVec {
    fn default() -> Self {
        Self {
            interacting_rna_id: HashSet::new(),
            urs: HashSet::new(),
            methods: HashSet::new(),
        }
    }
}

impl FromIterator<InteractingRna> for InteractingRnaVec {
    fn from_iter<I: IntoIterator<Item = InteractingRna>>(iter: I) -> Self {
        let mut value = InteractingRnaVec::default();

        for i in iter {
            value.interacting_rna_id.insert(i.interacting_rna_id);
            value.urs.insert(i.urs);
            value.methods.extend(i.methods);
        }

        value
    }
}
