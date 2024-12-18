//! Information regarding the version of data contained within a server.

use clap::crate_version;
use serde::Deserialize;
use serde::Serialize;
use url::Url;
use utoipa::ToSchema;

/// A description of how data is versioning within the source server.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(as = responses::info::data::version::About)]
pub enum About {
    /// A free-text description of the data version included with the response
    /// from the source server. This field is interpreted as Markdown (as
    /// defined by the [CommonMark](https://commonmark.org/) specification).
    #[serde(rename = "about")]
    Text(String),

    /// A URL where one can learn more about the data versioning for this source
    /// server.
    #[schema(value_type = String)]
    #[serde(rename = "about_url")]
    Url(Url),
}

/// The version of data published within this source server.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::info::data::Version)]
pub struct Version {
    /// The value of the version.
    ///
    /// This field represents a free-text field where data is arbitrarily
    /// versioned by the source server. Any versioning scheme is permissible.
    #[schema(example = 1, minimum = 1, value_type = usize)]
    version: String,

    /// A description of how data is versioned within the source server.
    #[serde(flatten)]
    #[schema(value_type = responses::info::data::version::About)]
    about: About,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            version: format!("v{}", crate_version!()),
            about: About::Text(String::from(
                "# About Versioning

Data within this example server is versioned according the semantic version \
of the crate that produces it (prefixed with a 'v').",
            )),
        }
    }
}
