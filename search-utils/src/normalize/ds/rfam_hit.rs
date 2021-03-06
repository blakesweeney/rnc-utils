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
    rfam_ids: String,
    rfam_family_names: String,
    rfam_clans: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RfamHitVec {
    rfam_ids: HashSet<String>,
    rfam_family_names: HashSet<String>,
    rfam_clans: HashSet<String>,
}

impl Default for RfamHitVec {
    fn default() -> Self {
        Self {
            rfam_ids: HashSet::new(),
            rfam_family_names: HashSet::new(),
            rfam_clans: HashSet::new(),
        }
    }
}

impl FromIterator<RfamHit> for RfamHitVec {
    fn from_iter<I: IntoIterator<Item = RfamHit>>(iter: I) -> Self {
        let mut value = RfamHitVec::default();

        for i in iter {
            value.rfam_ids.insert(i.rfam_ids);
            value.rfam_family_names.insert(i.rfam_family_names);
            value.rfam_clans.extend(i.rfam_clans);
        }

        value
    }
}
