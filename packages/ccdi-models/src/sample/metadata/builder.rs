//! A builder for [`Metadata`].

use crate::metadata::field;
use crate::metadata::fields;
use crate::sample::Metadata;

/// A builder for [`Metadata`].
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The phase of the disease when this sample was acquired.
    disease_phase: Option<field::unowned::sample::DiseasePhase>,

    /// The type of tissue for this sample.
    tissue_type: Option<field::unowned::sample::TissueType>,

    /// The classification for this tumor based mainly on histological
    /// characteristics.
    tumor_classification: Option<field::unowned::sample::TumorClassification>,

    /// An unharmonized map of metadata fields.
    unharmonized: fields::Unharmonized,
}

impl Builder {
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
    /// let field = DiseasePhase::new(cde::v1::sample::DiseasePhase::InitialDiagnosis, None, None);
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
    /// let field = TissueType::new(cde::v2::sample::TissueType::Tumor, None, None);
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
    /// let field = TumorClassification::new(cde::v1::sample::TumorClassification::Primary, None, None);
    /// let builder = Builder::default().tumor_classification(field);
    /// ```
    pub fn tumor_classification(
        mut self,
        field: field::unowned::sample::TumorClassification,
    ) -> Self {
        self.tumor_classification = Some(field);
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
            disease_phase: self.disease_phase,
            tissue_type: self.tissue_type,
            tumor_classification: self.tumor_classification,
            unharmonized: self.unharmonized,
        }
    }
}
