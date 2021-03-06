use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QaStatus {
    has_issue: bool,
    possible_contamination: bool,
    incomplete_sequence: bool,
    missing_rfam_match: bool,
}
