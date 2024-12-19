use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 2192199 v1.00`**
///
/// This metadata element is defined by the caDSR as "The text for reporting
/// information about race based on the Office of Management and Budget (OMB)
/// categories.". Upon examination of the large number of projects using the
/// term, it appears to be the preferred term for the general concept of race.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192199%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::subject::Race)]
pub enum Race {
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

    /// `Native Hawaiian or other Pacific Islander`
    ///
    /// * **VM Long Name**: Native Hawaiian or Other Pacific Islander
    /// * **VM Public ID**: 2572235
    /// * **Concept Code**: C41219
    /// * **Begin Date**:   05/31/2002
    ///
    /// Denotes a person having origins in any of the original peoples of
    /// Hawaii, Guam, Samoa, or other Pacific Islands. The term covers
    /// particularly people who identify themselves as part-Hawaiian, Native
    /// Hawaiian, Guamanian or Chamorro, Carolinian, Samoan, Chuukese (Trukese),
    /// Fijian, Kosraean, Melanesian, Micronesian, Northern Mariana Islander,
    /// Palauan, Papua New Guinean, Pohnpeian, Polynesian, Solomon Islander,
    /// Tahitian, Tokelauan, Tongan, Yapese, or Pacific Islander, not specified.
    #[serde(rename = "Native Hawaiian or other Pacific Islander")]
    NativeHawaiianOrOtherPacificIslander,

    /// `Not Reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 2572578
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   10/16/2003
    ///
    /// Not provided or available.
    #[serde(rename = "Not Reported")]
    NotReported,

    /// `Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 2572577
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   02/11/2002
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,

    /// `American Indian or Alaska Native`
    ///
    /// * **VM Long Name**: American Indian or Alaska Native
    /// * **VM Public ID**: 2572232
    /// * **Concept Code**: C41259
    /// * **Begin Date**:   05/31/2002
    ///
    /// A person having origins in any of the original peoples of North and
    /// South America (including Central America) and who maintains tribal
    /// affiliation or community attachment. (OMB)
    #[serde(rename = "American Indian or Alaska Native")]
    AmericanIndianOrAlaskaNative,

    /// `Asian`
    ///
    /// * **VM Long Name**: Asian
    /// * **VM Public ID**: 2572233
    /// * **Concept Code**: C41260
    /// * **Begin Date**:   05/31/2002
    ///
    /// A person having origins in any of the original peoples of the Far East,
    /// Southeast Asia, or the Indian subcontinent, including for example,
    /// Cambodia, China, India, Japan, Korea, Malaysia, Pakistan, the Philippine
    /// Islands, Thailand, and Vietnam. (OMB)
    #[serde(rename = "Asian")]
    Asian,

    /// `Black or African American`
    ///
    /// * **VM Long Name**: Black or African American
    /// * **VM Public ID**: 2572313
    /// * **Concept Code**: C16352
    /// * **Begin Date**:   05/31/2002
    ///
    /// A person having origins in any of the Black racial groups of Africa.
    /// Terms such as "Haitian" or "Negro" can be used in addition to "Black or
    /// African American". (OMB)
    #[serde(rename = "Black or African American")]
    BlackOrAfricanAmerican,

    /// `White`
    ///
    /// * **VM Long Name**: White
    /// * **VM Public ID**: 2572236
    /// * **Concept Code**: C41261
    /// * **Begin Date**:   05/31/2002
    ///
    /// Denotes person with European, Middle Eastern, or North African ancestral
    /// origin who identifies, or is identified, as White.
    #[serde(rename = "White")]
    White,
}

impl CDE for Race {}

impl std::fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Race::NotAllowedToCollect => write!(f, "Not allowed to collect"),
            Race::NativeHawaiianOrOtherPacificIslander => {
                write!(f, "Native Hawaiian or other Pacific Islander")
            }
            Race::NotReported => write!(f, "Not Reported"),
            Race::Unknown => write!(f, "Unknown"),
            Race::AmericanIndianOrAlaskaNative => write!(f, "American Indian or Alaska Native"),
            Race::Asian => write!(f, "Asian"),
            Race::BlackOrAfricanAmerican => write!(f, "Black or African American"),
            Race::White => write!(f, "White"),
        }
    }
}

impl Distribution<Race> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Race {
        match rng.gen_range(0..=7) {
            0 => Race::NotAllowedToCollect,
            1 => Race::NativeHawaiianOrOtherPacificIslander,
            2 => Race::NotReported,
            3 => Race::Unknown,
            4 => Race::AmericanIndianOrAlaskaNative,
            5 => Race::Asian,
            6 => Race::BlackOrAfricanAmerican,
            _ => Race::White,
        }
    }
}
