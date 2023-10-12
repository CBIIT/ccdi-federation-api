use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **caDSR CDE 2192199 v1.00**
///
/// This metadata element is defined by the caDSR as "The text for reporting
/// information about race based on the Office of Management and Budget (OMB)
/// categories.". Upon examination of the large number of projects using the
/// term, it appears to be the preferred term for the general concept of race.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192199%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = cde::v1::Race)]
pub enum Race
where
    Self: CDE,
{
    /// Not Allowed To Collect
    ///
    /// An indicator that specifies that a collection event was not permitted.
    #[serde(rename = "Not allowed to collect")]
    NotAllowedToCollect,

    /// Native Hawaiian or Other Pacific Islander
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

    /// Not Reported
    ///
    /// Not provided or available.
    #[serde(rename = "Not Reported")]
    NotReported,

    /// Unknown
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,

    /// American Indian or Alaska Native
    ///
    /// A person having origins in any of the original peoples of North and
    /// South America (including Central America) and who maintains tribal
    /// affiliation or community attachment. (OMB)
    #[serde(rename = "American Indian or Alaska Native")]
    AmericanIndianOrAlaskaNative,

    /// Asian
    ///
    /// A person having origins in any of the original peoples of the Far East,
    /// Southeast Asia, or the Indian subcontinent, including for example,
    /// Cambodia, China, India, Japan, Korea, Malaysia, Pakistan, the Philippine
    /// Islands, Thailand, and Vietnam. (OMB)
    #[serde(rename = "Asian")]
    Asian,

    /// Black or African American
    ///
    /// A person having origins in any of the Black racial groups of Africa.
    /// Terms such as "Haitian" or "Negro" can be used in addition to "Black or
    /// African American". (OMB)
    #[serde(rename = "Black or African American")]
    BlackOrAfricanAmerican,

    /// White
    ///
    /// Denotes person with European, Middle Eastern, or North African ancestral
    /// origin who identifies, or is identified, as White.
    #[serde(rename = "White")]
    White,
}

impl CDE for Race {}

impl crate::Standard for Race {
    fn standard() -> &'static str {
        "caDSR CDE 2192199 v1.00"
    }
}

impl crate::Url for Race {
    fn url() -> &'static str {
        "https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=2192199%20and%20ver_nr=1"
    }
}

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
