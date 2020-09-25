use std::convert::TryFrom;

use thiserror::Error;

use crate::publications::reference_type;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternalReference(reference_type::ReferenceType, String);

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("No prefix for the reference id: `{0}`")]
    MissingPrefix(String),

    #[error("Unknown type of reference: {0}")]
    RefTypeError(#[from] reference_type::ConversionError),

    #[error("Format of reference `{0}` is invalid")]
    InvalidFormat(String),
}

impl ExternalReference {
    pub fn new(ref_type: reference_type::ReferenceType, ref_id: String) -> Self {
        Self(ref_type, ref_id)
    }

    pub fn ref_type(&self) -> reference_type::ReferenceType {
        self.0
    }

    pub fn ref_id(&self) -> &String {
        &self.1
    }
}

impl TryFrom<&String> for ExternalReference {
    type Error = ConversionError;

    fn try_from(raw: &String) -> Result<ExternalReference, Self::Error> {
        let parts: Vec<&str> = raw.split(":").collect();
        if parts.len() == 1 {
            return Err(Self::Error::MissingPrefix(raw.to_string()));
        }

        if parts.len() > 2 {
            return Err(Self::Error::InvalidFormat(raw.to_string()));
        }

        let ref_type = reference_type::ReferenceType::try_from(parts[0])?;
        Ok(ExternalReference(ref_type, parts[1].to_string()))
    }
}

impl<'a> From<ExternalReference> for String {
    fn from(raw: ExternalReference) -> String {
        format!("{}:{}", raw.0, raw.1)
    }
}
