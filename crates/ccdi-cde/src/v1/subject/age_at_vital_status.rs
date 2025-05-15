//! Age at vital status for a subject.

use introspect::Introspect;
use ordered_float::OrderedFloat;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 12306025 v1.00`**
///
/// This metadata element is defined by the caDSR as "The age in days
/// of the subject when the vitals measurement was taken.". No permissible
/// values are defined for this CDE.
///
/// * When the age at vital status is collected by the source server in days,
///   the number of days is reported directly.
/// * When the age at vital status is collected by the source server in years,
///   the number of years is multiplied by 365.25 to arrive at an approximate 
///   number of days.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12306025%20and%20ver_nr=1>
#[derive(
    Clone, Debug, Deserialize, Eq, Introspect, Ord, PartialEq, PartialOrd, Serialize, ToSchema,
)]
#[schema(as = cde::v1::subject::AgeAtVitalStatus, value_type = f32)]
pub struct AgeAtVitalStatus(OrderedFloat<f32>);

impl CDE for AgeAtVitalStatus {}

impl std::fmt::Display for AgeAtVitalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<OrderedFloat<f32>> for AgeAtVitalStatus {
    fn from(value: OrderedFloat<f32>) -> Self {
        Self(value)
    }
}
