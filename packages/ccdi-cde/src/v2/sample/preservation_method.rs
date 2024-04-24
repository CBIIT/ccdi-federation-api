use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 8028962 v2.00`**
///
/// This metadata element is defined by the caDSR as "Text term that represents
/// the method used to maintain the sample or biospecimen in a viable state.".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=8028962%20and%20ver_nr=2>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v2::sample::PreservationMethod)]
pub enum PreservationMethod {
    /// `-80 degrees C`
    ///
    /// * **VM Long Name**: Minus 80 Degrees Celsius
    /// * **VM Public ID**: 14758216
    /// * **Concept Code**: C185336
    /// * **Begin Date**:   02/13/2024
    ///
    /// A temperature of minus 80 celsius.
    #[serde(rename = "-80 degrees C")]
    MinusEightyDegreesC,

    /// `Cryopreserved`
    ///
    /// * **VM Long Name**: Cryopreservation
    /// * **VM Public ID**: 2568180
    /// * **Concept Code**: C16475
    /// * **Begin Date**:   02/07/2022
    ///
    /// Preservation of cells, tissues, organs, or embryos by storage at low
    /// temperatures.
    #[serde(rename = "Cryopreserved")]
    Cryopreserved,

    /// `EDTA`
    ///
    /// * **VM Long Name**: Edetic Acid
    /// * **VM Public ID**: 3232500
    /// * **Concept Code**: C61742
    /// * **Begin Date**:   03/06/2024
    ///
    /// The acid form of edetate, a chelating agent with anti-hypercalcemic and
    /// anticoagulant properties. Edetic acid binds calcium and heavy metal
    /// ions, forming soluble stable complexes which are readily excreted by the
    /// kidneys. This results in a decrease in serum calcium levels. This agent
    /// is also used as an anticoagulant for blood specimens and is applied as a
    /// treatment of lead poisoning.
    #[serde(rename = "EDTA")]
    Edta,

    /// `FFPE`
    ///
    /// * **VM Long Name**: Formalin-Fixed Paraffin-Embedded
    /// * **VM Public ID**: 6050873
    /// * **Concept Code**: C143028
    /// * **Begin Date**:   02/07/2022
    ///
    /// Refers to samples that have been preserved with formalin and then
    /// embedded into a paraffin block for sectioning.
    #[serde(rename = "FFPE")]
    Ffpe,

    /// `Formalin Fixed - Buffered`
    ///
    /// * **VM Long Name**: Buffered Formalin Fixation
    /// * **VM Public ID**: 8031346
    /// * **Concept Code**: C185403
    /// * **Begin Date**:   02/09/2022
    ///
    /// The use of buffered formalin for preservation of tissue samples.
    #[serde(rename = "Formalin Fixed - Buffered")]
    FormalinFixedBuffered,

    /// `Formalin Fixed - Unbuffered`
    ///
    /// * **VM Long Name**: Unbuffered Formalin Fixation
    /// * **VM Public ID**: 8031347
    /// * **Concept Code**: C185402
    /// * **Begin Date**:   02/09/2022
    ///
    /// The use of unbuffered formalin for preservation of tissue samples.
    #[serde(rename = "Formalin Fixed - Unbuffered")]
    FormalinFixedUnbuffered,

    /// `Fresh`
    ///
    /// * **VM Long Name**: Fresh Specimen
    /// * **VM Public ID**: 3210685
    /// * **Concept Code**: C84517
    /// * **Begin Date**:   02/26/2024
    ///
    /// Tissue which has not been exposed to a fixative solution.
    #[serde(rename = "Fresh")]
    Fresh,

    /// `Fresh Dissociated`
    ///
    /// * **VM Long Name**: Fresh Dissociated Tissue Preparation
    /// * **VM Public ID**: 8031345
    /// * **Concept Code**: C185404
    /// * **Begin Date**:   02/09/2022
    ///
    /// A tissue preparation process that takes fresh tissue and dissociates it
    /// into single cell suspensions.
    #[serde(rename = "Fresh Dissociated")]
    FreshDissociated,

    /// `Fresh Dissociated and Single Cell Sorted`
    ///
    /// * **VM Long Name**: Fresh Dissociated Tissue Preparation, Single Cell
    ///   Sorted
    /// * **VM Public ID**: 8031344
    /// * **Concept Code**: C185405
    /// * **Begin Date**:   02/09/2022
    ///
    /// A tissue preparation process that separates fresh dissociated tissue
    /// cells into cell populations by single cell sorting.
    #[serde(rename = "Fresh Dissociated and Single Cell Sorted")]
    FreshDissociatedAndSingleCellSorted,

    /// `Fresh Dissociated and Single Cell Sorted into Plates`
    ///
    /// * **VM Long Name**: Fresh Dissociated Tissue Preparation, Single Cell
    ///   Sorted into Plates
    /// * **VM Public ID**: 8031343
    /// * **Concept Code**: C185406
    /// * **Begin Date**:   02/09/2022
    ///
    /// A tissue preparation process that takes dissociated, sorted cells and
    /// distributes them into cell propagation plates.
    #[serde(rename = "Fresh Dissociated and Single Cell Sorted into Plates")]
    FreshDissociatedAndSingleCellSortedIntoPlates,

    /// `Frozen`
    ///
    /// * **VM Long Name**: Frozen Specimen
    /// * **VM Public ID**: 3167629
    /// * **Concept Code**: C70717
    /// * **Begin Date**:   02/07/2022
    ///
    /// A specimen that has been subjected to and immobilized by severe cold.
    #[serde(rename = "Frozen")]
    Frozen,

    /// `Liquid Nitrogen`
    ///
    /// * **VM Long Name**: Liquid Nitrogen Storage
    /// * **VM Public ID**: 8015821
    /// * **Concept Code**: C185338
    /// * **Begin Date**:   02/07/2022
    ///
    /// An indication that a material has been stored in liquid nitrogen.
    #[serde(rename = "Liquid Nitrogen")]
    LiquidNitrogen,

    /// `Not Reported`
    ///
    /// * **VM Long Name**: Not Reported
    /// * **VM Public ID**: 2572231
    /// * **Concept Code**: C43234
    /// * **Begin Date**:   02/07/2022
    ///
    /// Not provided or available.
    #[serde(rename = "Not Reported")]
    NotReported,

    /// `OCT`
    ///
    /// * **VM Long Name**: Optimal Cutting Temperature Compound
    /// * **VM Public ID**: 5428812
    /// * **Concept Code**: C63523
    /// * **Begin Date**:   02/07/2022
    ///
    /// A solution of water soluble glycols and resins that provide a specimen
    /// support matrix for cryostat sectioning at temperatures of -10 degrees C
    /// and below.
    #[serde(rename = "OCT")]
    Oct,

    /// `Snap Frozen`
    ///
    /// * **VM Long Name**: Quick Freeze
    /// * **VM Public ID**: 4399755
    /// * **Concept Code**: C63521
    /// * **Begin Date**:   02/07/2022
    ///
    /// To freeze rapidly so as to preserve structure and prevent ice crystal
    /// formation.
    #[serde(rename = "Snap Frozen")]
    SnapFrozen,

    /// `Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 5682953
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   02/07/2022
    ///
    /// Not known, not observed, not recorded, or refused.
    #[serde(rename = "Unknown")]
    Unknown,
}

impl CDE for PreservationMethod {}

impl std::fmt::Display for PreservationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreservationMethod::MinusEightyDegreesC => write!(f, "-80 degrees C"),
            PreservationMethod::Cryopreserved => write!(f, "Cryopreserved"),
            PreservationMethod::Edta => write!(f, "EDTA"),
            PreservationMethod::Ffpe => write!(f, "FFPE"),
            PreservationMethod::FormalinFixedBuffered => write!(f, "Formalin Fixed - Buffered"),
            PreservationMethod::FormalinFixedUnbuffered => write!(f, "Formalin Fixed - Unbuffered"),
            PreservationMethod::Fresh => write!(f, "Fresh"),
            PreservationMethod::FreshDissociated => write!(f, "Fresh Dissociated"),
            PreservationMethod::FreshDissociatedAndSingleCellSorted => {
                write!(f, "Fresh Dissociated and Single Cell Sorted")
            }
            PreservationMethod::FreshDissociatedAndSingleCellSortedIntoPlates => {
                write!(f, "Fresh Dissociated and Single Cell Sorted into Plates")
            }
            PreservationMethod::Frozen => write!(f, "Frozen"),
            PreservationMethod::LiquidNitrogen => write!(f, "Liquid Nitrogen"),
            PreservationMethod::NotReported => write!(f, "Not Reported"),
            PreservationMethod::Oct => write!(f, "OCT"),
            PreservationMethod::SnapFrozen => write!(f, "Snap Frozen"),
            PreservationMethod::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Distribution<PreservationMethod> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PreservationMethod {
        match rng.gen_range(0..16) {
            0 => PreservationMethod::MinusEightyDegreesC,
            1 => PreservationMethod::Cryopreserved,
            2 => PreservationMethod::Edta,
            3 => PreservationMethod::Ffpe,
            4 => PreservationMethod::FormalinFixedBuffered,
            5 => PreservationMethod::FormalinFixedUnbuffered,
            6 => PreservationMethod::Fresh,
            7 => PreservationMethod::FreshDissociated,
            8 => PreservationMethod::FreshDissociatedAndSingleCellSorted,
            9 => PreservationMethod::FreshDissociatedAndSingleCellSortedIntoPlates,
            10 => PreservationMethod::Frozen,
            11 => PreservationMethod::LiquidNitrogen,
            12 => PreservationMethod::NotReported,
            13 => PreservationMethod::Oct,
            14 => PreservationMethod::SnapFrozen,
            _ => PreservationMethod::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        assert_eq!(
            PreservationMethod::MinusEightyDegreesC.to_string(),
            "-80 degrees C"
        );
        assert_eq!(
            PreservationMethod::Cryopreserved.to_string(),
            "Cryopreserved"
        );
        assert_eq!(PreservationMethod::Edta.to_string(), "EDTA");
        assert_eq!(PreservationMethod::Ffpe.to_string(), "FFPE");
        assert_eq!(
            PreservationMethod::FormalinFixedBuffered.to_string(),
            "Formalin Fixed - Buffered"
        );
        assert_eq!(
            PreservationMethod::FormalinFixedUnbuffered.to_string(),
            "Formalin Fixed - Unbuffered"
        );
        assert_eq!(PreservationMethod::Fresh.to_string(), "Fresh");
        assert_eq!(
            PreservationMethod::FreshDissociated.to_string(),
            "Fresh Dissociated"
        );
        assert_eq!(
            PreservationMethod::FreshDissociatedAndSingleCellSorted.to_string(),
            "Fresh Dissociated and Single Cell Sorted"
        );
        assert_eq!(
            PreservationMethod::FreshDissociatedAndSingleCellSortedIntoPlates.to_string(),
            "Fresh Dissociated and Single Cell Sorted into Plates"
        );
        assert_eq!(PreservationMethod::Frozen.to_string(), "Frozen");
        assert_eq!(
            PreservationMethod::LiquidNitrogen.to_string(),
            "Liquid Nitrogen"
        );
        assert_eq!(PreservationMethod::NotReported.to_string(), "Not Reported");
        assert_eq!(PreservationMethod::Oct.to_string(), "OCT");
        assert_eq!(PreservationMethod::SnapFrozen.to_string(), "Snap Frozen");
        assert_eq!(PreservationMethod::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        assert_eq!(
            serde_json::to_string(&PreservationMethod::MinusEightyDegreesC).unwrap(),
            "\"-80 degrees C\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::Cryopreserved).unwrap(),
            "\"Cryopreserved\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::Edta).unwrap(),
            "\"EDTA\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::Ffpe).unwrap(),
            "\"FFPE\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::FormalinFixedBuffered).unwrap(),
            "\"Formalin Fixed - Buffered\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::FormalinFixedUnbuffered).unwrap(),
            "\"Formalin Fixed - Unbuffered\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::Fresh).unwrap(),
            "\"Fresh\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::FreshDissociated).unwrap(),
            "\"Fresh Dissociated\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::FreshDissociatedAndSingleCellSorted)
                .unwrap(),
            "\"Fresh Dissociated and Single Cell Sorted\""
        );
        assert_eq!(
            serde_json::to_string(
                &PreservationMethod::FreshDissociatedAndSingleCellSortedIntoPlates
            )
            .unwrap(),
            "\"Fresh Dissociated and Single Cell Sorted into Plates\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::Frozen).unwrap(),
            "\"Frozen\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::LiquidNitrogen).unwrap(),
            "\"Liquid Nitrogen\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::NotReported).unwrap(),
            "\"Not Reported\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::Oct).unwrap(),
            "\"OCT\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::SnapFrozen).unwrap(),
            "\"Snap Frozen\""
        );
        assert_eq!(
            serde_json::to_string(&PreservationMethod::Unknown).unwrap(),
            "\"Unknown\""
        );
    }
}
