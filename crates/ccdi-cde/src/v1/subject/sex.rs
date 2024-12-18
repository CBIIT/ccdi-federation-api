use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 6343385 v1.00`**
///
/// This metadata element is defined by the caDSR as "Sex of the subject as
/// determined by the investigator." In particular, this field does not dictate
/// the time period: whether it represents sex at birth, sex at sample
/// collection, or any other determined time point. Further, the descriptions
/// for F and M suggest that this term can represent either biological sex,
/// cultural gender roles, or both. Thus, this field cannot be assumed to
/// strictly represent biological sex.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6343385%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::subject::Sex)]
pub enum Sex {
    /// `U`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 5682944
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   06/27/2018
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "U")]
    Unknown,

    /// `F`
    ///
    /// * **VM Long Name**: Female
    /// * **VM Public ID**: 2567172
    /// * **Concept Code**: C16576
    /// * **Begin Date**:   06/27/2018
    ///
    /// A person who belongs to the sex that normally produces ova. The term is
    /// used to indicate biological sex distinctions, or cultural gender role
    /// distinctions, or both.
    #[serde(rename = "F")]
    Female,

    /// `M`
    ///
    /// * **VM Long Name**: Male
    /// * **VM Public ID**: 2567171
    /// * **Concept Code**: C20197
    /// * **Begin Date**:   06/27/2018
    ///
    /// A person who belongs to the sex that normally produces sperm. The term
    /// is used to indicate biological sex distinctions, cultural gender role
    /// distinctions, or both.
    #[serde(rename = "M")]
    Male,

    /// `UNDIFFERENTIATED`
    ///
    /// * **VM Long Name**: Intersex
    /// * **VM Public ID**: 2575558
    /// * **Concept Code**: C45908
    /// * **Begin Date**:   06/27/2018
    ///
    /// A person (one of unisexual specimens) who is born with genitalia and/or
    /// secondary sexual characteristics of indeterminate sex, or which combine
    /// features of both sexes.
    #[serde(rename = "UNDIFFERENTIATED")]
    Undifferentiated,
}

impl CDE for Sex {}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sex::Unknown => write!(f, "U"),
            Sex::Female => write!(f, "F"),
            Sex::Male => write!(f, "M"),
            Sex::Undifferentiated => write!(f, "UNDIFFERENTIATED"),
        }
    }
}

impl Distribution<Sex> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sex {
        match rng.gen_range(0..=3) {
            0 => Sex::Unknown,
            1 => Sex::Female,
            2 => Sex::Male,
            _ => Sex::Undifferentiated,
        }
    }
}
