//! A builder for [`Metadata`].

use crate::metadata::common;
use crate::metadata::field;
use crate::metadata::fields;
use crate::sample::Metadata;

/// A builder for [`Metadata`].
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The approximate age at diagnosis.
    age_at_diagnosis: Option<field::unowned::sample::AgeAtDiagnosis>,

    /// The diagnosis for the sample.
    diagnosis: Option<field::unowned::sample::Diagnosis>,

    /// The phase of the disease when this sample was acquired.
    disease_phase: Option<field::unowned::sample::DiseasePhase>,

    /// The type of tissue for this sample.
    tissue_type: Option<field::unowned::sample::TissueType>,

    /// The classification for this tumor based mainly on histological
    /// characteristics.
    tumor_classification: Option<field::unowned::sample::TumorClassification>,

    /// The ICD-O-3 morphology code for the tumor tissue.
    tumor_tissue_morphology: Option<field::unowned::sample::TumorTissueMorphology>,

    /// The approximate age at collection.
    age_at_collection: Option<field::unowned::sample::AgeAtCollection>,

    /// The strategy for constructing the sequencing library.
    library_strategy: Option<field::unowned::sample::LibraryStrategy>,

    /// The preservation method for this sample or biospecimen.
    preservation_method: Option<field::unowned::sample::PreservationMethod>,

    /// The alternate identifiers for the sample.
    identifiers: Option<Vec<field::unowned::sample::Identifier>>,

    /// Common metadata elements for all metadata blocks.
    common: common::Metadata,

    /// An unharmonized map of metadata fields.
    unharmonized: fields::Unharmonized,
}

impl Builder {
    /// Sets the `age_at_diagnosis` field of the [`Builder`].
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
    /// let field = AgeAtDiagnosis::new(
    ///     models::sample::metadata::AgeAtDiagnosis::from(OrderedFloat(365.25)),
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default().age_at_diagnosis(field);
    /// ```
    pub fn age_at_diagnosis(mut self, field: field::unowned::sample::AgeAtDiagnosis) -> Self {
        self.age_at_diagnosis = Some(field);
        self
    }

    /// Sets the `diagnosis` field of the [`Builder`].
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
    /// let builder = Builder::default().diagnosis(Diagnosis::new(diagnosis.clone(), None, None, None));
    /// ```
    pub fn diagnosis(mut self, field: field::unowned::sample::Diagnosis) -> Self {
        self.diagnosis = Some(field);
        self
    }

    /// Sets the `disease_phase` field of the [`Builder`].
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
    /// let field = DiseasePhase::new(
    ///     cde::v1::sample::DiseasePhase::InitialDiagnosis,
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default().disease_phase(field);
    /// ```
    pub fn disease_phase(mut self, field: field::unowned::sample::DiseasePhase) -> Self {
        self.disease_phase = Some(field);
        self
    }

    /// Sets the `tissue_type` field of the [`Builder`].
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
    /// let field = TissueType::new(cde::v2::sample::TissueType::Tumor, None, None, None);
    /// let builder = Builder::default().tissue_type(field);
    /// ```
    pub fn tissue_type(mut self, field: field::unowned::sample::TissueType) -> Self {
        self.tissue_type = Some(field);
        self
    }

    /// Sets the `tumor_classification` field of the [`Builder`].
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
    /// let field = TumorClassification::new(
    ///     cde::v1::sample::TumorClassification::Primary,
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default().tumor_classification(field);
    /// ```
    pub fn tumor_classification(
        mut self,
        field: field::unowned::sample::TumorClassification,
    ) -> Self {
        self.tumor_classification = Some(field);
        self
    }

    /// Sets the `tumor_tissue_morphology` field of the [`Builder`].
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
    /// let field = TumorTissueMorphology::new(
    ///     cde::v1::sample::TumorTissueMorphology::from(String::from("8010/0")),
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default().tumor_tissue_morphology(field);
    /// ```
    pub fn tumor_tissue_morphology(
        mut self,
        field: field::unowned::sample::TumorTissueMorphology,
    ) -> Self {
        self.tumor_tissue_morphology = Some(field);
        self
    }

    /// Sets the `age_at_collection` field of the [`Builder`].
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
    /// let field = AgeAtCollection::new(
    ///     models::sample::metadata::AgeAtCollection::from(OrderedFloat(365.25)),
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default().age_at_collection(field);
    /// ```
    pub fn age_at_collection(mut self, field: field::unowned::sample::AgeAtCollection) -> Self {
        self.age_at_collection = Some(field);
        self
    }

    /// Sets the `library_strategy` field of the [`Builder`].
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::LibraryStrategy;
    /// use models::sample::metadata::Builder;
    ///
    /// let field = LibraryStrategy::new(cde::v1::sample::LibraryStrategy::RnaSeq, None, None, None);
    /// let builder = Builder::default().library_strategy(field);
    /// ```
    pub fn library_strategy(mut self, field: field::unowned::sample::LibraryStrategy) -> Self {
        self.library_strategy = Some(field);
        self
    }

    /// Sets the `preservation_method` field of the [`Builder`].
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
    /// let field = PreservationMethod::new(
    ///     cde::v2::sample::PreservationMethod::Unknown,
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default().preservation_method(field);
    /// ```
    pub fn preservation_method(
        mut self,
        field: field::unowned::sample::PreservationMethod,
    ) -> Self {
        self.preservation_method = Some(field);
        self
    }

    /// Append a value to the `identifier` field of the [`Builder`].
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
    /// let field = Identifier::new(sample_id, None, None, None);
    /// let builder = Builder::default().append_identifier(field);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn append_identifier(mut self, identifier: field::unowned::sample::Identifier) -> Self {
        let mut inner = self.identifiers.unwrap_or_default();
        inner.push(identifier);

        self.identifiers = Some(inner);

        self
    }

    /// Sets the common metadata for the [`Metadata`].
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
    /// let builder = Builder::default().common(common);
    /// ```
    pub fn common(mut self, common: common::Metadata) -> Self {
        self.common = common;
        self
    }

    /// Inserts an [`UnharmonizedField`](field::UnharmonizedField) into the
    /// `unharmonized` map.
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
    /// let builder = Builder::default()
    ///     .insert_unharmonized(
    ///         "unowned",
    ///         UnharmonizedField::Unowned(unowned::Field::new(
    ///             Value::String("test".into()),
    ///             None,
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
    ///             None,
    ///         )),
    ///     );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn insert_unharmonized<S: Into<String>>(
        mut self,
        key: S,
        field: field::UnharmonizedField,
    ) -> Self {
        let key = key.into();

        let mut unharmonized = self.unharmonized;
        unharmonized.inner_mut().insert(key, field);

        self.unharmonized = unharmonized;

        self
    }

    /// Consumes `self` to build a [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::sample::metadata::Builder;
    ///
    /// let builder = Builder::default();
    /// ```
    pub fn build(self) -> Metadata {
        Metadata {
            age_at_diagnosis: self.age_at_diagnosis,
            age_at_collection: self.age_at_collection,
            diagnosis: self.diagnosis,
            disease_phase: self.disease_phase,
            library_strategy: self.library_strategy,
            preservation_method: self.preservation_method,
            tissue_type: self.tissue_type,
            tumor_classification: self.tumor_classification,
            tumor_tissue_morphology: self.tumor_tissue_morphology,
            identifiers: self.identifiers,
            unharmonized: self.unharmonized,
            common: self.common,
        }
    }
}
