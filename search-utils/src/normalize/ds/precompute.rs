use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Precompute {
    description: String,
    rna_type: String,
    has_coordinates: bool,
    so_rna_type: String,
    databases: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecomputeSummary {
    description: String,
    rna_type: String,
    has_coordinates: String,
    databases: Vec<String>,
}

impl Precompute {
    pub fn so_rna_type(&self) -> &str {
        &self.so_rna_type
    }
}

impl From<Precompute> for PrecomputeSummary {
    fn from(pre: Precompute) -> PrecomputeSummary {
        let has_coordinates = match pre.has_coordinates {
            true => String::from("True"),
            false => String::from("False"),
        };

        Self {
            description: pre.description,
            rna_type: pre.rna_type,
            has_coordinates,
            databases: pre.databases,
        }
    }
}
