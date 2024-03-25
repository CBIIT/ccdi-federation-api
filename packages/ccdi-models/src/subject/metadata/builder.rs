//! A builder for [`Metadata`].

use crate::metadata::field;
use crate::metadata::fields;
use crate::subject::Metadata;

/// A builder for [`Metadata`].
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The sex of the subject.
    sex: Option<field::unowned::subject::Sex>,

    /// The race of the subject.
    race: Option<Vec<field::unowned::subject::Race>>,

    /// The ethnicity of the subject.
    ethnicity: Option<field::unowned::subject::Ethnicity>,

    /// The alternate identifiers for the subject.
    identifiers: Option<Vec<field::unowned::subject::Identifier>>,

    /// The vital status for the subject.
    vital_status: Option<field::unowned::subject::VitalStatus>,

    /// The approximate age at vital status.
    age_at_vital_status: Option<field::unowned::subject::AgeAtVitalStatus>,

    /// An unharmonized map of metadata fields.
    unharmonized: fields::Unharmonized,
}

impl Builder {
    /// Sets the `sex` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Sex;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Sex::new(cde::v1::subject::Sex::Unknown, None, None);
    /// let builder = Builder::default().sex(field);
    /// ```
    pub fn sex(mut self, sex: field::unowned::subject::Sex) -> Self {
        self.sex = Some(sex);
        self
    }

    /// Append a value to the `race` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Race;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Race::new(cde::v1::subject::Race::Unknown, None, None);
    /// let builder = Builder::default().append_race(field);
    /// ```
    pub fn append_race(mut self, race: field::unowned::subject::Race) -> Self {
        let mut inner = self.race.unwrap_or_default();
        inner.push(race);

        self.race = Some(inner);

        self
    }

    /// Sets the `ethnicity` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Ethnicity;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Ethnicity::new(cde::v2::subject::Ethnicity::Unknown, None, None);
    /// let builder = Builder::default().ethnicity(field);
    /// ```
    pub fn ethnicity(mut self, ethnicity: field::unowned::subject::Ethnicity) -> Self {
        self.ethnicity = Some(ethnicity);
        self
    }

    /// Append a value to the `identifier` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::subject::metadata::Builder;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization",
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
    /// );
    ///
    /// let subject_id = models::subject::identifier::referenced::Identifier::Linked(
    ///     models::subject::identifier::linked::Identifier::new(
    ///         models::subject::Identifier::new(namespace.id().clone(), "SubjectName001"),
    ///         "https://ccdi.example.com/api/v0"
    ///             .parse::<models::Url>()
    ///             .unwrap(),
    ///     ),
    /// );
    ///
    /// let field = Identifier::new(subject_id, None, None);
    /// let builder = Builder::default().append_identifier(field);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn append_identifier(mut self, identifier: field::unowned::subject::Identifier) -> Self {
        let mut inner = self.identifiers.unwrap_or_default();
        inner.push(identifier);

        self.identifiers = Some(inner);

        self
    }

    /// Sets the `vital_status` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::VitalStatus;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = VitalStatus::new(cde::v1::subject::VitalStatus::Unknown, None, None);
    /// let builder = Builder::default().vital_status(field);
    /// ```
    pub fn vital_status(mut self, vital_status: field::unowned::subject::VitalStatus) -> Self {
        self.vital_status = Some(vital_status);
        self
    }

    /// Sets the `age_at_vital_status` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ordered_float::OrderedFloat;
    ///
    /// use models::metadata::field::unowned::subject::AgeAtVitalStatus;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = AgeAtVitalStatus::new(
    ///     models::subject::metadata::AgeAtVitalStatus::from(OrderedFloat(365.25)),
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default().age_at_vital_status(field);
    /// ```
    pub fn age_at_vital_status(
        mut self,
        age_at_vital_status: field::unowned::subject::AgeAtVitalStatus,
    ) -> Self {
        self.age_at_vital_status = Some(age_at_vital_status);
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
    /// use models::subject::metadata::Builder;
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
    /// use models::subject::metadata::Builder;
    ///
    /// let builder = Builder::default();
    /// ```
    pub fn build(self) -> Metadata {
        Metadata {
            sex: self.sex,
            race: self.race,
            ethnicity: self.ethnicity,
            identifiers: self.identifiers,
            vital_status: self.vital_status,
            age_at_vital_status: self.age_at_vital_status,
            unharmonized: self.unharmonized,
        }
    }
}
