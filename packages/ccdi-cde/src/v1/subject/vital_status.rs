use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 2847330 v1.00`**
///
/// This metadata element is defined by the caDSR as "The response to a question
/// that describes a participant's survival status."
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2847330%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::subject::VitalStatus)]
pub enum VitalStatus {
    /// `Not reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 2572231
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   12/29/2020
    ///
    /// Not provided or available.
    #[serde(rename = "Not reported")]
    NotReported,

    /// `Alive`
    ///
    /// * **VM Long Name**: Alive
    /// * **VM Public ID**: 2580948
    /// * **Concept Code**: C37987
    /// * **Begin Date**:   03/09/2009
    ///
    /// Showing characteristics of life; displaying signs of life.
    Alive,

    /// `Dead`
    ///
    /// * **VM Long Name**: Death
    /// * **VM Public ID**: 2847328
    /// * **Concept Code**: C28554
    /// * **Begin Date**:   03/09/2009
    ///
    /// The absence of life or state of being dead.
    Dead,

    /// `Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 2575365
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   03/09/2009
    ///
    /// Not known, not observed, not recorded, or refused.
    Unknown,

    /// `Unspecified`
    ///
    /// * **VM Long Name**: Unspecified
    /// * **VM Public ID**: 2573360
    /// * **Concept Code**: C38046
    /// * **Begin Date**:   03/09/2009
    ///
    /// Not stated explicitly or in detail.
    Unspecified,
}

impl CDE for VitalStatus {}

impl std::fmt::Display for VitalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VitalStatus::NotReported => write!(f, "Not reported"),
            VitalStatus::Alive => write!(f, "Alive"),
            VitalStatus::Dead => write!(f, "Dead"),
            VitalStatus::Unknown => write!(f, "Unknown"),
            VitalStatus::Unspecified => write!(f, "Unspecified"),
        }
    }
}

impl Distribution<VitalStatus> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> VitalStatus {
        match rng.gen_range(0..=4) {
            0 => VitalStatus::NotReported,
            1 => VitalStatus::Alive,
            2 => VitalStatus::Dead,
            3 => VitalStatus::Unknown,
            _ => VitalStatus::Unspecified,
        }
    }
}
