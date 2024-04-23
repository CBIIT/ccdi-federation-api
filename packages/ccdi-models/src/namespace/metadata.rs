//! Metadata for a [`Namespace`](super::Namespace).

use nonempty::NonEmpty;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::common;
use crate::metadata::field;
use crate::metadata::fields;

mod builder;

pub use builder::Builder;

/// Metadata associated with a namespace.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::namespace::Metadata)]
pub struct Metadata {
    /// The study short title.
    #[schema(value_type = field::unowned::namespace::StudyShortTitle, nullable = true)]
    study_short_title: Option<field::unowned::namespace::StudyShortTitle>,

    /// The study name.
    #[schema(value_type = field::unowned::namespace::StudyName, nullable = true)]
    study_name: Option<field::unowned::namespace::StudyName>,

    /// The study funding id.
    #[schema(value_type = Vec<field::unowned::namespace::StudyFundingId>, nullable = true)]
    study_funding_id: Option<NonEmpty<field::unowned::namespace::StudyFundingId>>,

    /// The study id.
    #[schema(value_type = field::unowned::namespace::StudyId, nullable = true)]
    study_id: Option<field::unowned::namespace::StudyId>,

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
    /// Gets the study short title for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///     .study_short_title(StudyShortTitle::new(name.clone(), None, None))
    ///     .build();
    ///
    /// assert_eq!(metadata.study_short_title().unwrap().value(), &name);
    /// ```
    pub fn study_short_title(&self) -> Option<&field::unowned::namespace::StudyShortTitle> {
        self.study_short_title.as_ref()
    }

    /// Gets the study name for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///     .study_name(StudyName::new(name.clone(), None, None))
    ///     .build();
    ///
    /// assert_eq!(metadata.study_name().unwrap().value(), &name);
    /// ```
    pub fn study_name(&self) -> Option<&field::unowned::namespace::StudyName> {
        self.study_name.as_ref()
    }

    /// Gets the study funding id for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///     .push_study_funding_id(StudyFundingId::new(name.clone(), None, None))
    ///     .push_study_funding_id(StudyFundingId::new(name.clone(), None, None))
    ///     .build();
    ///
    /// let study_funding_id = metadata.study_funding_id().cloned().unwrap();
    /// assert_eq!(study_funding_id.len(), 2);
    ///
    /// let mut study_funding_id = study_funding_id.into_iter();
    /// assert_eq!(study_funding_id.next().unwrap().value(), &name);
    /// assert_eq!(study_funding_id.next().unwrap().value(), &name);
    /// ```
    pub fn study_funding_id(&self) -> Option<&NonEmpty<field::unowned::namespace::StudyFundingId>> {
        self.study_funding_id.as_ref()
    }

    /// Gets the study id for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///     .study_id(StudyId::new(name.clone(), None, None))
    ///     .build();
    ///
    /// assert_eq!(metadata.study_id().unwrap().value(), &name);
    /// ```
    pub fn study_id(&self) -> Option<&field::unowned::namespace::StudyId> {
        self.study_id.as_ref()
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
    /// use models::namespace::metadata::Builder;
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
    /// use models::namespace::metadata::Builder;
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
    use crate::namespace::metadata::builder;

    #[test]
    fn it_skips_serializing_the_unharmonized_key_when_it_is_empty() {
        let metadata = builder::Builder::default().build();
        assert_eq!(
            &serde_json::to_string(&metadata).unwrap(),
            "{\"study_short_title\":null,\"study_name\":null,\"study_funding_id\":null,\"study_id\":null,\"depositions\":null}"
        );
    }
}
