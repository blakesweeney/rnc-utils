use std::{
    collections::HashSet,
    iter::FromIterator,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfamHit {
    rfam_id: String,
    rfam_family_name: String,
    rfam_clan: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfamHitVec {
    rfam_id: HashSet<String>,
    rfam_family_name: HashSet<String>,
    rfam_clan: HashSet<String>,
}

impl Default for RfamHitVec {
    fn default() -> Self {
        Self {
            rfam_id: HashSet::new(),
            rfam_family_name: HashSet::new(),
            rfam_clan: HashSet::new(),
        }
    }
}

impl FromIterator<RfamHit> for RfamHitVec {
    fn from_iter<I: IntoIterator<Item = RfamHit>>(iter: I) -> Self {
        let mut value = RfamHitVec::default();

        for i in iter {
            value.rfam_id.insert(i.rfam_id);
            value.rfam_family_name.insert(i.rfam_family_name);
            value.rfam_id.extend(i.rfam_clan);
        }

        value
    }
}
