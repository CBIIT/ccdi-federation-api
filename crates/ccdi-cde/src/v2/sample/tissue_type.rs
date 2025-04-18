use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 14688604 v1.00`**
///
/// This metadata element is defined by the caDSR as "The category assigned to the
/// cytologic atypia found in cellular molecules, cells, tissues, organs,
/// body fluids, or body excretory products."
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=14688604%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v2::sample::TissueType)]
pub enum TissueType {
    /// `Not Reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 5612322
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   01/18/2024
    ///
    /// Not provided or available.
    #[serde(rename = "Not Reported")]
    NotReported,

    /// `Normal`
    ///
    /// * **VM Long Name**: Normal Tissue Sample
    /// * **VM Public ID**: 14741231
    /// * **Concept Code**: C162623
    /// * **Begin Date**:   01/31/2024
    ///
    /// Tissue sample with cellular composition and architectural patterns expected
    /// for the particular anatomic site in which it belongs. There is no evidence
    /// of abnormal cellular infiltrates or tumor mass formation.
    #[serde(rename = "Normal")]
    Normal,

    /// `Peritumoral`
    ///
    /// * **VM Long Name**: Tumor-Adjacent Normal Specimen
    /// * **VM Public ID**: 13332906
    /// * **Concept Code**: C164032
    /// * **Begin Date**:   01/18/2024
    ///
    /// A specimen comprised of morphologically normal tissue collected from the
    /// area immediately surrounding a tumor in an experimental subject.
    #[serde(rename = "Peritumoral")]
    Peritumoral,

    /// `Tumor`
    ///
    /// * **VM Long Name**: Tumor Tissue
    /// * **VM Public ID**: 3184945
    /// * **Concept Code**: C18009
    /// * **Begin Date**:   01/31/2024
    ///
    /// A tissue sample, or entire tumor that is removed for microscopic examination.
    #[serde(rename = "Tumor")]
    Tumor,

    /// `Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 5682953
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   05/16/2017
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,
}

impl CDE for TissueType {}

impl std::fmt::Display for TissueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TissueType::NotReported => write!(f, "Not Reported"),
            TissueType::Normal => write!(f, "Normal"),
            TissueType::Peritumoral => write!(f, "Peritumoral"),
            TissueType::Tumor => write!(f, "Tumor"),
            TissueType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Distribution<TissueType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> TissueType {
        match rng.gen_range(0..=4) {
            0 => TissueType::NotReported,
            1 => TissueType::Normal,
            2 => TissueType::Peritumoral,
            3 => TissueType::Tumor,
            _ => TissueType::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(TissueType::NotReported.to_string(), "Not Reported");
        assert_eq!(TissueType::Normal.to_string(), "Normal");
        assert_eq!(TissueType::Peritumoral.to_string(), "Peritumoral");
        assert_eq!(TissueType::Tumor.to_string(), "Tumor");
        assert_eq!(TissueType::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&TissueType::NotReported).unwrap(),
            "\"Not Reported\""
        );
        assert_eq!(
            serde_json::to_string(&TissueType::Normal).unwrap(),
            "\"Normal\""
        );
        assert_eq!(
            serde_json::to_string(&TissueType::Peritumoral).unwrap(),
            "\"Peritumoral\""
        );
        assert_eq!(
            serde_json::to_string(&TissueType::Tumor).unwrap(),
            "\"Tumor\""
        );
        assert_eq!(
            serde_json::to_string(&TissueType::Unknown).unwrap(),
            "\"Unknown\""
        );
    }
}
