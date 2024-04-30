//! An inner representation of a error [`Kind`](super::Kind).
//!
//! **NOTE:** you typically should not be using [`Inner`] types directly and,
//! instead, should use the provided [`Kind`](super::Kind) methods. The code was
//! designed this way to ensure that all error responses could have a `message`
//! field that is uniformly calculated for each [`Kind`](super::Kind).
//! Ultimately, this [`Inner`] type is never used directly in the API and is
//! always flattened into a [`Kind`](super::Kind) (see the [`Kind`](super::Kind)
//! struct for more details).
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// An inner type of a [Kind](super::Kind).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(tag = "kind")]
pub enum Inner {
    /// Attempted to access an invalid route.
    ///
    /// Also includes all routes for which the path exists, but the HTTP method
    /// is not supported for that path.
    #[schema(example = json!(
        Inner::InvalidRoute {
            method: String::from("GET"),
            route: String::from("/foobar")
        }
    ))]
    InvalidRoute {
        /// The HTTP method that was used in the request.
        method: String,

        /// The route that was requested.
        route: String,
    },

    /// One or more invalid query or path parameters were provided.
    #[schema(example = json!(
        Inner::InvalidParameters {
            parameters: Some(vec![String::from("id")]),
            reason: String::from("The parameter was a non-integer value.")
        }
    ))]
    InvalidParameters {
        /// If known, the parameters that are invalid. If not known, pass `None`
        /// to this field for a more general error message.
        parameters: Option<Vec<String>>,

        /// A plain-text reason describing why the parameters are invalid.
        reason: String,
    },

    /// An entity was not found.
    #[schema(example = json!(Inner::NotFound { entity: String::from("Samples") }))]
    NotFound {
        /// The entity (or entities) that are not found.
        entity: String,
    },

    /// Line-level data cannot be shared for the specified entity.
    #[schema(example = json!(Inner::UnshareableData {
        entity: String::from("Sample"),
        reason: String::from("Our agreement with data providers prohibits us from sharing line-level data.")
    }))]
    UnshareableData {
        /// The entity (or entities) where data cannot be shared.
        entity: String,

        /// The reason that the line-level data cannot be shared.
        reason: String,
    },

    /// A field name was not supported for the attempted operation.
    #[schema(example = json!(Inner::UnsupportedField {
        field: String::from("field"),
        reason: String::from("The field was not found in the metadata object.")
    }))]
    UnsupportedField {
        /// The field that is not supported.
        field: String,

        /// The reason that the field is not supported.
        reason: String,
    },
}

impl Inner {
    /// Creates an [`Inner::InvalidParameters`] with a formalized `reason`.
    ///
    /// For more information on the definition of **formalizing** the `reason`
    /// field, see the [`formalize_reason()`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::responses::error::kind::Inner;
    ///
    /// let error = Inner::invalid_route(String::from("GET"), String::from("/foobar"));
    ///
    /// assert_eq!(
    ///     error.to_string(),
    ///     String::from("Invalid route: GET /foobar.")
    /// );
    /// ```
    pub fn invalid_route(method: String, route: String) -> Self {
        Inner::InvalidRoute { method, route }
    }

    /// Creates an [`Inner::InvalidParameters`] with a formalized `reason`.
    ///
    /// For more information on the definition of **formalizing** the `reason`
    /// field, see the [`formalize_reason()`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::responses::error::kind::Inner;
    ///
    /// let error = Inner::invalid_parameters(
    ///     Some(vec![String::from("id")]),
    ///     String::from("parameter is not a non-zero integer"),
    /// );
    ///
    /// assert_eq!(
    ///     error.to_string(),
    ///     String::from("Invalid value for parameter 'id': parameter is not a non-zero integer.")
    /// );
    /// ```
    pub fn invalid_parameters(parameters: Option<Vec<String>>, reason: String) -> Self {
        let reason = formalize_reason(reason.clone()).unwrap_or_else(|| {
            panic!("you should always provide a reason for an invalid parameters error")
        });
        Inner::InvalidParameters { parameters, reason }
    }

    /// Creates an [`Inner::NotFound`] with a formalized `reason`.
    ///
    /// For more information on the definition of **formalizing** the `reason`
    /// field, see the [`formalize_reason()`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::responses::error::kind::Inner;
    ///
    /// let error = Inner::not_found(String::from("Sample"));
    ///
    /// assert_eq!(error.to_string(), String::from("Sample not found."));
    /// ```
    pub fn not_found(entity: String) -> Self {
        Inner::NotFound { entity }
    }

    /// Creates an [`Inner::UnshareableData`] with a formalized `reason`.
    ///
    /// For more information on the definition of **formalizing** the `reason`
    /// field, see the [`formalize_reason()`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::responses::error::kind::Inner;
    ///
    /// let error = Inner::unshareable_data(
    ///     String::from("samples"),
    ///     String::from(
    ///         "Our agreement with data providers prohibits us from sharing \
    ///         line-level data.",
    ///     ),
    /// );
    ///
    /// assert_eq!(
    ///     error.to_string(),
    ///     String::from("Unable to share data for samples: our agreement with data providers prohibits us from sharing line-level data.")
    /// );
    /// ```
    pub fn unshareable_data(entity: String, reason: String) -> Self {
        let entity = capitalize(entity).unwrap_or_else(|| {
            panic!("you should always provide an entity for an unshareable data error")
        });

        let reason = formalize_reason(reason).unwrap_or_else(|| {
            panic!("you should always provide a reason for an unshareable data error")
        });

        Inner::UnshareableData { entity, reason }
    }

    /// Creates an [`Inner::UnsupportedField`] with a formalized `reason`.
    ///
    /// For more information on the definition of **formalizing** the `reason`
    /// field, see the [`formalize_reason()`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::responses::error::kind::Inner;
    ///
    /// let error = Inner::unsupported_field(
    ///     String::from("id"),
    ///     String::from("field is not included in the metadata object"),
    /// );
    ///
    /// assert_eq!(
    ///     error.to_string(),
    ///     String::from("Field 'id' is not supported: field is not included in the metadata object.")
    /// );
    /// ```
    pub fn unsupported_field(field: String, reason: String) -> Self {
        let reason = formalize_reason(reason.clone()).unwrap_or_else(|| {
            panic!("you should always provide a reason for an unsupported field error")
        });
        Inner::UnsupportedField { field, reason }
    }
}

impl std::fmt::Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inner::InvalidRoute { method, route } => {
                write!(f, "Invalid route: {method} {route}.")
            }
            Inner::InvalidParameters { parameters, reason } => {
                let reason = reason.to_lowercase();

                match parameters {
                    Some(parameters) => {
                        write!(f, "Invalid value for parameter")?;

                        if parameters.len() > 1 {
                            write!(f, "s")?;
                        }

                        let parameters = parameters
                            .iter()
                            .map(|parameter| format!("'{}'", parameter))
                            .collect::<Vec<_>>()
                            .join(" and ");

                        write!(f, " {parameters}: {reason}")
                    }
                    None => {
                        write!(f, "Invalid parameters: {reason}")
                    }
                }
            }
            Inner::NotFound { entity } => write!(f, "{entity} not found."),
            Inner::UnshareableData { entity, reason } => {
                let entity = entity.to_lowercase();
                let reason = reason.to_lowercase();
                write!(f, "Unable to share data for {entity}: {reason}")
            }
            Inner::UnsupportedField { field, reason } => {
                let reason = reason.to_lowercase();
                write!(f, "Field '{field}' is not supported: {reason}")
            }
        }
    }
}

/// Ensure that the first letter in the [`String`] is capitalized.
///
/// # Examples
///
/// ```
/// use ccdi_server as server;
///
/// use server::responses::error::kind::inner::capitalize;
///
/// assert_eq!(
///     capitalize(String::from("hello, world")),
///     Some(String::from("Hello, world"))
/// );
///
/// assert_eq!(capitalize(String::from("")), None);
/// ```
pub fn capitalize(value: String) -> Option<String> {
    if value.is_empty() {
        return None;
    }

    Some(
        value
            .chars()
            .take(1)
            .flat_map(|c| c.to_uppercase())
            .chain(value.chars().skip(1))
            .collect::<String>(),
    )
}

/// Formalizes a [`String`] (in this case, specifically, a reason for an error
/// response returned by the API). The following steps are performed to the
/// input [`String`]:
///
/// 1. Ensures that the [`String`] is punctuated with an ASCII punctuation mark.
///    If no ASCII punctuation mark exists, then a period (`.`) is appended to
///    the `reason`.
/// 2. Ensuring that the first character in the [`String`] is capitalized (via
///    the [`capitalize()`] function).
///
/// **NOTE:** an empty string is simply returned as [`None`], as that cannot be
/// formalized.
///
/// ```
/// use ccdi_server as server;
///
/// use server::responses::error::kind::inner::formalize_reason;
///
/// assert_eq!(
///     formalize_reason(String::from("hello, world")),
///     Some(String::from("Hello, world."))
/// );
///
/// assert_eq!(formalize_reason(String::from("")), None);
/// ```
pub fn formalize_reason(mut value: String) -> Option<String> {
    let c = match value.chars().last() {
        Some(c) => c,
        None => return None,
    };

    if !c.is_ascii_punctuation() {
        value.push('.')
    }

    capitalize(value)
}

impl std::error::Error for Inner {}
