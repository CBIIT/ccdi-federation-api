use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 12922545 v1.00`**
///
/// This metadata element is defined by the caDSR as "The classification of a
/// tumor based primarily on histopathological characteristics.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12922545%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::sample::TumorClassification)]
pub enum TumorClassification {
    /// `Metastatic`
    ///
    /// * **VM Long Name**: Metastatic
    /// * **VM Public ID**: 5189148
    /// * **Concept Code**: C14174
    /// * **Begin Date**:   02/23/2023
    ///
    /// A term referring to the clinical or pathologic observation of a tumor
    /// extension from its original site of growth to another anatomic site.
    #[serde(rename = "Metastatic")]
    Metastatic,

    /// `Not Reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 5612322
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   02/23/2023
    ///
    /// Not provided or available.
    #[serde(rename = "Not Reported")]
    NotReported,

    /// `Primary`
    ///
    /// * **VM Long Name**: Primary tumor
    /// * **VM Public ID**: 5189150
    /// * **Concept Code**: C8509
    /// * **Begin Date**:   02/23/2023
    ///
    /// A tumor at the original site of origin.
    #[serde(rename = "Primary")]
    Primary,

    /// `Regional`
    ///
    /// * **VM Long Name**: Regional Disease
    /// * **VM Public ID**: 2971661
    /// * **Concept Code**: C41844
    /// * **Begin Date**:   02/23/2023
    ///
    /// A disease or condition that extends beyond the site and spreads into
    /// adjacent tissues and regional lymph nodes.
    #[serde(rename = "Regional")]
    Regional,

    /// `Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 4266671
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   02/23/2023
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,
}

impl CDE for TumorClassification {}

impl std::fmt::Display for TumorClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TumorClassification::Metastatic => write!(f, "Metastatic"),
            TumorClassification::NotReported => write!(f, "Not Reported"),
            TumorClassification::Primary => write!(f, "Primary"),
            TumorClassification::Regional => write!(f, "Regional"),
            TumorClassification::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Distribution<TumorClassification> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> TumorClassification {
        match rng.gen_range(0..=4) {
            0 => TumorClassification::Metastatic,
            1 => TumorClassification::NotReported,
            2 => TumorClassification::Primary,
            3 => TumorClassification::Regional,
            _ => TumorClassification::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(TumorClassification::Metastatic.to_string(), "Metastatic");
        assert_eq!(TumorClassification::NotReported.to_string(), "Not Reported");
        assert_eq!(TumorClassification::Primary.to_string(), "Primary");
        assert_eq!(TumorClassification::Regional.to_string(), "Regional");
        assert_eq!(TumorClassification::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&TumorClassification::Metastatic).unwrap(),
            "\"Metastatic\""
        );
        assert_eq!(
            serde_json::to_string(&TumorClassification::NotReported).unwrap(),
            "\"Not Reported\""
        );
        assert_eq!(
            serde_json::to_string(&TumorClassification::Primary).unwrap(),
            "\"Primary\""
        );
        assert_eq!(
            serde_json::to_string(&TumorClassification::Regional).unwrap(),
            "\"Regional\""
        );
        assert_eq!(
            serde_json::to_string(&TumorClassification::Unknown).unwrap(),
            "\"Unknown\""
        );
    }
}
