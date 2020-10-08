use std::{
    collections::HashSet,
    iter::FromIterator,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CrossReference {
    accession: String,
    common_name: Option<String>,
    external_id: String,
    functions: Option<String>,
    gene_synonyms: Option<String>,
    genes: Option<String>,
    locus_tags: Option<String>,
    name: String,
    non_coding_id: String,
    notes: Option<String>,
    optional_id: String,
    organelles: Option<String>,
    parent_accession: String,
    products: Option<String>,
    species: String,
    standard_names: Option<String>,
    tax_strings: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessionVec {
    species: HashSet<String>,
    organelles: HashSet<String>,
    tax_strings: HashSet<String>,
    functions: HashSet<String>,
    genes: HashSet<String>,
    gene_synonyms: HashSet<String>,
    common_name: HashSet<String>,
    notes: HashSet<String>,
    locus_tags: HashSet<String>,
    standard_names: HashSet<String>,
    products: HashSet<String>,
}

impl Default for AccessionVec {
    fn default() -> Self {
        Self {
            species: HashSet::new(),
            organelles: HashSet::new(),
            tax_strings: HashSet::new(),
            functions: HashSet::new(),
            genes: HashSet::new(),
            gene_synonyms: HashSet::new(),
            common_name: HashSet::new(),
            notes: HashSet::new(),
            locus_tags: HashSet::new(),
            standard_names: HashSet::new(),
            products: HashSet::new(),
        }
    }
}

impl FromIterator<CrossReference> for AccessionVec {
    fn from_iter<I: IntoIterator<Item = CrossReference>>(iter: I) -> Self {
        let mut a = AccessionVec::default();

        for i in iter {
            if !i.species.is_empty() {
                a.species.insert(i.species);
            }
            if !i.tax_strings.is_empty() {
                a.tax_strings.insert(i.tax_strings);
            }

            a.organelles.extend(i.organelles);

            if i.functions.is_some() {
                a.functions.insert(i.functions.unwrap());
            }
            if i.genes.is_some() {
                a.genes.insert(i.genes.unwrap());
            }

            if i.gene_synonyms.is_some() {
                let synonyms = i.gene_synonyms.unwrap();
                if synonyms.contains(";") {
                    a.gene_synonyms
                        .extend(synonyms.split(";").map(|p| p.trim()).map(str::to_string))
                } else if synonyms.contains(",") {
                    a.gene_synonyms
                        .extend(synonyms.split(",").map(|p| p.trim()).map(str::to_string))
                } else {
                    a.gene_synonyms.insert(synonyms);
                }
            }

            if i.common_name.is_some() {
                a.common_name.insert(i.common_name.unwrap());
            }
            if i.notes.is_some() {
                a.notes.insert(i.notes.unwrap());
            }
            if i.locus_tags.is_some() {
                a.locus_tags.insert(i.locus_tags.unwrap());
            }
            if i.standard_names.is_some() {
                a.standard_names.insert(i.standard_names.unwrap());
            }
            if i.products.is_some() {
                a.products.insert(i.products.unwrap());
            }
        }

        a
    }
}
