//! Owned metadata fields.

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
macro_rules! owned_field {
    ($name: ident, $as: ty, $inner: ty, $inner_as: ty, $value: expr, $import: expr) => {
        #[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, ToSchema)]
        #[schema(as = $as)]
        /// An owned field representing a [`${stringify!($name)}`].
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

            /// Any important details pertaining specifically to this assigned,
            /// harmonized value.
            ///
            /// Harmonization details _can_ be included for any metadata field,
            /// but, generally speaking, should be omitted unless there is
            /// important information to communicate regarding how the data in
            /// the field specifically was harmonized.
            ///
            /// See the "Interpreting metadata assignments" section of the
            /// specification for more details on when information should be
            /// included in the `/metadata/fields/<entity>` response for a
            /// metadata field and when it should be included in the `details`
            /// key.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[schema(nullable = false, value_type = Option<models::metadata::field::Details>)]
            details: Option<crate::metadata::field::Details>,

            /// A free-text comment field.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[schema(nullable = false)]
            comment: Option<String>,

            /// Whether or not the field is owned by the source server.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[schema(nullable = false)]
            owned: Option<bool>,
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
            ///     None,
            ///     None,
            ///     Some(true)
            /// );
            /// ```
            pub fn new(
                value: $inner,
                ancestors: Option<Vec<String>>,
                details: Option<crate::metadata::field::Details>,
                comment: Option<String>,
                owned: Option<bool>,
            ) -> Self {
                Self {
                    value,
                    ancestors,
                    details,
                    comment,
                    owned,
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
            ///     None,
            ///     None,
            ///     Some(false)
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
            ///     Some(vec![String::from("reported_sex")]),
            ///     None,
            ///     None,
            ///     Some(false)
            /// );
            ///
            /// assert_eq!(field.ancestors(), Some(&vec![String::from("reported_sex")]));
            /// ```
            pub fn ancestors(&self) -> Option<&Vec<String>> {
                self.ancestors.as_ref()
            }

            /// Gets harmonization details for the [`${stringify!($name)}`] by
            /// reference.
            ///
            /// # Examples
            ///
            /// ```
            /// use ${stringify!($import)};
            /// use ccdi_models as models;
            ///
            /// use models::metadata::${stringify!($as)};
            /// use models::metadata::field::details::Harmonizer;
            /// use models::metadata::field::details::Method;
            /// use models::metadata::field::Details;
            /// use models::Url;
            ///
            /// let details = Details::new(
            ///     Some(Method::Mapped),
            ///     Some(Harmonizer::DomainExpert),
            ///     Some(Url::from(
            ///         url::Url::try_from("https://hello.world/").unwrap(),
            ///     )),
            /// );
            ///
            /// let field = ${stringify!($name)}::new(
            ///     ${stringify!($value)},
            ///     None,
            ///     Some(details.clone()),
            ///     None,
            ///     Some(false)
            /// );
            ///
            /// assert_eq!(field.details(), Some(&details));
            /// ```
            pub fn details(&self) -> Option<&crate::metadata::field::Details> {
                self.details.as_ref()
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
            ///     None,
            ///     Some(String::from("Comment.")),
            ///     Some(false)
            /// );
            ///
            /// assert_eq!(field.comment(), Some(&String::from("Comment.")));
            /// ```
            pub fn comment(&self) -> Option<&String> {
                self.comment.as_ref()
            }

            /// Gets the ownership from the [`${stringify!($name)}`].
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
            ///     None,
            ///     None,
            ///     Some(true)
            /// );
            ///
            /// assert_eq!(field.owned(), Some(true));
            /// ```
            pub fn owned(&self) -> Option<bool> {
                self.owned.as_ref().copied()
            }
        }

        #[allow(trivial_bounds)]
        impl Distribution<$name> for Standard
        where
            Standard: Distribution<$inner>,
        {
            fn sample<R: rand::Rng + ?Sized>(&self, _: &mut R) -> $name {
                $name::new(rand::random(), None, None, None, Some(false))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }
    };
}

owned_field!(
    Field,
    field::owned::Field,
    Value,
    Value,
    Value::Null,
    serde_json::Value
);
