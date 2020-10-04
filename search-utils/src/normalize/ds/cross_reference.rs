use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossReference {
    name: String,
    external_id: String,
    optional_id: String,
    accession: String,
    non_coding_id: String,
    parent_accession: String,
}
