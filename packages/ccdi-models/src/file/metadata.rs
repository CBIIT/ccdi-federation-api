//! Metadata for a [`File`](super::File).

use ccdi_cde as cde;
use rand::Rng as _;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::common;
use crate::metadata::field;
use crate::metadata::fields;

pub mod builder;
mod checksums;

pub use builder::Builder;
pub use checksums::Checksums;

/// Metadata associated with a file.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::file::Metadata)]
pub struct Metadata {
    /// The type of the file.
    #[schema(value_type = field::unowned::file::Type, nullable = true)]
    r#type: Option<field::unowned::file::Type>,

    /// The size of the file in bytes.
    #[schema(value_type = field::unowned::file::Size, nullable = true)]
    size: Option<field::unowned::file::Size>,

    /// A set of checksums for the file.
    #[schema(value_type = field::unowned::file::Checksums, nullable = true)]
    checksums: Option<field::unowned::file::Checksums>,

    /// A free-text description of the file.
    #[schema(value_type = field::unowned::file::Description, nullable = true)]
    description: Option<field::unowned::file::Description>,

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
    /// Gets the type for the [`Metadata`].
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
    /// let metadata = Builder::default().r#type(field).build();
    ///
    /// assert_eq!(
    ///     metadata.r#type().unwrap().value(),
    ///     &cde::v1::file::Type::TXT
    /// );
    /// ```
    pub fn r#type(&self) -> Option<&field::unowned::file::Type> {
        self.r#type.as_ref()
    }

    /// Gets the size for the [`Metadata`].
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
    /// let metadata = Builder::default().size(field).build();
    ///
    /// assert_eq!(
    ///     metadata.size().unwrap().value(),
    ///     &cde::v1::file::Size::new(42)
    /// );
    /// ```
    pub fn size(&self) -> Option<&field::unowned::file::Size> {
        self.size.as_ref()
    }

    /// Gets the checksums for the [`Metadata`].
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
    /// let metadata = Builder::default().checksums(field).build();
    ///
    /// assert_eq!(
    ///     metadata.checksums().unwrap().value().md5().unwrap().inner(),
    ///     "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
    /// );
    /// ```
    pub fn checksums(&self) -> Option<&field::unowned::file::Checksums> {
        self.checksums.as_ref()
    }

    /// Gets the description for the [`Metadata`].
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
    /// let metadata = Builder::default().description(field).build();
    ///
    /// assert_eq!(
    ///     metadata.description().unwrap().value().inner(),
    ///     "This is a description."
    /// );
    /// ```
    pub fn description(&self) -> Option<&field::unowned::file::Description> {
        self.description.as_ref()
    }

    /// Gets the common metadata fields for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::metadata::Builder;
    /// use models::metadata::common;
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
    /// use models::file::metadata::Builder;
    /// use models::metadata::field::owned;
    /// use models::metadata::field::unowned;
    /// use models::metadata::field::UnharmonizedField;
    ///
    /// let metadata = Builder::default()
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
    /// use models::file::Metadata;
    ///
    /// let metadata = Metadata::random();
    /// ```
    pub fn random() -> Metadata {
        Metadata {
            r#type: Some(field::unowned::file::Type::new(
                cde::v1::file::Type::TXT,
                None,
                None,
                None,
            )),
            size: Some(field::unowned::file::Size::new(
                cde::v1::file::Size::new(rand::thread_rng().gen_range(u64::MIN..=u64::MAX)),
                None,
                None,
                None,
            )),
            checksums: Some(rand::random()),
            description: Some(field::unowned::file::Description::new(
                cde::v1::file::Description::new("This is an example description."),
                None,
                None,
                None,
            )),
            common: Default::default(),
            unharmonized: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file::metadata::builder;

    #[test]
    fn it_skips_serializing_the_unharmonized_key_when_it_is_empty() {
        let metadata = builder::Builder::default().build();
        assert_eq!(
            &serde_json::to_string(&metadata).unwrap(),
            "{\"type\":null,\"size\":null,\"checksums\":null,\"description\":null,\"depositions\":null}",
        );
    }
}
