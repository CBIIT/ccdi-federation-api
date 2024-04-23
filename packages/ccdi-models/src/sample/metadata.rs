//! Metadata for a [`Sample`](super::Sample).

use ordered_float::OrderedFloat;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng as _;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::common;
use crate::metadata::field;
use crate::metadata::fields;
use crate::sample::Identifier;

mod age_at_collection;
mod age_at_diagnosis;
pub mod builder;
mod diagnosis;

pub use age_at_collection::AgeAtCollection;
pub use age_at_diagnosis::AgeAtDiagnosis;
pub use builder::Builder;
pub use diagnosis::Diagnosis;

/// Metadata associated with a sample.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::sample::Metadata)]
pub struct Metadata {
    /// The approximate age at diagnosis.
    #[schema(value_type = field::unowned::sample::AgeAtDiagnosis, nullable = true)]
    age_at_diagnosis: Option<field::unowned::sample::AgeAtDiagnosis>,

    /// The diagnosis for the sample.
    #[schema(value_type = field::unowned::sample::Diagnosis, nullable = true)]
    diagnosis: Option<field::unowned::sample::Diagnosis>,

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

    /// The ICD-O-3 morphology code for the tumor tissue.
    #[schema(value_type = field::unowned::sample::TumorTissueMorphology, nullable = true)]
    tumor_tissue_morphology: Option<field::unowned::sample::TumorTissueMorphology>,

    /// The approximate age at collection.
    #[schema(value_type = field::unowned::sample::AgeAtCollection, nullable = true)]
    age_at_collection: Option<field::unowned::sample::AgeAtCollection>,

    /// The sequencing library technique type.
    #[schema(value_type = field::unowned::sample::LibraryStrategy, nullable = true)]
    library_strategy: Option<field::unowned::sample::LibraryStrategy>,

    /// The method used to maintain the sample or biospecimen in a viable state.
    #[schema(value_type = field::unowned::sample::PreservationMethod, nullable = true)]
    preservation_method: Option<field::unowned::sample::PreservationMethod>,

    /// The alternate identifiers for the sample.
    ///
    /// Note that this list of identifiers *must* include the main identifier
    /// for the [`Sample`].
    #[schema(value_type = Vec<field::unowned::sample::Identifier>, nullable = true)]
    identifiers: Option<Vec<field::unowned::sample::Identifier>>,

    /// Common metadata elements for all metadata blocks.
    #[schema(value_type = models::metadata::common::Metadata)]
    #[serde(flatten)]
    common: common::Metadata,

    /// An unharmonized map of metadata fields.
    #[schema(value_type = fields::Unharmonized)]
    #[serde(default, skip_serializing_if = "fields::Unharmonized::is_empty")]
    unharmonized: fields::Unharmonized,
}

impl Metadata {
    /// Gets the approximate age at diagnosis for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ordered_float::OrderedFloat;
    ///
    /// use models::metadata::field::unowned::sample::AgeAtDiagnosis;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .age_at_diagnosis(AgeAtDiagnosis::new(
    ///         models::sample::metadata::AgeAtDiagnosis::from(OrderedFloat(365.25)),
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.age_at_diagnosis(),
    ///     Some(&AgeAtDiagnosis::new(
    ///         models::sample::metadata::AgeAtDiagnosis::from(OrderedFloat(365.25)),
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn age_at_diagnosis(&self) -> Option<&field::unowned::sample::AgeAtDiagnosis> {
        self.age_at_diagnosis.as_ref()
    }

    /// Gets the diagnosis for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ordered_float::OrderedFloat;
    ///
    /// use models::metadata::field::unowned::sample::Diagnosis;
    /// use models::sample::metadata::Builder;
    ///
    /// let diagnosis =
    ///     models::sample::metadata::Diagnosis::from(String::from("Acute Lymphoblastic Leukemia"));
    ///
    /// let metadata = Builder::default()
    ///     .diagnosis(Diagnosis::new(diagnosis.clone(), None, None))
    ///     .build();
    ///
    /// assert_eq!(metadata.diagnosis().unwrap().value(), &diagnosis);
    /// ```
    pub fn diagnosis(&self) -> Option<&field::unowned::sample::Diagnosis> {
        self.diagnosis.as_ref()
    }

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
    ///     .disease_phase(DiseasePhase::new(
    ///         cde::v1::sample::DiseasePhase::InitialDiagnosis,
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.disease_phase(),
    ///     Some(&DiseasePhase::new(
    ///         cde::v1::sample::DiseasePhase::InitialDiagnosis,
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn disease_phase(&self) -> Option<&field::unowned::sample::DiseasePhase> {
        self.disease_phase.as_ref()
    }

    /// Gets the harmonized library strategy for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::LibraryStrategy;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .library_strategy(LibraryStrategy::new(
    ///         cde::v1::sample::LibraryStrategy::RnaSeq,
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.library_strategy(),
    ///     Some(&LibraryStrategy::new(
    ///         cde::v1::sample::LibraryStrategy::RnaSeq,
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn library_strategy(&self) -> Option<&field::unowned::sample::LibraryStrategy> {
        self.library_strategy.as_ref()
    }

    /// Gets the harmonized preservation method for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::PreservationMethod;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .preservation_method(PreservationMethod::new(
    ///         cde::v2::sample::PreservationMethod::Ffpe,
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.preservation_method(),
    ///     Some(&PreservationMethod::new(
    ///         cde::v2::sample::PreservationMethod::Ffpe,
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn preservation_method(&self) -> Option<&field::unowned::sample::PreservationMethod> {
        self.preservation_method.as_ref()
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
    ///     .tissue_type(TissueType::new(
    ///         cde::v2::sample::TissueType::Tumor,
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.tissue_type(),
    ///     Some(&TissueType::new(
    ///         cde::v2::sample::TissueType::Tumor,
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn tissue_type(&self) -> Option<&field::unowned::sample::TissueType> {
        self.tissue_type.as_ref()
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
    ///     .tumor_classification(TumorClassification::new(
    ///         cde::v1::sample::TumorClassification::Primary,
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.tumor_classification(),
    ///     Some(&TumorClassification::new(
    ///         cde::v1::sample::TumorClassification::Primary,
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn tumor_classification(&self) -> Option<&field::unowned::sample::TumorClassification> {
        self.tumor_classification.as_ref()
    }

    /// Gets the harmonized tumor tissue morphology code for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::TumorTissueMorphology;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .tumor_tissue_morphology(TumorTissueMorphology::new(
    ///         cde::v1::sample::TumorTissueMorphology::from(String::from("8010/0")),
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.tumor_tissue_morphology(),
    ///     Some(&TumorTissueMorphology::new(
    ///         cde::v1::sample::TumorTissueMorphology::from(String::from("8010/0")),
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn tumor_tissue_morphology(
        &self,
    ) -> Option<&field::unowned::sample::TumorTissueMorphology> {
        self.tumor_tissue_morphology.as_ref()
    }

    /// Gets the approximate age at collection for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ordered_float::OrderedFloat;
    ///
    /// use models::metadata::field::unowned::sample::AgeAtCollection;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .age_at_collection(AgeAtCollection::new(
    ///         models::sample::metadata::AgeAtCollection::from(OrderedFloat(365.25)),
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.age_at_collection(),
    ///     Some(&AgeAtCollection::new(
    ///         models::sample::metadata::AgeAtCollection::from(OrderedFloat(365.25)),
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn age_at_collection(&self) -> Option<&field::unowned::sample::AgeAtCollection> {
        self.age_at_collection.as_ref()
    }

    /// Gets the harmonized identifier(s) for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    ///     None,
    /// );
    ///
    /// let sample_id = models::sample::identifier::referenced::Identifier::Linked(
    ///     models::sample::identifier::linked::Identifier::new(
    ///         models::sample::Identifier::new(namespace.id().clone(), "SampleName001"),
    ///         "https://ccdi.example.com/api/v0"
    ///             .parse::<models::Url>()
    ///             .unwrap(),
    ///     ),
    /// );
    ///
    /// let field = Identifier::new(sample_id, None, None);
    /// let metadata = Builder::default().append_identifier(field.clone()).build();
    ///
    /// assert_eq!(metadata.identifiers(), Some(&vec![field]));
    /// ```
    pub fn identifiers(&self) -> Option<&Vec<field::unowned::sample::Identifier>> {
        self.identifiers.as_ref()
    }

    /// Gets the common metadata fields for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::common;
    /// use models::sample::metadata::Builder;
    ///
    /// let common = common::metadata::Builder::default().build();
    /// let metadata = Builder::default().common(common.clone()).build();
    ///
    /// assert_eq!(&common, metadata.common());
    /// ```
    pub fn common(&self) -> &common::Metadata {
        &self.common
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
    /// use models::metadata::field::owned;
    /// use models::metadata::field::unowned;
    /// use models::metadata::field::UnharmonizedField;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .insert_unharmonized(
    ///         "unowned",
    ///         UnharmonizedField::Unowned(unowned::Field::new(
    ///             Value::String("test".into()),
    ///             None,
    ///             None,
    ///         )),
    ///     )
    ///     .insert_unharmonized(
    ///         "owned",
    ///         UnharmonizedField::Owned(owned::Field::new(
    ///             Value::String("test".into()),
    ///             None,
    ///             None,
    ///             None,
    ///         )),
    ///     )
    ///     .build();
    ///
    /// assert!(matches!(
    ///     metadata.unharmonized().inner().get("unowned".into()),
    ///     Some(&UnharmonizedField::Unowned(_))
    /// ));
    /// assert!(matches!(
    ///     metadata.unharmonized().inner().get("owned".into()),
    ///     Some(&UnharmonizedField::Owned(_))
    /// ));
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
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::sample::Metadata;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    ///     None,
    /// );
    ///
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let metadata = Metadata::random(sample_id);
    /// ```
    pub fn random(identifier: Identifier) -> Metadata {
        let mut rng = thread_rng();

        Metadata {
            age_at_diagnosis: Some(field::unowned::sample::AgeAtDiagnosis::new(
                crate::sample::metadata::AgeAtDiagnosis::from(OrderedFloat(365.25)),
                None,
                None,
            )),
            diagnosis: Some(field::unowned::sample::Diagnosis::new(
                Diagnosis::from(String::from("Random Diagnosis")),
                None,
                None,
            )),
            disease_phase: rand::random(),
            library_strategy: rand::random(),
            preservation_method: rand::random(),
            tissue_type: rand::random(),
            tumor_classification: rand::random(),
            tumor_tissue_morphology: Some(field::unowned::sample::TumorTissueMorphology::new(
                // "8000/0" is the ICD-O-3 code for a "Neoplasm".
                ccdi_cde::v1::sample::TumorTissueMorphology::from(String::from("8000/0")),
                None,
                None,
            )),
            age_at_collection: Some(field::unowned::sample::AgeAtCollection::new(
                crate::sample::metadata::AgeAtCollection::from(OrderedFloat(365.25)),
                None,
                None,
            )),
            identifiers: Some(vec![
                field::unowned::sample::Identifier::new(
                    crate::sample::identifier::referenced::Identifier::Linked(
                        crate::sample::identifier::linked::Identifier::new(
                            identifier.clone(),
                            "https://ccdi.example.com/api/v0"
                                .parse::<crate::Url>()
                                .unwrap(),
                        ),
                    ),
                    None,
                    None,
                ),
                field::unowned::sample::Identifier::new(
                    crate::sample::identifier::referenced::Identifier::Unlinked(
                        crate::sample::identifier::unlinked::Identifier::from(format!(
                            "Sample-{}",
                            (0..8)
                                .map(|_| rng.sample(Alphanumeric).to_ascii_uppercase() as char)
                                .collect::<String>()
                        )),
                    ),
                    None,
                    None,
                ),
            ]),
            unharmonized: Default::default(),
            common: Default::default(),
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
            "{\"age_at_diagnosis\":null,\"diagnosis\":null,\"disease_phase\":null,\"tissue_type\":null,\"tumor_classification\":null,\"tumor_tissue_morphology\":null,\"age_at_collection\":null,\"library_strategy\":null,\"preservation_method\":null,\"identifiers\":null,\"depositions\":null}"
        );
    }
}
