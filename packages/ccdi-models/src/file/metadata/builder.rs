//! A builder for [`Metadata`].

use crate::file::Metadata;
use crate::metadata::field;
use crate::metadata::fields;

/// A builder for [`Metadata`].
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The type of the file.
    r#type: Option<field::unowned::file::Type>,

    /// The size of the file in bytes.
    size: Option<field::unowned::file::Size>,

    /// A set of checksums for the file.
    checksums: Option<field::unowned::file::Checksums>,

    /// A free-text description of the file.
    description: Option<field::unowned::file::Description>,

    /// An unharmonized map of metadata fields.
    unharmonized: fields::Unharmonized,
}

impl Builder {
    /// Sets the `type` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::metadata::Builder;
    /// use models::metadata::field::unowned::file::Type;
    ///
    /// let field = Type::new(cde::v1::file::Type::TXT, None, None, None);
    /// let builder = Builder::default().r#type(field);
    /// ```
    pub fn r#type(mut self, field: field::unowned::file::Type) -> Self {
        self.r#type = Some(field);
        self
    }

    /// Sets the `size` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::metadata::Builder;
    /// use models::metadata::field::unowned::file::Size;
    ///
    /// let field = Size::new(cde::v1::file::Size::new(42), None, None, None);
    /// let builder = Builder::default().size(field);
    /// ```
    pub fn size(mut self, field: field::unowned::file::Size) -> Self {
        self.size = Some(field);
        self
    }

    /// Sets the `checksums` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::metadata::Builder;
    /// use models::metadata::field::unowned::file::Checksums;
    ///
    /// let md5 = cde::v1::file::checksum::MD5::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    /// let field = Checksums::new(
    ///     models::file::metadata::Checksums::new(Some(md5)),
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default().checksums(field);
    /// ```
    pub fn checksums(mut self, field: field::unowned::file::Checksums) -> Self {
        self.checksums = Some(field);
        self
    }

    /// Sets the `description` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::metadata::Builder;
    /// use models::metadata::field::unowned::file::Description;
    ///
    /// let field = Description::new(
    ///     cde::v1::file::Description::new("This is a description."),
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// let builder = Builder::default().description(field);
    /// ```
    pub fn description(mut self, field: field::unowned::file::Description) -> Self {
        self.description = Some(field);
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
    /// use models::file::metadata::Builder;
    /// use models::metadata::field::owned;
    /// use models::metadata::field::unowned;
    /// use models::metadata::field::UnharmonizedField;
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
    /// use models::file::metadata::Builder;
    ///
    /// let builder = Builder::default().build();
    /// ```
    pub fn build(self) -> Metadata {
        Metadata {
            r#type: self.r#type,
            size: self.size,
            checksums: self.checksums,
            description: self.description,
            unharmonized: self.unharmonized,
        }
    }
}
