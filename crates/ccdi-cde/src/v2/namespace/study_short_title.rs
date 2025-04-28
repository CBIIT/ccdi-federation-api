use std::ops::Deref;
use std::ops::DerefMut;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11459812 v2.00`**
///
/// This metadata element is defined by the caDSR as "The acronym or
/// abbreviated form of the title for a research data collection".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11459812%20and%20ver_nr=2>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v2::namespace::StudyShortTitle)]
pub struct StudyShortTitle(String);

impl From<String> for StudyShortTitle {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for StudyShortTitle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StudyShortTitle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CDE for StudyShortTitle {}

impl std::fmt::Display for StudyShortTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
