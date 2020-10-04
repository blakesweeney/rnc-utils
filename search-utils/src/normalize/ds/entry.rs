use std::{
    collections::HashMap,
    iter::FromIterator,
};

use serde::{
    Deserialize,
    Serialize,
};

use rnc_core::{
    urs::Urs,
    urs_taxid::UrsTaxid,
};

use crate::normalize::utils;

use crate::normalize::ds::{
    accession::{
        Accession,
        AccessionVec,
    },
    basic::Basic,
    cross_reference::CrossReference,
    crs::{
        Crs,
        CrsVec,
    },
    feedback::{
        Feedback,
        FeedbackVec,
    },
    go_annotation::{
        GoAnnotation,
        GoAnnotationVec,
    },
    interacting_protein::{
        InteractingProtein,
        InteractingProteinVec,
    },
    interacting_rna::{
        InteractingRna,
        InteractingRnaVec,
    },
    precompute::{
        Precompute,
        PrecomputeSummary,
    },
    qa_status::QaStatus,
    r2dt::R2dt,
    reference::{
        Reference,
        ReferenceVec,
    },
    rfam_hit::{
        RfamHit,
        RfamHitVec,
    },
    so_tree,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Raw {
    id: String,
    accessions: Vec<Accession>,
    base: Vec<Basic>,
    cross_references: Vec<CrossReference>,
    crs: Vec<Crs>,
    feedback: Vec<Feedback>,
    go_annotations: Vec<GoAnnotation>,
    interacting_proteins: Vec<InteractingProtein>,
    interacting_rnas: Vec<InteractingRna>,
    precompute: Vec<Precompute>,
    qa_status: Vec<QaStatus>,
    r2dt: Vec<R2dt>,
    references: Vec<Reference>,
    rfam_hits: Vec<RfamHit>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Normalized {
    urs: String,
    taxid: u64,
    urs_taxid: String,
    short_urs: String,
    deleted: String,

    so_rna_type_tree: so_tree::SoTree,

    #[serde(flatten)]
    pre_summary: PrecomputeSummary,

    #[serde(flatten)]
    basic: Basic,

    // #[serde(flatten)]
    // dates: Dates,
    qa_status: QaStatus,
    secondary_structure: R2dt,

    accessions: AccessionVec,
    crs: CrsVec,
    feedback: FeedbackVec,
    go_annotations: GoAnnotationVec,
    interacting_proteins: InteractingProteinVec,
    interacting_rnas: InteractingRnaVec,
    references: ReferenceVec,
    rfam_hits: RfamHitVec,
}

impl Raw {
    pub fn urs(&self) -> anyhow::Result<String> {
        let ut: UrsTaxid = self.id.parse()?;
        let urs: Urs = ut.into();
        Ok(urs.to_string())
    }

    pub fn taxid(&self) -> anyhow::Result<u64> {
        let ut: UrsTaxid = self.id.parse()?;
        Ok(ut.taxid())
    }

    pub fn short_urs(&self) -> anyhow::Result<String> {
        let ut: UrsTaxid = self.id.parse()?;
        let urs: Urs = ut.into();
        Ok(urs.short_urs())
    }
}

impl Normalized {
    pub fn new(raw: &Raw, so_info: &HashMap<String, so_tree::SoTree>) -> anyhow::Result<Self> {
        let basic = utils::expect_single(&raw.base, "base")?;
        let precompute = utils::expect_single(&raw.precompute, "precompute")?;
        let qa_status = utils::expect_single(&raw.qa_status, "qa_status")?;
        let secondary_structure = utils::expect_single(&raw.r2dt, "r2dt")?;
        let so_rna_type_tree = so_info[precompute.so_rna_type()].clone();
        let pre_summary = PrecomputeSummary::from(precompute);

        Ok(Self {
            urs_taxid: raw.id.clone(),
            urs: raw.urs()?,
            taxid: raw.taxid()?,
            short_urs: raw.short_urs()?,
            deleted: String::from("N"),

            so_rna_type_tree,

            pre_summary,
            basic,
            qa_status,
            secondary_structure,

            accessions: AccessionVec::from_iter(raw.accessions.clone()),
            crs: CrsVec::from_iter(raw.crs.clone()),
            feedback: FeedbackVec::from_iter(raw.feedback.clone()),
            go_annotations: GoAnnotationVec::from_iter(raw.go_annotations.clone()),
            interacting_proteins: InteractingProteinVec::from_iter(
                raw.interacting_proteins.clone(),
            ),
            interacting_rnas: InteractingRnaVec::from_iter(raw.interacting_rnas.clone()),
            references: ReferenceVec::from_iter(raw.references.clone()),
            rfam_hits: RfamHitVec::from_iter(raw.rfam_hits.clone()),
        })
    }
}
