use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 14808227 v1.00`**
///
/// This metadata element is defined by the caDSR as "The cellular source of 
/// the double stranded DNA fragments analyzed by high-throughput sequencing.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=14808227%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::sample::LibrarySourceMaterial)]
pub enum LibrarySourceMaterial {
    /// `Bulk Cells`
    #[serde(rename = "Bulk-Cells")]
    BulkCells,

    /// `Bulk-Nuclei`
    #[serde(rename = "Bulk-Nuclei")]
    BulkNuclei,

    /// `Bulk-Tissue`
    #[serde(rename = "Bulk-Tissue")]
    BulkTissue,

    /// `Single-cells`
    #[serde(rename = "Single-cells")]
    SingleCells,

    /// `Single-nuclei`
    #[serde(rename = "Single-nuclei")]
    SingleNuclei,

    /// `Not-Reported`
    #[serde(rename = "Not-Reported")]
    NotReported,

    /// `Other`
    #[serde(rename = "Other")]
    Other,

}

impl CDE for LibrarySourceMaterial {}

impl std::fmt::Display for LibrarySourceMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibrarySourceMaterial::BulkCells => write!(f, "Bulk Cells"),
            LibrarySourceMaterial::BulkNuclei => write!(f, "Bulk Nuclei"),
            LibrarySourceMaterial::BulkTissue => write!(f, "Bulk Tissue"),
            LibrarySourceMaterial::SingleCells => write!(f, "Single-cells"),
            LibrarySourceMaterial::SingleNuclei => write!(f, "Single-nuclei"),
            LibrarySourceMaterial::NotReported => write!(f, "Not Reported"),
            LibrarySourceMaterial::Other => write!(f, "Other"),
        }
    }
}

impl Distribution<LibrarySourceMaterial> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> LibrarySourceMaterial {
        match rng.gen_range(0..6) {
            0 => LibrarySourceMaterial::BulkCells,
            1 => LibrarySourceMaterial::BulkNuclei,
            2 => LibrarySourceMaterial::BulkTissue,
            3 => LibrarySourceMaterial::SingleCells,
            4 => LibrarySourceMaterial::SingleNuclei,
            5 => LibrarySourceMaterial::NotReported,
            6 => LibrarySourceMaterial::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(LibrarySourceMaterial::BulkCells.to_string(), "Bulk Cells");
        assert_eq!(LibrarySourceMaterial::BulkNuclei.to_string(), "Bulk Nuclei");
        assert_eq!(LibrarySourceMaterial::BulkTissue.to_string(), "Bulk Tissue");
        assert_eq!(LibrarySourceMaterial::SingleCells.to_string(), "Single-cells");
        assert_eq!(LibrarySourceMaterial::SingleNuclei.to_string(), "Single-nuclei");
        assert_eq!(LibrarySourceMaterial::NotReported.to_string(), "Not Reported");
        assert_eq!(LibrarySourceMaterial::Other.to_string(), "Other");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&LibrarySourceMaterial::BulkCells).unwrap(),
            "\"Bulk Cells\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySourceMaterial::BulkNuclei).unwrap(),
            "\"Bulk Nuclei\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySourceMaterial::BulkTissue).unwrap(),
            "\"Bulk Tissue\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySourceMaterial::SingleCells).unwrap(),
            "\"Single-cells\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySourceMaterial::SingleNuclei).unwrap(),
            "\"Single-nuclei\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySourceMaterial::NotReported).unwrap(),
            "\"Not Reported\""
        );
        assert_eq!(
            serde_json::to_string(&LibrarySourceMaterial::Other).unwrap(),
            "\"Other\""
        );
    }
}