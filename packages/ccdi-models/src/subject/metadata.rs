//! Metadata for a [`Subject`](super::Subject).

use ordered_float::OrderedFloat;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::field;
use crate::metadata::fields;
use crate::subject::Identifier;

mod age_at_vital_status;
mod builder;

pub use age_at_vital_status::AgeAtVitalStatus;
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

    /// The vital status of the subject.
    #[schema(value_type = field::unowned::subject::VitalStatus, nullable = true)]
    vital_status: Option<field::unowned::subject::VitalStatus>,

    /// The approximate age at vital status.
    #[schema(value_type = field::unowned::subject::AgeAtVitalStatus, nullable = true)]
    age_at_vital_status: Option<field::unowned::subject::AgeAtVitalStatus>,

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

    /// Gets the approximate age at vital status for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ordered_float::OrderedFloat;
    ///
    /// use models::metadata::field::unowned::subject::AgeAtVitalStatus;
    /// use models::subject::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .age_at_vital_status(AgeAtVitalStatus::new(
    ///         models::subject::metadata::AgeAtVitalStatus::from(OrderedFloat(365.25)),
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.age_at_vital_status(),
    ///     Some(&AgeAtVitalStatus::new(
    ///         models::subject::metadata::AgeAtVitalStatus::from(OrderedFloat(365.25)),
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn age_at_vital_status(&self) -> Option<&field::unowned::subject::AgeAtVitalStatus> {
        self.age_at_vital_status.as_ref()
    }

    /// Gets the vital status for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::VitalStatus;
    /// use models::subject::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .vital_status(VitalStatus::new(
    ///         cde::v1::subject::VitalStatus::Unknown,
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.vital_status(),
    ///     Some(&VitalStatus::new(
    ///         cde::v1::subject::VitalStatus::Unknown,
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn vital_status(&self) -> Option<&field::unowned::subject::VitalStatus> {
        self.vital_status.as_ref()
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

    /// Generates a random [`Metadata`] based on a particular
    /// [`Identifier`](ccdi_cde::v1::subject::Identifier).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::subject::Identifier;
    /// use models::subject::Metadata;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let identifier = Identifier::new(&namespace, "Name");
    /// let metadata = Metadata::random(identifier);
    /// ```
    pub fn random(identifier: Identifier) -> Metadata {
        Metadata {
            sex: Some(rand::random()),
            race: Some(vec![rand::random()]),
            ethnicity: Some(rand::random()),
            identifiers: Some(vec![field::owned::subject::Identifier::new(
                identifier.into_inner(),
                None,
                None,
                Some(true),
            )]),
            vital_status: Some(rand::random()),
            age_at_vital_status: Some(field::unowned::subject::AgeAtVitalStatus::new(
                crate::subject::metadata::AgeAtVitalStatus::from(OrderedFloat(365.25)),
                None,
                None,
            )),
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
            "{\"sex\":null,\"race\":null,\"ethnicity\":null,\"identifiers\":null,\"vital_status\":null,\"age_at_vital_status\":null}"
        );
    }
}
