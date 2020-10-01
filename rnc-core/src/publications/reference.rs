use thiserror::Error;

use md5::{Md5, Digest};

use crate::publications::external_reference::ExternalReference;
use crate::publications::reference_type;

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

#[derive(Debug, PartialEq, Eq)]
pub struct Author(String, String);

#[derive(Debug, PartialEq, Eq)]
pub struct Reference {
    title: String,
    authors: Vec<Author>,
    journal: String,
    year: String,
    external_ids: Vec<ExternalReference>,
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
    pmid: Option<ExternalReference>,
    doi: Option<ExternalReference>,
    pmcid: Option<ExternalReference>,
}

impl Reference {
    pub fn builder() -> ReferenceBuilder {
        ReferenceBuilder::new()
    }

    pub fn external_ids(&self) -> &Vec<ExternalReference> {
        &self.external_ids
    }

    pub fn location(&self) -> String {
        "".to_string()
    }

    pub fn authors(&self) -> String {
        "".to_string()
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn year(&self) -> &String {
        &self.year
    }

    pub fn md5(&self) -> String {
        let mut hasher = Md5::new();
        hasher.update(self.authors().as_bytes());
        hasher.update(self.location().as_bytes());
        hasher.update(self.title().as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl AuthorBuilder {
    pub fn new() -> Self {
        Self {
            last: None,
            first: None,
        }
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
        Self {
            title: None,
            authors: Vec::new(),
            journal: None,
            year: None,
            pmid: None,
            pmcid: None,
            doi: None,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn add_author(&mut self, author: Author) {
        self.authors.push(author);
    }

    pub fn set_doi(&mut self, doi: String) {
        self.doi = Some(ExternalReference::new(
            reference_type::ReferenceType::Doi,
            doi,
        ));
    }

    pub fn set_pmid(&mut self, pmid: String) {
        self.pmid = Some(ExternalReference::new(
            reference_type::ReferenceType::Pmid,
            pmid,
        ));
    }

    pub fn set_pmcid(&mut self, pmcid: String) {
        self.pmcid = Some(ExternalReference::new(
            reference_type::ReferenceType::Pmcid,
            pmcid,
        ));
    }

    pub fn set_year(&mut self, year: String) {
        self.year = Some(year);
    }

    pub fn set_journal(&mut self, journal: String) {
        self.journal = Some(journal);
    }

    pub fn build(self) -> Result<Reference, ReferenceBuildError> {
        let title = self.title.ok_or_else(|| ReferenceBuildError::NoTitle)?;
        let mut external_ids = Vec::with_capacity(3);
        if self.pmid.is_some() {
            external_ids.push(self.pmid.unwrap())
        }
        if self.pmcid.is_some() {
            external_ids.push(self.pmcid.unwrap())
        }
        if self.doi.is_some() {
            external_ids.push(self.doi.unwrap())
        }

        Ok(Reference {
            title,
            authors: self.authors,
            journal: self.journal.unwrap(),
            year: self.year.unwrap(),
            external_ids,
        })
    }

    pub fn clear(&mut self) {
        self.title = None;
        self.authors.clear();
        self.journal = None;
        self.year = None;
        self.pmid = None;
        self.pmcid = None;
        self.doi = None;
    }
}
