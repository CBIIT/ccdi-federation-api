use std::ops::Deref;
use std::ops::DerefMut;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 14528051 v1.00`**
///
/// This metadata element is defined by the caDSR as "A sequence of characters
/// used to uniquely identify, name, or characterize the study funding
/// organization.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=14528051%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::namespace::StudyFundingOrganization)]
pub struct StudyFundingId(String);

impl From<String> for StudyFundingId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for StudyFundingId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StudyFundingId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CDE for StudyFundingId {}

impl std::fmt::Display for StudyFundingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
