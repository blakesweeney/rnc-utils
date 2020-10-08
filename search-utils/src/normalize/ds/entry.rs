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
    basic::Basic,
    cross_reference::{
        AccessionVec,
        CrossReference,
    },
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

/*
{
  "interacting_proteins": [],
  "base": [
    {
      "id": "URS0000614226_291828",
      "length": 181,
      "md5": "1b40575dabf9994947faba61876fc1a6",
      "urs": "URS0000614226"
    }
  ],
  "id": "URS0000614226_291828",
  "crs": [],
  "cross_references": [],
  "qa_status": [
    {
      "has_issue": true,
      "id": "URS0000614226_291828",
      "incomplete_sequence": true,
      "missing_rfam_match": false,
      "possible_contamination": false
    }
  ],
  "go_annotations": [],
  "feedback": [],
  "interacting_rnas": [],
  "references": [],
  "precompute": [
    {
      "databases": "ENA",
      "description": "uncultured Parvibaculum sp. partial 16S ribosomal RNA",
      "has_coordinates": false,
      "id": "URS0000614226_291828",
      "rna_type": "rRNA",
      "so_rna_type": "SO:0000650"
    }
  ],
  "r2dt": [],
  "rfam_hits": [
    {
      "id": "URS0000614226_291828",
      "rfam_clans": "CL00111",
      "rfam_family_names": "SSU_rRNA_bacteria",
      "rfam_ids": "RF00177",
      "urs": "URS0000614226"
    }
  ]
}
*/

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
    secondary_structure: Option<R2dt>,

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
        let so_rna_type_tree = so_info[precompute.so_rna_type()].clone();
        let pre_summary = PrecomputeSummary::from(precompute);
        let secondary_structure = utils::maybe_single(&raw.r2dt, "r2dt")?;

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

            accessions: AccessionVec::from_iter(raw.cross_references.clone()),
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
