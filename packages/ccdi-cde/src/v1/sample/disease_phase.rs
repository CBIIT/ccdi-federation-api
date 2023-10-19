use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 12217251 v1.00`**
///
/// This metadata element is defined by the caDSR as "The stage or period of an
/// individual's treatment process during which relevant observations were
/// recorded.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=12217251%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::sample::DiseasePhase)]
pub enum DiseasePhase {
    /// `Post-Mortem`
    ///
    /// * **VM Long Name**: Postmortem
    /// * **VM Public ID**: 5236215
    /// * **Concept Code**: C94193
    /// * **Begin Date**:   03/10/2023
    ///
    /// After death. Often used to describe an autopsy.
    #[serde(rename = "Post-Mortem")]
    PostMortem,

    /// `Not Reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 5612322
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   03/09/2023
    ///
    /// Not provided or available.
    #[serde(rename = "Not Reported")]
    NotReported,

    /// `Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 4266671
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   03/09/2023
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,

    /// `Initial Diagnosis`
    ///
    /// * **VM Long Name**: Initial Diagnosis
    /// * **VM Public ID**: 8002761
    /// * **Concept Code**: C156813
    /// * **Begin Date**:   12/27/2022
    ///
    /// The first diagnosis of the individual's condition.
    #[serde(rename = "Initial Diagnosis")]
    InitialDiagnosis,

    /// `Progression`
    ///
    /// * **VM Long Name**: Disease Progression
    /// * **VM Public ID**: 2816916
    /// * **Concept Code**: C17747
    /// * **Begin Date**:   2/27/2022
    ///
    /// The worsening of a disease over time
    #[serde(rename = "Progression")]
    Progression,

    /// `Refactory`
    ///
    /// * **VM Long Name**: Refractory
    /// * **VM Public ID**: 2566882
    /// * **Concept Code**: C38014
    /// * **Begin Date**:   12/27/2022
    ///
    /// Not responding to treatment.
    #[serde(rename = "Refactory")]
    Refactory,

    /// `Relapse`
    ///
    /// * **VM Long Name**: Recurrent Disease
    /// * **VM Public ID**: 3828963
    /// * **Concept Code**: C38155
    /// * **Begin Date**:   12/27/2022
    ///
    /// The return of a disease after a period of remission.
    #[serde(rename = "Relapse")]
    Relapse,

    /// `Relapse/Progression`
    ///
    /// * **VM Long Name**: Disease Relapse/Progression
    /// * **VM Public ID**: 12217248
    /// * **Concept Code**: C174991
    /// * **Begin Date**:   12/27/2022
    ///
    /// Either the return of the disease or the progression of the disease.
    #[serde(rename = "Relapse/Progression")]
    RelapseOrProgression,
}

impl CDE for DiseasePhase {}

impl std::fmt::Display for DiseasePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiseasePhase::PostMortem => write!(f, "Post-Mortem"),
            DiseasePhase::NotReported => write!(f, "Not Reported"),
            DiseasePhase::Unknown => write!(f, "Unknown"),
            DiseasePhase::InitialDiagnosis => write!(f, "Initial Diagnosis"),
            DiseasePhase::Progression => write!(f, "Progression"),
            DiseasePhase::Refactory => write!(f, "Refactory"),
            DiseasePhase::Relapse => write!(f, "Relapse"),
            DiseasePhase::RelapseOrProgression => write!(f, "Relapse/Progression"),
        }
    }
}

impl Distribution<DiseasePhase> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DiseasePhase {
        match rng.gen_range(0..=7) {
            0 => DiseasePhase::PostMortem,
            1 => DiseasePhase::NotReported,
            2 => DiseasePhase::Unknown,
            3 => DiseasePhase::InitialDiagnosis,
            4 => DiseasePhase::Progression,
            5 => DiseasePhase::Refactory,
            6 => DiseasePhase::Relapse,
            _ => DiseasePhase::RelapseOrProgression,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(DiseasePhase::PostMortem.to_string(), "Post-Mortem");
        assert_eq!(DiseasePhase::NotReported.to_string(), "Not Reported");
        assert_eq!(DiseasePhase::Unknown.to_string(), "Unknown");
        assert_eq!(
            DiseasePhase::InitialDiagnosis.to_string(),
            "Initial Diagnosis"
        );
        assert_eq!(DiseasePhase::Progression.to_string(), "Progression");
        assert_eq!(DiseasePhase::Refactory.to_string(), "Refactory");
        assert_eq!(DiseasePhase::Relapse.to_string(), "Relapse");
        assert_eq!(
            DiseasePhase::RelapseOrProgression.to_string(),
            "Relapse/Progression"
        );
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&DiseasePhase::PostMortem).unwrap(),
            "\"Post-Mortem\""
        );
        assert_eq!(
            serde_json::to_string(&DiseasePhase::NotReported).unwrap(),
            "\"Not Reported\""
        );
        assert_eq!(
            serde_json::to_string(&DiseasePhase::Unknown).unwrap(),
            "\"Unknown\""
        );
        assert_eq!(
            serde_json::to_string(&DiseasePhase::InitialDiagnosis).unwrap(),
            "\"Initial Diagnosis\""
        );
        assert_eq!(
            serde_json::to_string(&DiseasePhase::Progression).unwrap(),
            "\"Progression\""
        );
        assert_eq!(
            serde_json::to_string(&DiseasePhase::Refactory).unwrap(),
            "\"Refactory\""
        );
        assert_eq!(
            serde_json::to_string(&DiseasePhase::Relapse).unwrap(),
            "\"Relapse\""
        );
        assert_eq!(
            serde_json::to_string(&DiseasePhase::RelapseOrProgression).unwrap(),
            "\"Relapse/Progression\""
        );
    }
}
