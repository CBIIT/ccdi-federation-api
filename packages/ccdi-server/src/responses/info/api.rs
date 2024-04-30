//! Information regarding the API implemented by a server.

use clap::crate_version;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// Information that is specific to the API that the server implements.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::info::api::Information)]
pub struct Information {
    /// The version of the API that this server supports.
    #[schema(example = "v1.0.0")]
    api_version: String,

    /// A URL pointing to the latest version of the Swagger documentation.
    ///
    /// Note that, at times, the latest version of the Swagger documentation may
    /// not be in sync with the version of the API deployed for this server. The
    /// intention of this field is not to link to a Swagger specification that
    /// strictly matches this particular server, but rather, to point users to
    /// where the specification is developed and hosted.
    #[schema(default = "https://cbiit.github.io/ccdi-federation-api/")]
    documentation_url: String,
}

impl Default for Information {
    fn default() -> Self {
        Self {
            api_version: format!("v{}", crate_version!()),
            documentation_url: String::from("https://cbiit.github.io/ccdi-federation-api/"),
        }
    }
}
