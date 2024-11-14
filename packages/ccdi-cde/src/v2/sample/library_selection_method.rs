use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 6347743 2.00`**
///
/// This metadata element is defined by the caDSR as "The type of systematic
/// actions performed to select or enrich DNA fragments used in analysis by
/// high-throughput sequencing.". This data element is a subset of the
/// designated CDE as noted
/// [here](https://github.com/CBIIT/ccdi-federation-api/discussions/47#discussioncomment-9863376).
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6347743%20and%20ver_nr=2>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v2::sample::LibrarySelectionMethod)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum LibrarySelectionMethod {
    /// `Random PCR`
    ///
    /// * **VM Long Name**: Random PCR
    /// * **VM Public ID**: 6287022
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Source material was selected by randomly generated primers
    #[serde(rename = "Random PCR")]
    RandomPCR,

    /// `PCR`
    ///
    /// * **VM Long Name**: Polymerase Chain Reaction
    /// * **VM Public ID**: 3234683
    /// * **Concept Code**: C17003
    /// * **Begin Date**:   12/01/2023
    ///
    /// A method for amplifying a DNA base sequence using multiple rounds of
    /// heat denaturation of the DNA and annealing of oligonucleotide primers
    /// complementary to flanking regions in the presence of a heat-stable
    /// polymerase.  This results in duplication of the targeted DNA region.
    /// Newly synthesized DNA strands can subsequently serve as additional
    /// templates for the same primer sequences, so that successive rounds of
    /// primer annealing, strand elongation, and dissociation produce rapid and
    /// highly specific amplification of the desired sequence. PCR also can be
    /// used to detect the existence of the defined sequence in a DNA sample.
    #[serde(rename = "PCR")]
    PCR,

    /// `Random`
    ///
    /// * **VM Long Name**: Random
    /// * **VM Public ID**: 6287023
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Random selection by shearing or other method
    #[serde(rename = "Random")]
    Random,

    /// `Hybrid Selection`
    ///
    /// * **VM Long Name**: Hybrid Selection
    /// * **VM Public ID**: 6287909
    /// * **Concept Code**:
    /// * **Begin Date**:   05/15/2018
    ///
    /// Selection by hybridization in array or solution.
    #[serde(rename = "Hybrid Selection")]
    HybridSelection,

    /// `Unspecified`
    ///
    /// * **VM Long Name**: Unspecified Sequencing Library Method
    /// * **VM Public ID**: 6287913
    /// * **Concept Code**: C71460
    /// * **Begin Date**:   05/15/2018
    ///
    /// Not stated explicitly or in detail.: A collection of double stranded DNA
    /// fragments flanked by oligonucleotide sequence adapters to enable their
    /// analysis by high-throughput sequencing.: A means, manner of procedure,
    /// or systematic course of actions that have to be performed in order to
    /// accomplish a particular goal.
    #[serde(rename = "Unspecified")]
    Unspecified,

    /// `rRNA Depletion`
    ///
    /// * **VM Long Name**: rRNA Depleted Genomic Library
    /// * **VM Public ID**: 7537069
    /// * **Concept Code**: C163990
    /// * **Begin Date**:   12/22/2020
    ///
    /// A genomic library where the sample has been subjected to a ribosomal RNA
    /// depletion step before it is amplified.
    #[serde(rename = "rRNA Depletion")]
    rRNADepletion,

    /// `Not applicable`
    ///
    /// * **VM Long Name**: Not Applicable
    /// * **VM Public ID**: 5682946
    /// * **Concept Code**: C48660
    /// * **Begin Date**:   08/30/2024
    ///
    /// Determination of a value is not relevant in the current context.
    #[serde(rename = "Not applicable")]
    NotApplicable,

    /// `Poly-A Enriched Genomic Library`
    ///
    /// * **VM Long Name**: Poly-A Enriched Genomic Library
    /// * **VM Public ID**: 14901531
    /// * **Concept Code**: C163988
    /// * **Begin Date**:   04/18/2024
    ///
    /// A genomic library where the sample has been subjected to enrichment via
    /// binding to immobilized poly-T oligonucleotides, which will
    /// preferentially isolate polyadenylated (poly-A) RNAs.
    #[serde(rename = "Poly-A Enriched Genomic Library")]
    PolyAEnrichedGenomicLibrary,
}

impl CDE for LibrarySelectionMethod {}

impl std::fmt::Display for LibrarySelectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibrarySelectionMethod::RandomPCR => write!(f, "Random PCR"),
            LibrarySelectionMethod::PCR => write!(f, "PCR"),
            LibrarySelectionMethod::Random => write!(f, "Random"),
            LibrarySelectionMethod::HybridSelection => write!(f, "Hybrid Selection"),
            LibrarySelectionMethod::Unspecified => write!(f, "Unspecified"),
            LibrarySelectionMethod::rRNADepletion => write!(f, "rRNA Depletion"),
            LibrarySelectionMethod::NotApplicable => write!(f, "Not applicable"),
            LibrarySelectionMethod::PolyAEnrichedGenomicLibrary => {
                write!(f, "Poly-A Enriched Genomic Library")
            }
        }
    }
}

impl Distribution<LibrarySelectionMethod> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> LibrarySelectionMethod {
        match rng.gen_range(0..=7) {
            0 => LibrarySelectionMethod::RandomPCR,
            1 => LibrarySelectionMethod::PCR,
            2 => LibrarySelectionMethod::Random,
            3 => LibrarySelectionMethod::HybridSelection,
            4 => LibrarySelectionMethod::Unspecified,
            5 => LibrarySelectionMethod::rRNADepletion,
            6 => LibrarySelectionMethod::NotApplicable,
            _ => LibrarySelectionMethod::PolyAEnrichedGenomicLibrary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(LibrarySelectionMethod::RandomPCR.to_string(), "Random PCR");
        assert_eq!(LibrarySelectionMethod::PCR.to_string(), "PCR");
        assert_eq!(LibrarySelectionMethod::Random.to_string(), "Random");
        assert_eq!(
            LibrarySelectionMethod::HybridSelection.to_string(),
            "Hybrid Selection"
        );
        assert_eq!(
            LibrarySelectionMethod::Unspecified.to_string(),
            "Unspecified"
        );
        assert_eq!(
            LibrarySelectionMethod::rRNADepletion.to_string(),
            "rRNA Depletion"
        );
        assert_eq!(
            LibrarySelectionMethod::NotApplicable.to_string(),
            "Not applicable"
        );
        assert_eq!(
            LibrarySelectionMethod::PolyAEnrichedGenomicLibrary.to_string(),
            "Poly-A Enriched Genomic Library"
        );
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::RandomPCR).unwrap(),
            "\"Random PCR\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::PCR).unwrap(),
            "\"PCR\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Random).unwrap(),
            "\"Random\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::HybridSelection).unwrap(),
            "\"Hybrid Selection\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::Unspecified).unwrap(),
            "\"Unspecified\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::rRNADepletion).unwrap(),
            "\"rRNA Depletion\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::NotApplicable).unwrap(),
            "\"Not applicable\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySelectionMethod::PolyAEnrichedGenomicLibrary).unwrap(),
            "\"Poly-A Enriched Genomic Library\""
        );
    }
}
