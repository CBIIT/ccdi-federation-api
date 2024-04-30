//! Error responses.

use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpResponseBuilder;
use actix_web::ResponseError;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod kind;

pub use kind::Kind;

/// A wrapper around one or more [errors](Kind).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(as = responses::Errors)]
pub struct Errors {
    /// The errors within this response.
    #[schema(value_type = Vec<responses::error::Kind>)]
    errors: Vec<Kind>,
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let errors = self
            .errors
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "errors: {errors}")
    }
}

impl std::error::Error for Errors {}

impl ResponseError for Errors {
    fn status_code(&self) -> StatusCode {
        self.errors
            .first()
            .map(|error| error.status_code())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .json(web::Json(self))
    }
}

impl Errors {
    /// Creates a new [`Errors`] from a list of [`Kind`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::responses::Errors;
    /// use server::responses::error::Kind;
    ///
    /// let errors = server::responses::Errors::new(vec![
    ///     Kind::invalid_parameters(
    ///         Some(vec![String::from("id")]),
    ///         String::from("Parameter was not an integer.")
    ///     ),
    ///     Kind::not_found(String::from("Sample")),
    ///     Kind::unsupported_field(
    ///         String::from("handedness"),
    ///         String::from("Handedness does not apply to samples.")
    ///     ),
    /// ]);
    ///
    /// assert_eq!(serde_json::to_string(&errors)?, String::from("{\"errors\":[{\"kind\":\"InvalidParameters\",\"parameters\":[\"id\"],\"reason\":\"Parameter was not an integer.\",\"message\":\"Invalid value for parameter 'id': parameter was not an integer.\"},{\"kind\":\"NotFound\",\"entity\":\"Sample\",\"message\":\"Sample not found.\"},{\"kind\":\"UnsupportedField\",\"field\":\"handedness\",\"reason\":\"Handedness does not apply to samples.\",\"message\":\"Field 'handedness' is not supported: handedness does not apply to samples.\"}]}"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(errors: Vec<Kind>) -> Self {
        Errors { errors }
    }
}

impl From<Kind> for Errors {
    fn from(kind: Kind) -> Self {
        Errors::new(vec![kind])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_route() -> Result<(), Box<dyn std::error::Error>> {
        let errors = Errors::from(Kind::invalid_route(
            String::from("GET"),
            String::from("/foobar"),
        ));

        let result = serde_json::to_string(&errors)?;
        assert_eq!(&result, "{\"errors\":[{\"kind\":\"InvalidRoute\",\"method\":\"GET\",\"route\":\"/foobar\",\"message\":\"Invalid route: GET /foobar.\"}]}");

        Ok(())
    }

    #[test]
    fn invalid_parameter() -> Result<(), Box<dyn std::error::Error>> {
        let errors = Errors::from(Kind::invalid_parameters(
            Some(vec![String::from("id")]),
            String::from("cannot be zero"),
        ));

        let result = serde_json::to_string(&errors)?;
        assert_eq!(&result, "{\"errors\":[{\"kind\":\"InvalidParameters\",\"parameters\":[\"id\"],\"reason\":\"Cannot be zero.\",\"message\":\"Invalid value for parameter 'id': cannot be zero.\"}]}");

        let error = Errors::from(Kind::invalid_parameters(
            Some(vec![String::from("id")]),
            String::from("cannot be zero."),
        ));

        let result = serde_json::to_string(&error)?;
        assert_eq!(&result, "{\"errors\":[{\"kind\":\"InvalidParameters\",\"parameters\":[\"id\"],\"reason\":\"Cannot be zero.\",\"message\":\"Invalid value for parameter 'id': cannot be zero.\"}]}");

        let error = Errors::from(Kind::invalid_parameters(
            Some(vec![String::from("id")]),
            String::from("cannot be zero!"),
        ));

        let result = serde_json::to_string(&error)?;
        assert_eq!(&result, "{\"errors\":[{\"kind\":\"InvalidParameters\",\"parameters\":[\"id\"],\"reason\":\"Cannot be zero!\",\"message\":\"Invalid value for parameter 'id': cannot be zero!\"}]}");

        let error = Errors::from(Kind::invalid_parameters(
            None,
            String::from("could not parse"),
        ));

        let result = serde_json::to_string(&error)?;
        assert_eq!(&result, "{\"errors\":[{\"kind\":\"InvalidParameters\",\"parameters\":null,\"reason\":\"Could not parse.\",\"message\":\"Invalid parameters: could not parse.\"}]}");

        Ok(())
    }

    #[test]
    fn not_found() -> Result<(), Box<dyn std::error::Error>> {
        let error = Errors::from(Kind::not_found(String::from("Samples")));
        let result = serde_json::to_string(&error)?;

        assert_eq!(&result, "{\"errors\":[{\"kind\":\"NotFound\",\"entity\":\"Samples\",\"message\":\"Samples not found.\"}]}");

        Ok(())
    }

    #[test]
    fn unsupported_field() -> Result<(), Box<dyn std::error::Error>> {
        let error = Errors::from(Kind::unsupported_field(
            String::from("field"),
            String::from("The field was not included in the metadata object."),
        ));
        let result = serde_json::to_string(&error)?;

        assert_eq!(&result, "{\"errors\":[{\"kind\":\"UnsupportedField\",\"field\":\"field\",\"reason\":\"The field was not included in the metadata object.\",\"message\":\"Field 'field' is not supported: the field was not included in the metadata object.\"}]}");

        Ok(())
    }
}
