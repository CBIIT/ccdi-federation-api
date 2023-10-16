//! A builder for [`Metadata`].

use crate::metadata::field;
use crate::metadata::fields;
use crate::subject::Metadata;

/// A builder for [`Metadata`].
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The sex of the subject.
    sex: Option<field::Sex>,

    /// The race of the subject.
    race: Option<Vec<field::Race>>,

    /// The ethnicity of the subject.
    ethnicity: Option<field::Ethnicity>,

    /// The identifiers for the subject.
    identifiers: Option<Vec<field::Identifier>>,

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
    /// use models::metadata::field::Sex;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Sex::new(cde::v1::Sex::Unknown, None, None);
    /// let builder = Builder::default().sex(field);
    /// ```
    pub fn sex(mut self, sex: field::Sex) -> Self {
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
    /// use models::metadata::field::Race;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Race::new(cde::v1::Race::Unknown, None, None);
    /// let builder = Builder::default().append_race(field);
    /// ```
    pub fn append_race(mut self, race: field::Race) -> Self {
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
    /// use models::metadata::field::Ethnicity;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Ethnicity::new(cde::v2::Ethnicity::Unknown, None, None);
    /// let builder = Builder::default().ethnicity(field);
    /// ```
    pub fn ethnicity(mut self, ethnicity: field::Ethnicity) -> Self {
        self.ethnicity = Some(ethnicity);
        self
    }

    /// Append a value to the `identifier` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::Identifier;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Identifier::new(
    ///     cde::v1::Identifier::parse("organization:Name", ":")?,
    ///     None,
    ///     None,
    ///     None
    /// );
    ///
    /// let builder = Builder::default().append_identifier(field);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn append_identifier(mut self, identifier: field::Identifier) -> Self {
        let mut inner = self.identifiers.unwrap_or_default();
        inner.push(identifier);

        self.identifiers = Some(inner);

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
    /// use models::metadata::field::Identifier;
    /// use models::metadata::field::UnharmonizedField;
    /// use models::metadata::field::owned;
    /// use models::metadata::field::unowned;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Identifier::new(
    ///     cde::v1::Identifier::parse("organization:Name", ":")?,
    ///     None,
    ///     None,
    ///     None
    /// );
    ///
    /// let builder = Builder::default()
    ///                         .insert_unharmonized(
    ///                             "unowned",
    ///                             UnharmonizedField::Unowned(unowned::Field::new(Value::String("test".into()), None, None))
    ///                         )
    ///                         .insert_unharmonized(
    ///                             "owned",
    ///                             UnharmonizedField::Owned(owned::Field::new(Value::String("test".into()), None, None, None))
    ///                         );
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
            unharmonized: self.unharmonized,
        }
    }
}
