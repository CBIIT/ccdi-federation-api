//! A builder for [`Metadata`].

use nonempty::NonEmpty;

use crate::metadata::common;
use crate::metadata::field;
use crate::metadata::fields;
use crate::namespace::Metadata;

/// A builder for [`Metadata`].
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The study short title.
    study_short_title: Option<field::unowned::namespace::StudyShortTitle>,

    /// The study name.
    study_name: Option<field::unowned::namespace::StudyName>,

    /// The study funding id.
    study_funding_id: Option<NonEmpty<field::unowned::namespace::StudyFundingId>>,

    /// The study id.
    study_id: Option<field::unowned::namespace::StudyId>,

    /// Common metadata elements for all metadata blocks.
    common: common::Metadata,

    /// An unharmonized map of metadata fields.
    unharmonized: fields::Unharmonized,
}

impl Builder {
    /// Sets the `study_short_title` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::namespace::StudyShortTitle;
    /// use models::namespace::metadata::Builder;
    ///
    /// let name = cde::v1::namespace::StudyShortTitle::from(String::from("A study short title"));
    /// let builder = Builder::default().study_short_title(StudyShortTitle::new(name, None, None));
    /// ```
    pub fn study_short_title(mut self, field: field::unowned::namespace::StudyShortTitle) -> Self {
        self.study_short_title = Some(field);
        self
    }

    /// Sets the `study_name` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::namespace::StudyName;
    /// use models::namespace::metadata::Builder;
    ///
    /// let name = cde::v1::namespace::StudyName::from(String::from("A study name"));
    /// let builder = Builder::default().study_name(StudyName::new(name, None, None));
    /// ```
    pub fn study_name(mut self, field: field::unowned::namespace::StudyName) -> Self {
        self.study_name = Some(field);
        self
    }

    /// Sets the `study_funding_id` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::namespace::StudyFundingId;
    /// use models::namespace::metadata::Builder;
    ///
    /// let name = cde::v1::namespace::StudyFundingId::from(String::from("A study funding id"));
    /// let builder = Builder::default()
    ///     .push_study_funding_id(StudyFundingId::new(name.clone(), None, None))
    ///     .push_study_funding_id(StudyFundingId::new(name, None, None));
    /// ```
    pub fn push_study_funding_id(
        mut self,
        field: field::unowned::namespace::StudyFundingId,
    ) -> Self {
        let study_funding_id = match self.study_funding_id {
            Some(mut ids) => {
                ids.push(field);
                ids
            }
            None => NonEmpty::new(field),
        };

        self.study_funding_id = Some(study_funding_id);
        self
    }

    /// Sets the `study_id` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::namespace::StudyId;
    /// use models::namespace::metadata::Builder;
    ///
    /// let name = cde::v1::namespace::StudyId::AALL0232;
    /// let builder = Builder::default().study_id(StudyId::new(name, None, None));
    /// ```
    pub fn study_id(mut self, field: field::unowned::namespace::StudyId) -> Self {
        self.study_id = Some(field);
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
    /// use models::namespace::metadata::Builder;
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

    /// Sets the common metadata for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::common;
    /// use models::namespace::metadata::Builder;
    ///
    /// let common = common::metadata::Builder::default().build();
    /// let builder = Builder::default().common(common);
    /// ```
    pub fn common(mut self, common: common::Metadata) -> Self {
        self.common = common;
        self
    }

    /// Consumes `self` to build a [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace::metadata::Builder;
    ///
    /// let builder = Builder::default();
    /// ```
    pub fn build(self) -> Metadata {
        Metadata {
            study_short_title: self.study_short_title,
            study_name: self.study_name,
            study_funding_id: self.study_funding_id,
            study_id: self.study_id,
            common: self.common,
            unharmonized: self.unharmonized,
        }
    }
}
