//! Metadata for a [`Sample`](super::Sample).

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::field;
use crate::metadata::fields;

pub mod builder;

pub use builder::Builder;

/// Metadata associated with a sample.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::sample::Metadata)]
pub struct Metadata {
    /// The phase of the disease when this sample was acquired.
    #[schema(value_type = field::unowned::sample::DiseasePhase, nullable = true)]
    disease_phase: Option<field::unowned::sample::DiseasePhase>,

    /// The type of tissue for this sample.
    #[schema(value_type = field::unowned::sample::TissueType, nullable = true)]
    tissue_type: Option<field::unowned::sample::TissueType>,

    /// The classification for this tumor based mainly on histological
    /// characteristics.
    #[schema(value_type = field::unowned::sample::TumorClassification, nullable = true)]
    tumor_classification: Option<field::unowned::sample::TumorClassification>,

    /// An unharmonized map of metadata fields.
    #[schema(value_type = fields::Unharmonized)]
    #[serde(skip_serializing_if = "fields::Unharmonized::is_empty")]
    unharmonized: fields::Unharmonized,
}

impl Metadata {
    /// Gets the harmonized disease phase for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::DiseasePhase;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///                 .disease_phase(
    ///                     DiseasePhase::new(cde::v1::sample::DiseasePhase::InitialDiagnosis, None, None)
    ///                 )
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.disease_phase(),
    ///     &Some(DiseasePhase::new(cde::v1::sample::DiseasePhase::InitialDiagnosis, None, None))
    /// );
    /// ```
    pub fn disease_phase(&self) -> &Option<field::unowned::sample::DiseasePhase> {
        &self.disease_phase
    }

    /// Gets the harmonized tissue type for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::TissueType;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///                 .tissue_type(
    ///                     TissueType::new(cde::v2::sample::TissueType::Tumor, None, None)
    ///                 )
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.tissue_type(),
    ///     &Some(TissueType::new(cde::v2::sample::TissueType::Tumor, None, None))
    /// );
    /// ```
    pub fn tissue_type(&self) -> &Option<field::unowned::sample::TissueType> {
        &self.tissue_type
    }

    /// Gets the harmonized tumor classification for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::TumorClassification;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///                 .tumor_classification(
    ///                     TumorClassification::new(cde::v1::sample::TumorClassification::Primary, None, None)
    ///                 )
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.tumor_classification(),
    ///     &Some(TumorClassification::new(cde::v1::sample::TumorClassification::Primary, None, None))
    /// );
    /// ```
    pub fn tumor_classification(&self) -> &Option<field::unowned::sample::TumorClassification> {
        &self.tumor_classification
    }

    /// Gets the unharmonized fields for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::UnharmonizedField;
    /// use models::metadata::field::owned;
    /// use models::metadata::field::unowned;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///                         .insert_unharmonized(
    ///                             "unowned",
    ///                             UnharmonizedField::Unowned(unowned::Field::new(Value::String("test".into()), None, None))
    ///                         )
    ///                         .insert_unharmonized(
    ///                             "owned",
    ///                             UnharmonizedField::Owned(owned::Field::new(Value::String("test".into()), None, None, None))
    ///                         )
    ///                         .build();
    ///
    /// assert!(matches!(metadata.unharmonized().inner().get("unowned".into()), Some(&UnharmonizedField::Unowned(_))));
    /// assert!(matches!(metadata.unharmonized().inner().get("owned".into()), Some(&UnharmonizedField::Owned(_))));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn unharmonized(&self) -> &fields::Unharmonized {
        &self.unharmonized
    }

    /// Generates a random [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::sample::Metadata;
    ///
    /// let metadata = Metadata::random();
    /// ```
    pub fn random() -> Metadata {
        Metadata {
            disease_phase: rand::random(),
            tissue_type: rand::random(),
            tumor_classification: rand::random(),
            unharmonized: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sample::metadata::builder;

    #[test]
    fn it_skips_serializing_the_unharmonized_key_when_it_is_empty() {
        let metadata = builder::Builder::default().build();
        assert_eq!(
            &serde_json::to_string(&metadata).unwrap(),
            "{\"disease_phase\":null,\"tissue_type\":null,\"tumor_classification\":null}"
        );
    }
}
