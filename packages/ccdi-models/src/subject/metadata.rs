//! Metadata for a [`Subject`](super::Subject).

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::field;

use ccdi_cde as cde;

pub mod builder;

pub use builder::Builder;

/// Metadata associated with a subject.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::subject::Metadata)]
pub struct Metadata {
    /// The sex of the subject.
    #[schema(value_type = field::Sex, nullable = true)]
    sex: Option<field::Sex>,

    /// The race(s) of the subject.
    #[schema(value_type = Vec<field::Race>, nullable = true)]
    race: Option<Vec<field::Race>>,

    /// The ethnicity of the subject.
    #[schema(value_type = field::Ethnicity, nullable = true)]
    ethnicity: Option<field::Ethnicity>,

    /// The identifiers for the subject.
    ///
    /// Note that this list of identifiers *must* include the main identifier
    /// for the [`Subject`].
    #[schema(value_type = Vec<field::Identifier>, nullable = true)]
    identifiers: Option<Vec<field::Identifier>>,
}

impl Metadata {
    /// Gets the harmonized sex for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///                 .sex(Sex::new(cde::v1::Sex::Female, None, None))
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.sex(),
    ///     &Some(Sex::new(cde::v1::Sex::Female, None, None))
    /// );
    /// ```
    pub fn sex(&self) -> &Option<field::Sex> {
        &self.sex
    }

    /// Gets the harmonized race(s) for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///                 .append_race(Race::new(cde::v1::Race::Asian, None, None))
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.race(),
    ///     &Some(vec![Race::new(cde::v1::Race::Asian, None, None)])
    /// );
    /// ```
    pub fn race(&self) -> &Option<Vec<field::Race>> {
        &self.race
    }

    /// Gets the harmonized ethnicity for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///                 .ethnicity(Ethnicity::new(cde::v2::Ethnicity::NotHispanicOrLatino, None, None))
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.ethnicity(),
    ///     &Some(Ethnicity::new(cde::v2::Ethnicity::NotHispanicOrLatino, None, None))
    /// );
    /// ```
    pub fn ethnicity(&self) -> &Option<field::Ethnicity> {
        &self.ethnicity
    }

    /// Gets the harmonized identifier(s) for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///                 .append_identifier(
    ///                     Identifier::new(
    ///                         cde::v1::Identifier::parse("organization:Name", ":").unwrap(),
    ///                         None, None, Some(true)
    ///                     )
    ///                 )
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.identifiers(),
    ///     &Some(
    ///         vec![
    ///             Identifier::new(
    ///                 cde::v1::Identifier::parse("organization:Name", ":").unwrap(),
    ///                 None, None, Some(true)
    ///             )
    ///         ]
    ///     )
    /// );
    /// ```
    pub fn identifiers(&self) -> &Option<Vec<field::Identifier>> {
        &self.identifiers
    }

    /// Generates a random [`Metadata`] based on a particular [`Identifier`](cde::v1::Identifier).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::subject::Metadata;
    ///
    /// let identifier = cde::v1::Identifier::parse("organization:Name", ":").unwrap();
    /// let metadata = Metadata::random(identifier);
    /// ```
    pub fn random(identifier: cde::v1::Identifier) -> Metadata {
        Metadata {
            sex: Some(rand::random()),
            race: Some(vec![rand::random()]),
            ethnicity: Some(rand::random()),
            identifiers: Some(vec![field::owned::Identifier::new(
                identifier,
                None,
                None,
                Some(true),
            )]),
        }
    }
}
