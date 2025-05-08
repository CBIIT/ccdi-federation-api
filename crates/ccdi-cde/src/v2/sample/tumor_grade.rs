use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11325685 v2.00`**
///
/// This metadata element is defined by the caDSR as "A text term to express
/// the degree of abnormality of cancer cells as a measure of differentiation
/// and aggressiveness.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11325685%20and%20ver_nr=2>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v2::sample::TumorGrade)]
pub enum TumorGrade {
    /// `G1 Low Grade`
    ///
    /// * **VM Long Name**: Grade 1
    /// * **VM Public ID**: 13270009
    /// * **Concept Code**: C28077
    /// * **Begin Date**:   03/03/2025
    ///
    /// A morphologic qualifier indicating that a cancerous lesion is
    /// well differentiated.
    #[serde(rename = "G1 Low Grade")]
    G1LowGrade,

    /// `G2 Intermediate Grade`
    ///
    /// * **VM Long Name**: Grade 2
    /// * **VM Public ID**: 5622672
    /// * **Concept Code**: C28078
    /// * **Begin Date**:   03/03/2025
    ///
    /// A morphologic qualifier indicating that a cancerous lesion is
    /// moderately differentiated.
    #[serde(rename = "G2 Intermediate Grade")]
    G2IntermediateGrade,

    /// `G3 High Grade`
    ///
    /// * **VM Long Name**: Grade 3
    /// * **VM Public ID**: 5622671
    /// * **Concept Code**: C28079
    /// * **Begin Date**:   03/03/2025
    ///
    /// A morphologic qualifier indicating that a cancerous lesion is
    /// poorly differentiated.
    #[serde(rename = "G3 High Grade")]
    G3HighGrade,

    /// `G4 Anaplastic`
    ///
    /// * **VM Long Name**: Grade 4
    /// * **VM Public ID**: 15872601
    /// * **Concept Code**: C28082
    /// * **Begin Date**:   03/03/2025
    ///
    /// A morphologic qualifier indicating that a cancerous lesion is
    /// undifferentiated.
    #[serde(rename = "G4 Anaplastic")]
    G4Anaplastic,

    /// `GB Borderline`
    ///
    /// * **VM Long Name**: Grade B
    /// * **VM Public ID**: 6050322
    /// * **Concept Code**: C113729
    /// * **Begin Date**:   09/27/2022
    ///
    /// A morphologic qualifier indicating that a neoplasm is of
    /// borderline malignancy.
    #[serde(rename = "GB Borderline")]
    GBBorderline,

    /// `GX Grade Cannot Be Assessed`
    ///
    /// * **VM Long Name**: Grade X
    /// * **VM Public ID**: 10987209
    /// * **Concept Code**: C113730
    /// * **Begin Date**:   09/27/2022
    ///
    /// A morphologic qualifier indicating that the grade of a
    /// neoplasm cannot be assessed.
    #[serde(rename = "GX Grade Cannot Be Assessed")]
    GXGrade,

    /// `Not Applicable`
    ///
    /// * **VM Long Name**: Not Applicable
    /// * **VM Public ID**: 7590024
    /// * **Concept Code**: C48660
    /// * **Begin Date**:   09/27/2022
    ///
    /// Determination of a value is not relevant in the current
    /// context.
    #[serde(rename = "Not Applicable")]
    NotApplicable,

    /// `Not Reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 5612322
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   03/27/2023
    ///
    /// Not provided or available.
    #[serde(rename = "Not Reported")]
    NotReported,

    /// `Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 5682953
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   07/30/2023
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,
}

impl CDE for TumorGrade {}

impl std::fmt::Display for TumorGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TumorGrade::G1LowGrade => write!(f, "G1 Low Grade"),
            TumorGrade::G2IntermediateGrade => write!(f, "G2 Intermediate Grade"),
            TumorGrade::G3HighGrade => write!(f, "G3 High Grade"),
            TumorGrade::G4Anaplastic => write!(f, "G4 Anaplastic"),
            TumorGrade::GBBorderline => write!(f, "GB Borderline"),
            TumorGrade::GXGrade => write!(f, "GX Grade Cannot Be Assessed"),
            TumorGrade::NotApplicable => write!(f, "Not Applicable"),
            TumorGrade::NotReported => write!(f, "Not Reported"),
            TumorGrade::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Distribution<TumorGrade> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> TumorGrade {
        match rng.gen_range(0..9) {
            0 => TumorGrade::G1LowGrade,
            1 => TumorGrade::G2IntermediateGrade,
            2 => TumorGrade::G3HighGrade,
            3 => TumorGrade::G4Anaplastic,
            4 => TumorGrade::GBBorderline,
            5 => TumorGrade::GXGrade,
            6 => TumorGrade::NotApplicable,
            7 => TumorGrade::NotReported,
            _ => TumorGrade::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(TumorGrade::G1LowGrade.to_string(), "G1 Low Grade");
        assert_eq!(
            TumorGrade::G2IntermediateGrade.to_string(),
            "G2 Intermediate Grade"
        );
        assert_eq!(TumorGrade::G3HighGrade.to_string(), "G3 High Grade");
        assert_eq!(TumorGrade::G4Anaplastic.to_string(), "G4 Anaplastic");
        assert_eq!(TumorGrade::GBBorderline.to_string(), "GB Borderline");
        assert_eq!(
            TumorGrade::GXGrade.to_string(),
            "GX Grade Cannot Be Assessed"
        );
        assert_eq!(TumorGrade::NotApplicable.to_string(), "Not Applicable");
        assert_eq!(TumorGrade::NotReported.to_string(), "Not Reported");
        assert_eq!(TumorGrade::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&TumorGrade::G1LowGrade).unwrap(),
            "\"G1 Low Grade\""
        );
        assert_eq!(
            serde_json::to_string(&TumorGrade::G2IntermediateGrade).unwrap(),
            "\"G2 Intermediate Grade\""
        );
        assert_eq!(
            serde_json::to_string(&TumorGrade::G3HighGrade).unwrap(),
            "\"G3 High Grade\""
        );
        assert_eq!(
            serde_json::to_string(&TumorGrade::G4Anaplastic).unwrap(),
            "\"G4 Anaplastic\""
        );
        assert_eq!(
            serde_json::to_string(&TumorGrade::GBBorderline).unwrap(),
            "\"GB Borderline\""
        );
        assert_eq!(
            serde_json::to_string(&TumorGrade::GXGrade).unwrap(),
            "\"GX Grade Cannot Be Assessed\""
        );
        assert_eq!(
            serde_json::to_string(&TumorGrade::NotApplicable).unwrap(),
            "\"Not Applicable\""
        );
        assert_eq!(
            serde_json::to_string(&TumorGrade::NotReported).unwrap(),
            "\"Not Reported\""
        );
        assert_eq!(
            serde_json::to_string(&TumorGrade::Unknown).unwrap(),
            "\"Unknown\""
        );
    }
}
