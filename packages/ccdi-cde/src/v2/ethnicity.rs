use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **caDSR CDE 2192217 v2.00**
///
/// This metadata element is defined by the caDSR as "The text for reporting
/// information about ethnicity based on the Office of Management and Budget
/// (OMB) categories." Upon examination of the large number of projects using
/// the term, it appears to be the preferred term for the general concept of
/// ethnicity.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192217%20and%20ver_nr=2.0>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = cde::v2::Ethnicity)]
pub enum Ethnicity
where
    Self: CDE,
{
    /// Not Allowed To Collect
    ///
    /// An indicator that specifies that a collection event was not permitted.
    #[serde(rename = "Not allowed to collect")]
    NotAllowedToCollect,

    /// Hispanic or Latino
    ///
    /// A person of Cuban, Mexican, Puerto Rican, South or Central American, or
    /// other Spanish culture or origin, regardless of race. The term, "Spanish
    /// origin," can be used in addition to "Hispanic or Latino." (OMB)
    #[serde(rename = "Hispanic or Latino")]
    HispanicOrLatino,

    /// Not Hispanic or Latino
    ///
    /// A person not of Cuban, Mexican, Puerto Rican, South or Central American,
    /// or other Spanish culture or origin, regardless of race.
    #[serde(rename = "Not Hispanic or Latino")]
    NotHispanicOrLatino,

    /// Unknown
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,

    /// Not Reported
    ///
    /// Not provided or available.
    #[serde(rename = "Not reported")]
    NotReported,
}

impl CDE for Ethnicity {}

impl crate::Standard for Ethnicity {
    fn standard() -> &'static str {
        "caDSR CDE 2192217 v2.00"
    }
}

impl crate::Url for Ethnicity {
    fn url() -> &'static str {
        "https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192217%20and%20ver_nr=2.0"
    }
}

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
