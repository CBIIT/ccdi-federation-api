use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 2192217 v2.00`**
///
/// This metadata element is defined by the caDSR as "The text for reporting
/// information about ethnicity based on the Office of Management and Budget
/// (OMB) categories." Upon examination of the large number of projects using
/// the term, it appears to be the preferred term for the general concept of
/// ethnicity.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192217%20and%20ver_nr=2>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v2::subject::Ethnicity)]
pub enum Ethnicity {
    /// `Not allowed to collect`
    ///
    /// * **VM Long Name**: Not Allowed To Collect
    /// * **VM Public ID**: 6662191
    /// * **Concept Code**: C141478
    /// * **Begin Date**:   03/06/2019
    ///
    /// An indicator that specifies that a collection event was not permitted.
    #[serde(rename = "Not allowed to collect")]
    NotAllowedToCollect,

    /// `Hispanic or Latino`
    ///
    /// * **VM Long Name**: Hispanic or Latino
    /// * **VM Public ID**: 2581176
    /// * **Concept Code**: C17459
    /// * **Begin Date**:   05/20/2002
    ///
    /// A person of Cuban, Mexican, Puerto Rican, South or Central American, or
    /// other Spanish culture or origin, regardless of race. The term, "Spanish
    /// origin," can be used in addition to "Hispanic or Latino." (OMB)
    #[serde(rename = "Hispanic or Latino")]
    HispanicOrLatino,

    /// `Not Hispanic or Latino`
    ///
    /// * **VM Long Name**: Not Hispanic or Latino
    /// * **VM Public ID**: 2567110
    /// * **Concept Code**: C41222
    /// * **Begin Date**:   05/20/2002
    ///
    /// A person not of Cuban, Mexican, Puerto Rican, South or Central American,
    /// or other Spanish culture or origin, regardless of race.
    #[serde(rename = "Not Hispanic or Latino")]
    NotHispanicOrLatino,

    /// `Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 2572577
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   07/09/2002
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,

    /// `Not reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 2572578
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   10/16/2003
    ///
    /// Not provided or available.
    #[serde(rename = "Not reported")]
    NotReported,
}

impl CDE for Ethnicity {}

impl std::fmt::Display for Ethnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ethnicity::NotAllowedToCollect => write!(f, "Not allowed to collect"),
            Ethnicity::HispanicOrLatino => write!(f, "Hispanic or Latino"),
            Ethnicity::NotHispanicOrLatino => write!(f, "Not Hispanic or Latino"),
            Ethnicity::Unknown => write!(f, "Unknown"),
            Ethnicity::NotReported => write!(f, "Not reported"),
        }
    }
}

impl Distribution<Ethnicity> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Ethnicity {
        match rng.gen_range(0..=4) {
            0 => Ethnicity::NotAllowedToCollect,
            1 => Ethnicity::HispanicOrLatino,
            2 => Ethnicity::NotHispanicOrLatino,
            3 => Ethnicity::Unknown,
            _ => Ethnicity::NotReported,
        }
    }
}
