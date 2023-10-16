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

use ccdi_cde as cde;

#[macropol::macropol]
macro_rules! unowned_field {
    ($name: ident, $as: ty, $inner: ty, $value: expr, $import: expr) => {
        #[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, ToSchema)]
        #[schema(as = $as)]
        /// An unowned field representing a [`${stringify!($name)}`].
        pub struct $name {
            /// The value of the metadata field.
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
            /// use models::metadata::field::unowned::${stringify!($name)};
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
            /// use models::metadata::field::unowned::${stringify!($name)};
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
            /// use models::metadata::field::unowned::${stringify!($name)};
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
            /// use models::metadata::field::unowned::${stringify!($name)};
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
    };
}

unowned_field!(
    Field,
    field::unowned::Field,
    Value,
    Value::Null,
    serde_json::Value
);

unowned_field!(
    Sex,
    field::Sex,
    cde::v1::Sex,
    cde::v1::Sex::Unknown,
    ccdi_cde as cde
);

unowned_field!(
    Race,
    field::Race,
    cde::v1::Race,
    cde::v1::Race::Unknown,
    ccdi_cde as cde
);

unowned_field!(
    Ethnicity,
    field::Ethnicity,
    cde::v2::Ethnicity,
    cde::v2::Ethnicity::Unknown,
    ccdi_cde as cde
);
