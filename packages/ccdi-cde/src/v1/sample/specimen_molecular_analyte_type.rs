use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 15063661 v1.00`**
///
/// This metadata element is defined by the caDSR as "The sample or material
/// being subjected to analysis.". This data element is a subset of the
/// designated CDE as noted
/// [here](https://github.com/CBIIT/ccdi-federation-api/discussions/116#discussioncomment-10848175).
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=15063661%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::sample::SpecimenMolecularAnalyteType)]
pub enum SpecimenMolecularAnalyteType {
    /// `Protein`
    ///
    /// * **VM Long Name**: Protein
    /// * **VM Public ID**: 2581951
    /// * **Concept Code**: C17021
    /// * **Begin Date**:   11/15/2006
    ///
    /// A group of complex organic macromolecules composed of one or more chains
    /// (linear polymers) of alpha-L-amino acids linked by peptide bonds and
    /// ranging in size from a few thousand to over 1 million Daltons. Proteins
    /// are fundamental genetically encoded components of living cells with
    /// specific structures and functions dictated by amino acid sequence.
    #[serde(rename = "Protein")]
    Protein,

    /// `DNA`
    ///
    /// * **VM Long Name**: DNA
    /// * **VM Public ID**: 2581946
    /// * **Concept Code**: C449
    /// * **Begin Date**:   12/15/2015
    ///
    /// A long linear double-stranded polymer formed from nucleotides attached
    /// to a deoxyribose backbone and found in the nucleus of a cell; associated
    /// with the transmission of genetic information.
    #[serde(rename = "DNA")]
    Dna,

    /// `RNA`
    ///
    /// * **VM Long Name**: RNA Specimen
    /// * **VM Public ID**: 14239169
    /// * **Concept Code**: C198568
    /// * **Begin Date**:   08/01/2023
    ///
    /// A biospecimen created to contain an isolated or enriched RNA sample.
    #[serde(rename = "RNA")]
    Rna,
}

impl CDE for SpecimenMolecularAnalyteType {}

impl std::fmt::Display for SpecimenMolecularAnalyteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecimenMolecularAnalyteType::Protein => write!(f, "Protein"),
            SpecimenMolecularAnalyteType::Dna => write!(f, "DNA"),
            SpecimenMolecularAnalyteType::Rna => write!(f, "RNA"),
        }
    }
}

impl Distribution<SpecimenMolecularAnalyteType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SpecimenMolecularAnalyteType {
        match rng.gen_range(0..=2) {
            0 => SpecimenMolecularAnalyteType::Protein,
            1 => SpecimenMolecularAnalyteType::Dna,
            _ => SpecimenMolecularAnalyteType::Rna,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(SpecimenMolecularAnalyteType::Protein.to_string(), "Protein");
        assert_eq!(SpecimenMolecularAnalyteType::Dna.to_string(), "DNA");
        assert_eq!(SpecimenMolecularAnalyteType::Rna.to_string(), "RNA");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&SpecimenMolecularAnalyteType::Protein).unwrap(),
            "\"Protein\""
        );
        assert_eq!(
            serde_json::to_string(&SpecimenMolecularAnalyteType::Dna).unwrap(),
            "\"DNA\""
        );
        assert_eq!(
            serde_json::to_string(&SpecimenMolecularAnalyteType::Rna).unwrap(),
            "\"RNA\""
        );
    }
}
