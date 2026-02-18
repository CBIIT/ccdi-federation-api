//! Metadata for a [`Subject`](super::Subject).

use ordered_float::OrderedFloat;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng as _;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::common;
use crate::metadata::field;
use crate::metadata::fields;
use crate::subject::Identifier;

mod age_at_vital_status;
mod associated_diagnoses;
mod associated_diagnosis_categories;
mod builder;

pub use age_at_vital_status::AgeAtVitalStatus;
pub use associated_diagnoses::AssociatedDiagnoses;
pub use associated_diagnosis_categories::AssociatedDiagnosisCategories;
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

    /// The alternate identifiers for the subject.
    ///
    /// Note that this list of identifiers *must* include the main identifier
    /// for the [`Subject`].
    #[schema(value_type = Vec<field::unowned::subject::Identifier>, nullable = true)]
    identifiers: Option<Vec<field::unowned::subject::Identifier>>,

    /// The vital status of the subject.
    #[schema(value_type = field::unowned::subject::VitalStatus, nullable = true)]
    vital_status: Option<field::unowned::subject::VitalStatus>,

    /// The approximate age at vital status.
    #[schema(value_type = field::unowned::subject::AgeAtVitalStatus, nullable = true)]
    age_at_vital_status: Option<field::unowned::subject::AgeAtVitalStatus>,

    /// The associated diagnoses for the subject.
    #[schema(value_type = Vec<field::unowned::subject::AssociatedDiagnoses>, nullable = true)]
    associated_diagnoses: Option<Vec<field::unowned::subject::AssociatedDiagnoses>>,

    /// The associated diagnoses categories for the subject.
    #[schema(value_type = Vec<field::unowned::subject::AssociatedDiagnosisCategories>, nullable = true)]
    associated_diagnosis_categories:
        Option<Vec<field::unowned::subject::AssociatedDiagnosisCategories>>,

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
    ///     .sex(Sex::new(cde::v1::subject::Sex::Female, None, None, None))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.sex(),
    ///     Some(&Sex::new(cde::v1::subject::Sex::Female, None, None, None))
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
    ///     .append_race(Race::new(cde::v1::subject::Race::Asian, None, None, None))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.race(),
    ///     Some(&vec![Race::new(
    ///         cde::v1::subject::Race::Asian,
    ///         None,
    ///         None,
    ///         None
    ///     )])
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
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.ethnicity(),
    ///     Some(&Ethnicity::new(
    ///         cde::v2::subject::Ethnicity::NotHispanicOrLatino,
    ///         None,
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
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::subject::metadata::Builder;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    ///     None,
    /// );
    ///
    /// let subject_id = models::subject::identifier::referenced::Identifier::Linked(
    ///     models::subject::identifier::linked::Identifier::new(
    ///         models::subject::Identifier::new(namespace.id().clone(), "SubjectName001"),
    ///         "https://ccdi.example.com/api/v0"
    ///             .parse::<models::Url>()
    ///             .unwrap(),
    ///     ),
    /// );
    ///
    /// let field = Identifier::new(subject_id, None, None, None);
    /// let metadata = Builder::default().append_identifier(field.clone()).build();
    ///
    /// assert_eq!(metadata.identifiers(), Some(&vec![field]));
    /// ```
    pub fn identifiers(&self) -> Option<&Vec<field::unowned::subject::Identifier>> {
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
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.age_at_vital_status(),
    ///     Some(&AgeAtVitalStatus::new(
    ///         models::subject::metadata::AgeAtVitalStatus::from(OrderedFloat(365.25)),
    ///         None,
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
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.vital_status(),
    ///     Some(&VitalStatus::new(
    ///         cde::v1::subject::VitalStatus::Unknown,
    ///         None,
    ///         None,
    ///         None
    ///     ))
    /// );
    /// ```
    pub fn vital_status(&self) -> Option<&field::unowned::subject::VitalStatus> {
        self.vital_status.as_ref()
    }

    /// Gets the associated diagnoses for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::AssociatedDiagnoses;
    /// use models::subject::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .append_associated_diagnoses(AssociatedDiagnoses::new(
    ///         models::subject::metadata::AssociatedDiagnoses::from(String::from(
    ///             "Acute Lymphoblastic Leukemia",
    ///         )),
    ///         None,
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.associated_diagnoses(),
    ///     Some(&vec![AssociatedDiagnoses::new(
    ///         models::subject::metadata::AssociatedDiagnoses::from(String::from(
    ///             "Acute Lymphoblastic Leukemia"
    ///         )),
    ///         None,
    ///         None,
    ///         None
    ///     )])
    /// );
    /// ```
    pub fn associated_diagnoses(
        &self,
    ) -> Option<&Vec<field::unowned::subject::AssociatedDiagnoses>> {
        self.associated_diagnoses.as_ref()
    }

    /// Gets the associated diagnoses categories for the [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::AssociatedDiagnosisCategories;
    /// use models::subject::metadata::Builder;
    ///
    /// let metadata = Builder::default()
    ///     .append_associated_diagnosis_categories(AssociatedDiagnosisCategories::new(
    ///         vec![cde::v1::sample::DiagnosisCategory::AtypicalTeratoidRhabdoidTumors].into(),
    ///         None,
    ///         None,
    ///         None,
    ///     ))
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.associated_diagnosis_categories(),
    ///     Some(&vec![AssociatedDiagnosisCategories::new(
    ///         vec![cde::v1::sample::DiagnosisCategory::AtypicalTeratoidRhabdoidTumors].into(),
    ///         None,
    ///         None,
    ///         None
    ///     )])
    /// );
    /// ```
    pub fn associated_diagnosis_categories(
        &self,
    ) -> Option<&Vec<field::unowned::subject::AssociatedDiagnosisCategories>> {
        self.associated_diagnosis_categories.as_ref()
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
    /// use models::subject::metadata::Builder;
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
    /// use models::subject::metadata::Builder;
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

    /// Generates a random [`Metadata`] based on a particular [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::subject::Metadata;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    ///     None,
    /// );
    ///
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    /// let metadata = Metadata::random(subject_id);
    /// ```
    pub fn random(identifier: Identifier) -> Metadata {
        let mut rng = thread_rng();

        Metadata {
            sex: Some(rand::random()),
            race: Some(vec![rand::random()]),
            ethnicity: Some(rand::random()),
            identifiers: Some(vec![
                field::unowned::subject::Identifier::new(
                    crate::subject::identifier::referenced::Identifier::Linked(
                        crate::subject::identifier::linked::Identifier::new(
                            identifier.clone(),
                            "https://ccdi.example.com/api/v0"
                                .parse::<crate::Url>()
                                .unwrap(),
                        ),
                    ),
                    None,
                    None,
                    None,
                ),
                field::unowned::subject::Identifier::new(
                    crate::subject::identifier::referenced::Identifier::Unlinked(
                        crate::subject::identifier::unlinked::Identifier::from(format!(
                            "Subject-{}",
                            (0..8)
                                .map(|_| rng.sample(Alphanumeric).to_ascii_uppercase() as char)
                                .collect::<String>()
                        )),
                    ),
                    None,
                    None,
                    None,
                ),
            ]),
            vital_status: Some(rand::random()),
            age_at_vital_status: Some(field::unowned::subject::AgeAtVitalStatus::new(
                crate::subject::metadata::AgeAtVitalStatus::from(OrderedFloat(365.25)),
                None,
                None,
                None,
            )),
            // One to three diagnoses of the format Random Diagnosis X
            associated_diagnoses: Some(
                (0..rng.gen_range(1..4))
                    .map(|_| {
                        field::unowned::subject::AssociatedDiagnoses::new(
                            AssociatedDiagnoses::from(format!(
                                "Random Diagnosis {}",
                                rng.sample(Alphanumeric).to_ascii_uppercase() as char,
                            )),
                            None,
                            None,
                            None,
                        )
                    })
                    .collect(),
            ),
            associated_diagnosis_categories: Some(vec![rand::random()]),

            common: Default::default(),
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
            "{\"sex\":null,\"race\":null,\"ethnicity\":null,\"identifiers\":null,\"vital_status\":null,\"age_at_vital_status\":null,\"associated_diagnoses\":null,\"associated_diagnosis_categories\":null,\"depositions\":null}"
        );
    }
}
