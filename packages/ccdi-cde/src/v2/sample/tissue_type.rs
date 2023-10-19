use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 5432687 v2.00`**
///
/// This metadata element is defined by the caDSR as "Text term that represents
/// a description of the kind of tissue collected with respect to disease status
/// or proximity to tumor tissue."
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=5432687%20and%20ver_nr=2>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v2::sample::TissueType)]
pub enum TissueType {
    /// `Not Reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 5612322
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   10/03/2023
    ///
    /// Not provided or available.
    #[serde(rename = "Not Reported")]
    NotReported,

    /// `Abnormal`
    ///
    /// * **VM Long Name**: Abnormal
    /// * **VM Public ID**: 4265117
    /// * **Concept Code**: C25401
    /// * **Begin Date**:   08/25/2016
    ///
    /// Deviating in any way from the state, position, structure, condition,
    /// behavior, or rule which is considered a norm.
    #[serde(rename = "Abnormal")]
    Abnormal,

    /// `Normal`
    ///
    /// * **VM Long Name**: Normal
    /// * **VM Public ID**: 4494160
    /// * **Concept Code**: C14165
    /// * **Begin Date**:   08/25/2016
    ///
    /// Being approximately average or within certain limits; conforming with or
    /// constituting a norm or standard or level or type or social norm.
    #[serde(rename = "Normal")]
    Normal,

    /// `Peritumoral`
    ///
    /// * **VM Long Name**: Peritumoral
    /// * **VM Public ID**: 4633527
    /// * **Concept Code**: C119010
    /// * **Begin Date**:   08/25/2016
    ///
    /// Located in tissues surrounding a tumor.
    #[serde(rename = "Peritumoral")]
    Peritumoral,

    /// `Tumor`
    ///
    /// * **VM Long Name**: Malignant Neoplasm
    /// * **VM Public ID**: 2749852
    /// * **Concept Code**: C9305
    /// * **Begin Date**:   /25/2016
    ///
    /// A tumor composed of atypical neoplastic, often pleomorphic cells that
    /// invade other tissues.  Malignant neoplasms usually metastasize to
    /// distant anatomic sites and may recur after excision.  The most common
    /// malignant neoplasms are carcinomas (adenocarcinomas or squamous cell
    /// carcinomas), Hodgkin's and non-Hodgkin's lymphomas, leukemias,
    /// melanomas, and sarcomas. -- 2004
    #[serde(rename = "Tumor")]
    Tumor,

    /// `Non-neoplastic`
    ///
    /// * **VM Long Name**: Non-Neoplastic
    /// * **VM Public ID**: 5828001
    /// * **Concept Code**: C25594:C45315
    /// * **Begin Date**:   05/16/2017
    ///
    /// An operation in which a term denies or inverts the meaning of another
    /// term or construction.: Exhibiting characteristics of independently
    /// proliferating cells, notably altered morphology, growth characteristics,
    /// and/or biochemical and molecular properties.
    #[serde(rename = "Non-neoplastic")]
    NonNeoplastic,

    /// `Unavailable`
    ///
    /// * **VM Long Name**: Unavailable
    /// * **VM Public ID**: 5828000
    /// * **Concept Code**: C126101
    /// * **Begin Date**:   05/16/2017
    ///
    /// The desired information is not available.
    #[serde(rename = "Unavailable")]
    Unavailable,

    /// `Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 2572577
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   05/16/2017
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,

    /// `Unspecified`
    ///
    /// * **VM Long Name**: Unspecified
    /// * **VM Public ID**: 2573360
    /// * **Concept Code**: C38046
    /// * **Begin Date**:   05/16/2017
    ///
    /// Not stated explicitly or in detail.
    #[serde(rename = "Unspecified")]
    Unspecified,
}

impl CDE for TissueType {}

impl std::fmt::Display for TissueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TissueType::NotReported => write!(f, "Not Reported"),
            TissueType::Abnormal => write!(f, "Abnormal"),
            TissueType::Normal => write!(f, "Normal"),
            TissueType::Peritumoral => write!(f, "Peritumoral"),
            TissueType::Tumor => write!(f, "Tumor"),
            TissueType::NonNeoplastic => write!(f, "Non-neoplastic"),
            TissueType::Unavailable => write!(f, "Unavailable"),
            TissueType::Unknown => write!(f, "Unknown"),
            TissueType::Unspecified => write!(f, "Unspecified"),
        }
    }
}

impl Distribution<TissueType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> TissueType {
        match rng.gen_range(0..=8) {
            0 => TissueType::NotReported,
            1 => TissueType::Abnormal,
            2 => TissueType::Normal,
            3 => TissueType::Peritumoral,
            4 => TissueType::Tumor,
            5 => TissueType::NonNeoplastic,
            6 => TissueType::Unavailable,
            7 => TissueType::Unknown,
            _ => TissueType::Unspecified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(TissueType::NotReported.to_string(), "Not Reported");
        assert_eq!(TissueType::Abnormal.to_string(), "Abnormal");
        assert_eq!(TissueType::Normal.to_string(), "Normal");
        assert_eq!(TissueType::Peritumoral.to_string(), "Peritumoral");
        assert_eq!(TissueType::Tumor.to_string(), "Tumor");
        assert_eq!(TissueType::NonNeoplastic.to_string(), "Non-neoplastic");
        assert_eq!(TissueType::Unavailable.to_string(), "Unavailable");
        assert_eq!(TissueType::Unknown.to_string(), "Unknown");
        assert_eq!(TissueType::Unspecified.to_string(), "Unspecified");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&TissueType::NotReported).unwrap(),
            "\"Not Reported\""
        );
        assert_eq!(
            serde_json::to_string(&TissueType::Abnormal).unwrap(),
            "\"Abnormal\""
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
            serde_json::to_string(&TissueType::NonNeoplastic).unwrap(),
            "\"Non-neoplastic\""
        );
        assert_eq!(
            serde_json::to_string(&TissueType::Unavailable).unwrap(),
            "\"Unavailable\""
        );
        assert_eq!(
            serde_json::to_string(&TissueType::Unknown).unwrap(),
            "\"Unknown\""
        );
        assert_eq!(
            serde_json::to_string(&TissueType::Unspecified).unwrap(),
            "\"Unspecified\""
        );
    }
}
