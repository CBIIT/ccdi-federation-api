use std::ops::Deref;
use std::ops::DerefMut;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11459810 v1.00`**
///
/// This metadata element is defined by the caDSR as "The acronym or abbreviated
/// form of the title for a research data collection. Example – GLIOMA01".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11459810%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::namespace::StudyName)]
pub struct StudyName(String);

impl From<String> for StudyName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for StudyName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StudyName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CDE for StudyName {}

impl std::fmt::Display for StudyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
