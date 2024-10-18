use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 15235975 v1.00`**
///
/// This metadata element is defined by the caDSR as "TThe cell or cellular component that makes up the sample that has been prepared for testing or research purposes.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=15235975%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::sample::LibrarySourceMaterial)]
pub enum LibrarySourceMaterial {
    /// `Bulk Cells`
    ///
    /// * **VM Long Name**: Bulk Cell Specimen
    /// * **VM Public ID**: 7592130
    /// * **Concept Code**: C178223
    /// * **Begin Date**:   03/12/2024
    ///
    /// A biospecimen consisting of multiple cells intended to be analyzed as a pool.
    #[serde(rename = "Bulk Cells")]
    BulkCells,

    /// `Bulk Nuclei`
    ///
    /// * **VM Long Name**: Bulk Nucleus Specimen
    /// * **VM Public ID**: 7592129
    /// * **Concept Code**: C178224
    /// * **Begin Date**:   02/28/2024
    ///
    /// A biospecimen consisting of multiple nuclei intended to be analyzed as a pool.
    #[serde(rename = "Bulk Nuclei")]
    BulkNuclei,

    /// `Bulk Tissue`
    ///
    /// * **VM Long Name**: Bulk Tissue Specimen
    /// * **VM Public ID**: 7592128
    /// * **Concept Code**: C178225
    /// * **Begin Date**:   02/28/2024
    ///
    /// A biospecimen either derived from a whole tissue specimen or tissue section, which may consist of heterogeneous cells or tissues.
    #[serde(rename = "Bulk Tissue")]
    BulkTissue,

    /// `Single-cells`
    ///
    /// * **VM Long Name**: Single Cell Suspension
    /// * **VM Public ID**: 14838800
    /// * **Concept Code**: C204464
    /// * **Begin Date**:   03/12/2024
    ///
    /// A dilute suspension of cells intended to be further fractionated for assays focused on single-cells.
    #[serde(rename = "Single-cells")]
    SingleCells,

    /// `Single-nuclei`
    ///
    /// * **VM Long Name**: Single Nucleus Suspension
    /// * **VM Public ID**: 14838802
    /// * **Concept Code**: C204465
    /// * **Begin Date**:   03/12/2024
    ///
    /// A dilute suspension comprised of isolated intact cell nuclei intended to be further fractionated for assays focused on single-nuclei.
    #[serde(rename = "Single-nuclei")]
    SingleNuclei,

    /// `Not Reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 5612322
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   03/01/2024
    ///
    /// Not provided or available.
    #[serde(rename = "Not Reported")]
    NotReported,
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
        }
    }
}

impl Distribution<LibrarySourceMaterial> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> LibrarySourceMaterial {
        match rng.gen_range(0..5) {
            0 => LibrarySourceMaterial::BulkCells,
            1 => LibrarySourceMaterial::BulkNuclei,
            2 => LibrarySourceMaterial::BulkTissue,
            3 => LibrarySourceMaterial::SingleCells,
            4 => LibrarySourceMaterial::SingleNuclei,
            _ => LibrarySourceMaterial::NotReported,
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
        assert_eq!(
            LibrarySourceMaterial::SingleCells.to_string(),
            "Single-cells"
        );
        assert_eq!(
            LibrarySourceMaterial::SingleNuclei.to_string(),
            "Single-nuclei"
        );
        assert_eq!(
            LibrarySourceMaterial::NotReported.to_string(),
            "Not Reported"
        );
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
    }
}
