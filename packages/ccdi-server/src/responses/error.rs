use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// A response indicating an error from the API.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Error)]
pub struct Error {
    #[schema(example = "An error occurred.")]
    error: String,
}

impl Error {
    /// Creates a new [`Error`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::responses::Error;
    ///
    /// let error = Error::new("This is an error.");
    ///
    /// let result = serde_json::to_string(&error)?;
    /// assert_eq!(&result, "{\"error\":\"This is an error.\"}");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<S: Into<String>>(error: S) -> Self {
        Self {
            error: error.into(),
        }
    }
}
