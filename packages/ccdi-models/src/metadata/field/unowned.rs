//! Unowned metadata fields.

// This must be present because we are using aliases from utoipa. Utoipa does
// not give us a way to document those generated types, and it is not possible
// to add this statement only for those generated types, so we must allow it in
// the entire file.
#![allow(missing_docs)]

use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

#[macropol::macropol]
macro_rules! unowned_field {
    ($name: ident, $as: ty, $inner: ty, $inner_as: ty, $value: expr, $import: expr) => {
        #[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, ToSchema)]
        #[schema(as = $as)]
        /// An unowned field representing a [`${stringify!($name)}`].
        pub struct $name {
            /// The value of the metadata field.
            #[schema(value_type = $inner_as)]
            value: $inner,

            /// The ancestors from which this field was derived.
            ///
            /// Ancestors should be provided as period (`.`) delimited paths
            /// from the `metadata` key in the subject response object.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[schema(nullable = false)]
            ancestors: Option<Vec<String>>,

            /// A free-text comment field.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[schema(nullable = false)]
            comment: Option<String>,
        }

        impl $name {
            /// Creates a new [`${stringify!($name)}`].
            ///
            /// # Examples
            ///
            /// ```
            /// use ${stringify!($import)};
            /// use ccdi_models as models;
            ///
            /// use models::metadata::${stringify!($as)};
            ///
            /// let field = ${stringify!($name)}::new(
            ///     ${stringify!($value)},
            ///     None,
            ///     None
            /// );
            /// ```
            pub fn new(
                value: $inner,
                ancestors: Option<Vec<String>>,
                comment: Option<String>,
            ) -> Self {
                Self {
                    value,
                    ancestors,
                    comment,
                }
            }

            /// Gets the value from the [`${stringify!($name)}`] by reference.
            ///
            /// # Examples
            ///
            /// ```
            /// use ${stringify!($import)};
            /// use ccdi_models as models;
            ///
            /// use models::metadata::${stringify!($as)};
            ///
            /// let field = ${stringify!($name)}::new(
            ///     ${stringify!($value)},
            ///     None,
            ///     None
            /// );
            ///
            /// assert_eq!(field.value(), &${stringify!($value)});
            /// ```
            pub fn value(&self) -> &$inner {
                &self.value
            }

            /// Gets the ancestors from the [`${stringify!($name)}`] by reference.
            ///
            /// # Examples
            ///
            /// ```
            /// use ${stringify!($import)};
            /// use ccdi_models as models;
            ///
            /// use models::metadata::${stringify!($as)};
            ///
            /// let field = ${stringify!($name)}::new(
            ///     ${stringify!($value)},
            ///     Some(vec![String::from("another_field")]),
            ///     None
            /// );
            ///
            /// assert_eq!(field.ancestors(), Some(&vec![String::from("another_field")]));
            /// ```
            pub fn ancestors(&self) -> Option<&Vec<String>> {
                self.ancestors.as_ref()
            }

            /// Gets the comment from the [`${stringify!($name)}`] by reference.
            ///
            /// # Examples
            ///
            /// ```
            /// use ${stringify!($import)};
            /// use ccdi_models as models;
            ///
            /// use models::metadata::${stringify!($as)};
            ///
            /// let field = ${stringify!($name)}::new(
            ///     ${stringify!($value)},
            ///     None,
            ///     Some(String::from("Comment."))
            /// );
            ///
            /// assert_eq!(field.comment(), Some(&String::from("Comment.")));
            /// ```
            pub fn comment(&self) -> Option<&String> {
                self.comment.as_ref()
            }
        }

        #[allow(trivial_bounds)]
        impl Distribution<$name> for Standard
        where
            Standard: Distribution<$inner>,
        {
            fn sample<R: rand::Rng + ?Sized>(&self, _: &mut R) -> $name {
                $name::new(rand::random(), None, None)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }
    };
}

unowned_field!(
    Field,
    field::unowned::Field,
    Value,
    Value,
    Value::Null,
    serde_json::Value
);

pub mod sample {
    use super::*;

    use ccdi_cde as cde;

    unowned_field!(
        AgeAtDiagnosis,
        field::unowned::sample::AgeAtDiagnosis,
        crate::sample::metadata::AgeAtDiagnosis,
        models::sample::metadata::AgeAtDiagnosis,
        models::sample::metadata::AgeAtDiagnosis::from(OrderedFloat(365.25)),
        ordered_float::OrderedFloat
    );

    unowned_field!(
        AgeAtCollection,
        field::unowned::sample::AgeAtCollection,
        crate::sample::metadata::AgeAtCollection,
        models::sample::metadata::AgeAtCollection,
        models::sample::metadata::AgeAtCollection::from(OrderedFloat(365.25)),
        ordered_float::OrderedFloat
    );

    unowned_field!(
        DiseasePhase,
        field::unowned::sample::DiseasePhase,
        cde::v1::sample::DiseasePhase,
        cde::v1::sample::DiseasePhase,
        cde::v1::sample::DiseasePhase::InitialDiagnosis,
        ccdi_cde as cde
    );

    unowned_field!(
        TissueType,
        field::unowned::sample::TissueType,
        cde::v2::sample::TissueType,
        cde::v2::sample::TissueType,
        cde::v2::sample::TissueType::Tumor,
        ccdi_cde as cde
    );

    unowned_field!(
        TumorClassification,
        field::unowned::sample::TumorClassification,
        cde::v1::sample::TumorClassification,
        cde::v1::sample::TumorClassification,
        cde::v1::sample::TumorClassification::Primary,
        ccdi_cde as cde
    );

    unowned_field!(
        TumorTissueMorphology,
        field::unowned::sample::TumorTissueMorphology,
        cde::v1::sample::TumorTissueMorphology,
        cde::v1::sample::TumorTissueMorphology,
        cde::v1::sample::TumorTissueMorphology::from(String::from("8010/0")),
        ccdi_cde as cde
    );

    unowned_field!(
        Identifier,
        field::unowned::sample::Identifier,
        crate::sample::identifier::referenced::Identifier,
        models::sample::identifier::referenced::Identifier,
        models::sample::identifier::referenced::Identifier::Linked(
            models::sample::identifier::linked::Identifier::new(
                models::sample::Identifier::new(
                    models::Namespace::new(
                        models::namespace::Identifier::new(
                            models::Organization::new(
                                "example-organization"
                                    .parse::<models::organization::Identifier>()
                                    .unwrap(),
                                "Example Organization"
                                    .parse::<models::organization::Name>()
                                    .unwrap(),
                            )
                            .id()
                            .clone(),
                            "ExampleNamespace"
                                .parse::<models::namespace::identifier::Name>()
                                .unwrap(),
                        ),
                        "support@example.com",
                        None,
                    )
                    .id()
                    .clone(),
                    "SampleName001"
                ),
                "https://ccdi.example.com/api/v0"
                    .parse::<models::Url>()
                    .unwrap()
            )
        ),
        ccdi_cde as cde
    );
}

pub mod subject {
    use super::*;

    use ccdi_cde as cde;

    unowned_field!(
        Sex,
        field::unowned::subject::Sex,
        cde::v1::subject::Sex,
        cde::v1::subject::Sex,
        cde::v1::subject::Sex::Unknown,
        ccdi_cde as cde
    );

    unowned_field!(
        Race,
        field::unowned::subject::Race,
        cde::v1::subject::Race,
        cde::v1::subject::Race,
        cde::v1::subject::Race::Unknown,
        ccdi_cde as cde
    );

    unowned_field!(
        Ethnicity,
        field::unowned::subject::Ethnicity,
        cde::v2::subject::Ethnicity,
        cde::v2::subject::Ethnicity,
        cde::v2::subject::Ethnicity::Unknown,
        ccdi_cde as cde
    );

    unowned_field!(
        AgeAtVitalStatus,
        field::unowned::subject::AgeAtVitalStatus,
        crate::subject::metadata::AgeAtVitalStatus,
        models::subject::metadata::AgeAtVitalStatus,
        models::subject::metadata::AgeAtVitalStatus::from(OrderedFloat(365.25)),
        ordered_float::OrderedFloat
    );

    unowned_field!(
        VitalStatus,
        field::unowned::subject::VitalStatus,
        cde::v1::subject::VitalStatus,
        cde::v1::subject::VitalStatus,
        cde::v1::subject::VitalStatus::Unknown,
        ccdi_cde as cde
    );

    unowned_field!(
        Identifier,
        field::unowned::subject::Identifier,
        crate::subject::identifier::referenced::Identifier,
        models::subject::identifier::referenced::Identifier,
        models::subject::identifier::referenced::Identifier::Linked(
            models::subject::identifier::linked::Identifier::new(
                models::subject::Identifier::new(
                    models::Namespace::new(
                        models::namespace::Identifier::new(
                            models::Organization::new(
                                "example-organization"
                                    .parse::<models::organization::Identifier>()
                                    .unwrap(),
                                "Example Organization"
                                    .parse::<models::organization::Name>()
                                    .unwrap(),
                            )
                            .id()
                            .clone(),
                            "ExampleNamespace"
                                .parse::<models::namespace::identifier::Name>()
                                .unwrap(),
                        ),
                        "support@example.com",
                        None,
                    )
                    .id()
                    .clone(),
                    "SubjectName001"
                ),
                "https://ccdi.example.com/api/v0"
                    .parse::<models::Url>()
                    .unwrap()
            )
        ),
        ccdi_cde as cde
    );
}

pub mod file {
    use super::*;

    use ccdi_cde as cde;

    unowned_field!(
        Type,
        field::unowned::file::Type,
        cde::v1::file::Type,
        cde::v1::file::Type,
        cde::v1::file::Type::TXT,
        ccdi_cde as cde
    );

    unowned_field!(
        Size,
        field::unowned::file::Size,
        cde::v1::file::Size,
        cde::v1::file::Size,
        cde::v1::file::Size::new(42),
        ccdi_cde as cde
    );

    unowned_field!(
        Checksums,
        field::unowned::file::Checksums,
        crate::file::metadata::Checksums,
        models::file::metadata::Checksums,
        models::file::metadata::Checksums::default(),
        ccdi_cde as cde
    );

    unowned_field!(
        Description,
        field::unowned::file::Description,
        cde::v1::file::Description,
        cde::v1::file::Description,
        cde::v1::file::Description::new("Hello, world!"),
        ccdi_cde as cde
    );
}
