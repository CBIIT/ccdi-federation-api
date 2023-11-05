//! Metadata for a [`Subject`](super::Subject).

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::field;
use crate::metadata::fields;

use ccdi_cde as cde;

pub mod builder;

pub use builder::Builder;

/// Metadata associated with a subject.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::subject::Metadata)]
pub struct Metadata {
    /// The sex of the subject.
    #[schema(value_type = field::unowned::subject::Sex, nullable = true)]
    sex: Option<field::unowned::subject::Sex>,

    /// The race(s) of the subject.
    #[schema(value_type = Vec<field::unowned::subject::Race>, nullable = true)]
    race: Option<Vec<field::unowned::subject::Race>>,

    /// The ethnicity of the subject.
    #[schema(value_type = field::unowned::subject::Ethnicity, nullable = true)]
    ethnicity: Option<field::unowned::subject::Ethnicity>,

    /// The identifiers for the subject.
    ///
    /// Note that this list of identifiers *must* include the main identifier
    /// for the [`Subject`].
    #[schema(value_type = Vec<field::owned::subject::Identifier>, nullable = true)]
    identifiers: Option<Vec<field::owned::subject::Identifier>>,

    /// An unharmonized map of metadata fields.
    #[schema(value_type = fields::Unharmonized)]
    #[serde(skip_serializing_if = "fields::Unharmonized::is_empty")]
    unharmonized: fields::Unharmonized,
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
    /// use models::metadata::field::unowned::subject::Sex;
    /// use models::subject::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .sex(Sex::new(cde::v1::subject::Sex::Female, None, None))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.sex(),
    ///     Some(&Sex::new(cde::v1::subject::Sex::Female, None, None))
    /// );
    /// ```
    pub fn sex(&self) -> Option<&field::unowned::subject::Sex> {
        self.sex.as_ref()
    }

    /// Gets the harmonized race(s) for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///     .append_race(Race::new(cde::v1::subject::Race::Asian, None, None))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.race(),
    ///     Some(&vec![Race::new(cde::v1::subject::Race::Asian, None, None)])
    /// );
    /// ```
    pub fn race(&self) -> Option<&Vec<field::unowned::subject::Race>> {
        self.race.as_ref()
    }

    /// Gets the harmonized ethnicity for the [`Metadata`].
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
    /// let metadata = Builder::default()
    ///     .ethnicity(Ethnicity::new(
    ///         cde::v2::subject::Ethnicity::NotHispanicOrLatino,
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.ethnicity(),
    ///     Some(&Ethnicity::new(
    ///         cde::v2::subject::Ethnicity::NotHispanicOrLatino,
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn ethnicity(&self) -> Option<&field::unowned::subject::Ethnicity> {
        self.ethnicity.as_ref()
    }

    /// Gets the harmonized identifier(s) for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::owned::subject::Identifier;
    /// use models::subject::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .append_identifier(Identifier::new(
    ///         cde::v1::subject::Identifier::parse("organization:Name", ":").unwrap(),
    ///         None,
    ///         None,
    ///         Some(true),
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.identifiers(),
    ///     Some(&vec![Identifier::new(
    ///         cde::v1::subject::Identifier::parse("organization:Name", ":").unwrap(),
    ///         None,
    ///         None,
    ///         Some(true)
    ///     )])
    /// );
    /// ```
    pub fn identifiers(&self) -> Option<&Vec<field::owned::subject::Identifier>> {
        self.identifiers.as_ref()
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
    /// use models::subject::metadata::Builder;
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

    /// Generates a random [`Metadata`] based on a particular [`Identifier`](cde::v1::subject::Identifier).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::subject::Metadata;
    ///
    /// let identifier = cde::v1::subject::Identifier::parse("organization:Name", ":").unwrap();
    /// let metadata = Metadata::random(identifier);
    /// ```
    pub fn random(identifier: cde::v1::subject::Identifier) -> Metadata {
        Metadata {
            sex: Some(rand::random()),
            race: Some(vec![rand::random()]),
            ethnicity: Some(rand::random()),
            identifiers: Some(vec![field::owned::subject::Identifier::new(
                identifier,
                None,
                None,
                Some(true),
            )]),
            unharmonized: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::subject::metadata::builder;

    #[test]
    fn it_skips_serializing_the_unharmonized_key_when_it_is_empty() {
        let metadata = builder::Builder::default().build();
        assert_eq!(
            &serde_json::to_string(&metadata).unwrap(),
            "{\"sex\":null,\"race\":null,\"ethnicity\":null,\"identifiers\":null}"
        );
    }
}
