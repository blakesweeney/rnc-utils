use std::convert::TryFrom;

use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Error, Debug)]
pub enum ReferenceConversionError {
    #[error("No prefix for the reference id")]
    MissingPrefix,

    #[error("The prefix `{0}` is not known")]
    UnknownPrefix(String),

    #[error("Format of reference `{0}` is invalid")]
    InvalidFormat(String),
}

#[derive(Error, Debug)]
pub enum ReferenceBuildError {
    #[error("Reference must have a title")]
    NoTitle,
}

#[derive(Error, Debug)]
pub enum AuthorBuildingError {
    #[error("Author must have a name")]
    NoName,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Author(String, String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Reference {
    title: String,
    authors: Vec<Author>,
    journal: String,
    year: String,
    pmid: Option<String>,
    pmcid: Option<String>,
    doi: Option<String>,
}

pub struct AuthorBuilder {
    first: Option<String>,
    last: Option<String>,
}

pub struct ReferenceBuilder {
    title: Option<String>,
    authors: Vec<Author>,
    journal: Option<String>,
    year: Option<String>,
    pmid: Option<String>,
    pmcid: Option<String>,
    doi: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ReferenceId {
    Pmid(String),
    Doi(String),
    Pmcid(String),
}

impl Reference {
    pub fn builder() -> ReferenceBuilder {
        return ReferenceBuilder::new();
    }
}

impl AuthorBuilder {
    pub fn new() -> Self {
        return Self { last: None, first: None };
    }

    pub fn set_last_name(&mut self, last: String) {
        self.last = Some(last);
    }

    pub fn set_first_name(&mut self, first: String) {
        self.first = Some(first);
    }

    pub fn build(self) -> Result<Author, AuthorBuildingError> {
        if self.first.is_none() && self.last.is_none() {
            return Err(AuthorBuildingError::NoName);
        }

        Ok(Author(
            self.first.unwrap_or("".to_string()),
            self.last.unwrap_or("".to_string()),
        ))
    }
}

impl ReferenceBuilder {
    pub fn new() -> Self {
        return Self {
            title: None,
            authors: Vec::new(),
            journal: None,
            year: None,
            pmid: None,
            pmcid: None,
            doi: None,
        };
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn add_author(&mut self, author: Author) {
        self.authors.push(author);
    }

    pub fn set_doi(&mut self, doi: String) {
        self.doi = Some(doi);
    }

    pub fn set_pmid(&mut self, pmid: String) {
        self.pmid = Some(pmid);
    }

    pub fn set_pmcid(&mut self, pmcid: String) {
        self.pmcid = Some(pmcid);
    }

    pub fn set_year(&mut self, year: String) {
        self.year = Some(year);
    }

    pub fn set_journal(&mut self, journal: String) {
        self.journal = Some(journal);
    }

    pub fn build(self) -> Result<Reference, ReferenceBuildError> {
        let title = self.title.ok_or_else(|| ReferenceBuildError::NoTitle)?;
        return Ok(Reference {
            title,
            authors: self.authors,
            journal: self.journal.unwrap(),
            year: self.year.unwrap(),
            pmid: self.pmid,
            pmcid: self.pmcid,
            doi: self.doi,
        });
    }
}

impl TryFrom<String> for ReferenceId {
    type Error = ReferenceConversionError;

    fn try_from(raw: String) -> Result<ReferenceId, Self::Error> {
        let parts: Vec<&str> = raw.split(":").collect();
        if parts.len() == 1 {
            return Err(Self::Error::MissingPrefix);
        }

        if parts.len() > 2 {
            return Err(Self::Error::InvalidFormat(raw));
        }

        match parts[0] {
            "pmid" => Ok(Self::Pmid(parts[1].to_string())),
            "doi" => Ok(Self::Doi(parts[1].to_string())),
            "pmcid" => Ok(Self::Pmcid(parts[1].to_string())),
            _ => Err(Self::Error::UnknownPrefix(parts[0].to_string())),
        }
    }
}

impl From<ReferenceId> for String {
    fn from(raw: ReferenceId) -> String {
        match raw {
            ReferenceId::Pmid(id) => format!("pmid:{}", id),
            ReferenceId::Doi(id) => format!("doi:{}", id),
            ReferenceId::Pmcid(id) => format!("pmcid:{}", id),
        }
    }
}
