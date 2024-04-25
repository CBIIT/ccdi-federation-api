//! A builder for [`Metadata`].

use nonempty::NonEmpty;

use crate::metadata::common;
use crate::metadata::field;
use crate::metadata::fields;
use crate::organization::Metadata;

/// A builder for [`Metadata`].
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// Institutions associated with an organization.
    institution: Option<NonEmpty<field::unowned::organization::Institution>>,

    /// Common metadata elements for all metadata blocks.
    common: common::Metadata,

    /// An unharmonized map of metadata fields.
    unharmonized: fields::Unharmonized,
}

impl Builder {
    /// Sets the `institution` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::organization::Institution;
    /// use models::organization::metadata::Builder;
    ///
    /// let field = Institution::new(
    ///     cde::v1::organization::Institution::Treehouse,
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default()
    ///     .push_institution(field.clone())
    ///     .push_institution(field);
    /// ```
    pub fn push_institution(mut self, field: field::unowned::organization::Institution) -> Self {
        let institution = match self.institution {
            Some(mut institutions) => {
                institutions.push(field);
                institutions
            }
            None => NonEmpty::new(field),
        };

        self.institution = Some(institution);
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
    /// use models::organization::metadata::Builder;
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
    /// use models::organization::metadata::Builder;
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
    ///             Some(true),
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
    /// use models::organization::metadata::Builder;
    ///
    /// let builder = Builder::default().build();
    /// ```
    pub fn build(self) -> Metadata {
        Metadata {
            institution: self.institution,
            common: self.common,
            unharmonized: self.unharmonized,
        }
    }
}
