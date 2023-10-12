use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// A kind of [`Subject`](super::Subject).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::subject::Kind)]
pub enum Kind {
    /// A participant or individual.
    #[serde(rename = "Participant")]
    Participant,

    /// A xenograft which was originally derived from a patient tumor.
    #[serde(rename = "Patient Derived Xenograft")]
    PatientDerivedXenograft,

    /// A cell line.
    #[serde(rename = "Cell Line")]
    CellLine,

    /// An organoid.
    #[serde(rename = "Organoid")]
    Organoid,
}

impl Distribution<Kind> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Kind {
        match rng.gen_range(0..=3) {
            0 => Kind::Participant,
            1 => Kind::PatientDerivedXenograft,
            2 => Kind::CellLine,
            _ => Kind::Organoid,
        }
    }
}
