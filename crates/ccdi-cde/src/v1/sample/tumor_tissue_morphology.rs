use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11326261 v1.00`**
///
/// This metadata element is defined by the caDSR as "The microscopic anatomy of
/// normal and abnormal cells and tissues of the specimen as captured in the
/// morphology codes of the International Classification of Diseases for
/// Oncology, 3rd Edition (ICD-O-3)."
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11326261%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::sample::TumorTissueMorphology)]
pub struct TumorTissueMorphology {
    /// The ICD-O-3 code.
    icd_o_3: String,
}

impl From<String> for TumorTissueMorphology {
    fn from(value: String) -> Self {
        TumorTissueMorphology { icd_o_3: value }
    }
}

impl CDE for TumorTissueMorphology {}

impl std::fmt::Display for TumorTissueMorphology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.icd_o_3)
    }
}
