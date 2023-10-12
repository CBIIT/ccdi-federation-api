//! Metadata for a [`Subject`](super::Subject).

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::field;
use crate::metadata::field::EthnicityOrNull;
use crate::metadata::field::IdentifiersOrNull;
use crate::metadata::field::RacesOrNull;
use crate::metadata::field::SexOrNull;

use ccdi_cde as cde;

pub mod builder;

pub use builder::Builder;

/// Metadata associated with a subject.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::subject::Metadata)]
pub struct Metadata {
    /// The sex of the subject.
    #[schema(value_type = field::SexOrNull, nullable = true)]
    sex: SexOrNull,

    /// The race(s) of the subject.
    #[schema(value_type = field::RacesOrNull, nullable = true)]
    race: RacesOrNull,

    /// The ethnicity of the subject.
    #[schema(value_type = field::EthnicityOrNull, nullable = true)]
    ethnicity: EthnicityOrNull,

    /// The identifiers for the subject.
    ///
    /// Note that this list of identifiers *must* include the main identifier
    /// for the [`Subject`].
    #[schema(value_type = field::IdentifiersOrNull, nullable = true)]
    identifiers: IdentifiersOrNull,
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
    /// use models::metadata::field::SexOrNull;
    /// use models::metadata::field::unowned::Sex;
    /// use models::subject::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///                 .sex(Sex::new(cde::v1::Sex::Female, None, None))
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.sex(),
    ///     &SexOrNull::Unowned(
    ///         Sex::new(cde::v1::Sex::Female, None, None)
    ///     )
    /// );
    /// ```
    pub fn sex(&self) -> &SexOrNull {
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
    /// use models::metadata::field::RacesOrNull;
    /// use models::metadata::field::unowned::Race;
    /// use models::subject::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///                 .append_race(Race::new(cde::v1::Race::Asian, None, None))
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.race(),
    ///     &RacesOrNull::MultipleUnowned(
    ///         vec![
    ///             Race::new(cde::v1::Race::Asian, None, None)
    ///         ]
    ///     )
    /// );
    /// ```
    pub fn race(&self) -> &RacesOrNull {
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
    /// use models::metadata::field::EthnicityOrNull;
    /// use models::metadata::field::unowned::Ethnicity;
    /// use models::subject::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///                 .ethnicity(Ethnicity::new(cde::v2::Ethnicity::NotHispanicOrLatino, None, None))
    ///                 .build();
    ///
    /// assert_eq!(
    ///     metadata.ethnicity(),
    ///     &EthnicityOrNull::Unowned(Ethnicity::new(cde::v2::Ethnicity::NotHispanicOrLatino, None, None))
    /// );
    /// ```
    pub fn ethnicity(&self) -> &EthnicityOrNull {
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
    /// use models::metadata::field::IdentifiersOrNull;
    /// use models::metadata::field::owned::Identifier;
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
    ///     &IdentifiersOrNull::MultipleOwned(
    ///         vec![
    ///             Identifier::new(
    ///                 cde::v1::Identifier::parse("organization:Name", ":").unwrap(),
    ///                 None, None, Some(true)
    ///             )
    ///         ]
    ///     )
    /// );
    /// ```
    pub fn identifiers(&self) -> &IdentifiersOrNull {
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
            sex: SexOrNull::Unowned(rand::random()),
            race: RacesOrNull::MultipleUnowned(vec![rand::random()]),
            ethnicity: EthnicityOrNull::Unowned(rand::random()),
            identifiers: IdentifiersOrNull::MultipleOwned(vec![field::owned::Identifier::new(
                identifier,
                None,
                None,
                Some(true),
            )]),
        }
    }
}
