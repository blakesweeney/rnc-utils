use std::{
    collections::HashSet,
    iter::FromIterator,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Accession {
    species: String,
    organelles: Option<String>,
    product: Option<String>,
    tax_strings: String,
    functions: Option<String>,
    genes: Option<String>,
    gene_synonyms: Option<String>,
    common_name: Option<String>,
    notes: Option<String>,
    locus_tags: Option<String>,
    standard_names: Option<String>,
    products: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessionVec {
    species: HashSet<String>,
    organelles: HashSet<String>,
    product: HashSet<String>,
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
            product: HashSet::new(),
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

impl FromIterator<Accession> for AccessionVec {
    fn from_iter<I: IntoIterator<Item = Accession>>(iter: I) -> Self {
        let mut a = AccessionVec::default();

        for i in iter {
            if !i.species.is_empty() {
                a.species.insert(i.species);
            }
            if !i.tax_strings.is_empty() {
                a.tax_strings.insert(i.tax_strings);
            }

            a.organelles.extend(i.organelles);

            if i.product.is_some() {
                a.product.insert(i.product.unwrap());
            }

            if i.functions.is_some() {
                a.functions.insert(i.functions.unwrap());
            }
            if i.genes.is_some() {
                a.genes.insert(i.genes.unwrap());
            }
            if i.gene_synonyms.is_some() {
                a.gene_synonyms.insert(i.gene_synonyms.unwrap());
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
