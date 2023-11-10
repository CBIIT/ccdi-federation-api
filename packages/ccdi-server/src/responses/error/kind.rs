//! A kind of API error response.

use actix_web::body::BoxBody;
use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web::ResponseError;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod inner;

pub use inner::Inner;

/// A response indicating an error from the API.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::error::Kind)]
pub struct Kind {
    #[serde(flatten)]
    #[schema(inline)]
    inner: Inner,

    /// A plain-text description of the error.
    ///
    /// This field is intended to be shown within a user interface or similar if
    /// needed. Please use this field if you intend to pass the error along to a
    /// user.
    message: String,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl ResponseError for Kind {
    fn status_code(&self) -> StatusCode {
        match self.inner {
            Inner::InvalidParameters { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Inner::NotFound { .. } => StatusCode::NOT_FOUND,
            Inner::UnsupportedField { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Inner::UnshareableData { .. } => StatusCode::NOT_FOUND,
            Inner::InvalidRoute { .. } => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .json(web::Json(self))
    }
}

impl Kind {
    /// Creates a new [Kind] with an [`InvalidRoute`](Inner::InvalidRoute) inner.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// let error = server::responses::error::Kind::invalid_route(
    ///     String::from("GET"),
    ///     String::from("/foobar")
    /// );
    ///
    /// assert_eq!(serde_json::to_string(&error)?, String::from("{\"kind\":\"InvalidRoute\",\"method\":\"GET\",\"route\":\"/foobar\",\"message\":\"Invalid route: GET /foobar.\"}"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn invalid_route(method: String, route: String) -> Self {
        let inner = Inner::invalid_route(method, route);

        Self {
            message: inner.to_string(),
            inner,
        }
    }

    /// Creates a new [Kind] with an [`InvalidParameters`](Inner::InvalidParameters) inner.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// let error = server::responses::error::Kind::invalid_parameters(
    ///     Some(vec![String::from("id")]),
    ///     String::from("Parameter was not an integer.")
    /// );
    ///
    /// assert_eq!(serde_json::to_string(&error)?, String::from("{\"kind\":\"InvalidParameters\",\"parameters\":[\"id\"],\"reason\":\"Parameter was not an integer.\",\"message\":\"Invalid value for parameter 'id': parameter was not an integer.\"}"));
    ///
    /// let error = server::responses::error::Kind::invalid_parameters(
    ///     None,
    ///     String::from("Parameter not within serializable range.")
    /// );
    ///
    /// assert_eq!(serde_json::to_string(&error)?, String::from("{\"kind\":\"InvalidParameters\",\"parameters\":null,\"reason\":\"Parameter not within serializable range.\",\"message\":\" Invalid parameters: parameter not within serializable range.\"}"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn invalid_parameters(parameters: Option<Vec<String>>, reason: String) -> Self {
        let inner = Inner::invalid_parameters(parameters, reason);

        Self {
            message: inner.to_string(),
            inner,
        }
    }

    /// Creates a new [Kind] with an [`NotFound`](Inner::NotFound) inner.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// let error = server::responses::error::Kind::not_found(String::from("Sample"));
    ///
    /// assert_eq!(
    ///     serde_json::to_string(&error)?,
    ///     String::from(
    ///         "{\"kind\":\"NotFound\",\"entity\":\"Sample\",\"message\":\"Sample not found.\"}"
    ///     )
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn not_found(entity: String) -> Self {
        let inner = Inner::not_found(entity);

        Self {
            message: inner.to_string(),
            inner,
        }
    }

    /// Creates a new [Kind] with an
    /// [`UnshareableData`](Inner::UnshareableData) inner.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// let error = server::responses::error::Kind::unshareable_data(
    ///     String::from("samples"),
    ///     String::from(
    ///         "Our agreement with data providers prohibits us from sharing \
    ///         line-level data."
    ///     ),
    /// );
    ///
    /// assert_eq!(serde_json::to_string(&error)?, String::from("{\"kind\":\"UnshareableData\",\"entity\":\"Samples\",\"reason\":\"Our agreement with data providers prohibits us from sharing line-level data.\",\"message\":\"Unable to share data for samples: our agreement with data providers prohibits us from sharing line-level data.\"}"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn unshareable_data(entity: String, reason: String) -> Self {
        let inner = Inner::unshareable_data(entity, reason);

        Self {
            message: inner.to_string(),
            inner,
        }
    }

    /// Creates a new [Kind] with an
    /// [`UnsupportedField`](Inner::UnsupportedField) inner.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// let error = server::responses::error::Kind::unsupported_field(
    ///     String::from("handedness"),
    ///     String::from("Handedness does not apply to samples.")
    /// );
    ///
    /// assert_eq!(serde_json::to_string(&error)?, String::from("{\"kind\":\"UnsupportedField\",\"field\":\"handedness\",\"reason\":\"Handedness does not apply to samples.\",\"message\":\"Field 'handedness' is not supported: handedness does not apply to samples.\"}"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn unsupported_field(field: String, reason: String) -> Self {
        let inner = Inner::unsupported_field(field, reason);

        Self {
            message: inner.to_string(),
            inner,
        }
    }
}
