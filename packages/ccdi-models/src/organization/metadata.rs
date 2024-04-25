//! Metadata for an [`Organization`](super::Organization).

use nonempty::NonEmpty;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::common;
use crate::metadata::field;
use crate::metadata::fields;

mod builder;

pub use builder::Builder;

/// Metadata associated with an organization.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::organization::Metadata)]
pub struct Metadata {
    /// Institutions associated with an organization.
    ///
    /// **NOTE:** in this design, an organization in not always a single
    /// institution—it may also represent a consortium of institutions, for
    /// instance. This is necessary, since a namespace can be tied to one and
    /// only one organization in the API specification. As such, if the above is
    /// not true, there is no way to make a namespace where data is contributed
    /// from multiple institutions.
    #[schema(value_type = Vec<field::unowned::organization::Institution>, nullable = true)]
    institution: Option<NonEmpty<field::unowned::organization::Institution>>,

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
    /// Gets the institution(s) from the [`Metadata`].
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
    /// let field = Institution::new(cde::v1::organization::Institution::Treehouse, None, None);
    /// let metadata = Builder::default()
    ///     .push_institution(field.clone())
    ///     .push_institution(field.clone())
    ///     .build();
    ///
    /// let institution = metadata.institution().cloned().unwrap();
    /// assert_eq!(institution.len(), 2);
    ///
    /// let mut institution = institution.into_iter();
    /// assert_eq!(institution.next().unwrap(), field.clone());
    /// assert_eq!(institution.next().unwrap(), field);
    /// ```
    pub fn institution(&self) -> Option<&NonEmpty<field::unowned::organization::Institution>> {
        self.institution.as_ref()
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
    /// use models::organization::metadata::Builder;
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
}

#[cfg(test)]
mod tests {
    use crate::organization::metadata::builder;

    #[test]
    fn it_skips_serializing_the_unharmonized_key_when_it_is_empty() {
        let metadata = builder::Builder::default().build();
        assert_eq!(
            &serde_json::to_string(&metadata).unwrap(),
            "{\"institution\":null,\"depositions\":null}",
        );
    }
}
