//! Information regarding the data contained within a server.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod version;

pub use version::Version;

/// Information that is specific to the API that the server implements.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::info::data::Information)]
pub struct Information {
    /// The version of data published within this server.
    #[schema(value_type = responses::info::data::Version)]
    version: Version,

    /// The ISO 8601 formatted, UTC-based date and time when the data was last
    /// updated.
    ///
    /// This represents the last _update_ time. In contrast to the
    /// `data_version` field, this field is updated whenever a data update is
    /// performed irrespective of whether there were actually changes in the
    /// data.
    last_updated: DateTime<Utc>,

    /// A URL pointing to the wiki.
    ///
    /// The intention of this field is to make users aware that we maintain a
    /// federation-wide wiki that describes the data elements in detail.
    #[schema(default = "https://github.com/CBIIT/ccdi-federation-api/wiki")]
    wiki_url: String,

    /// If available, a link pointing to where users can learn more about the
    /// data contained within this particular server.
    ///
    /// This is intended to be a server-specification documentation link, not
    /// any link that is developed by the federation.
    #[schema(default = "https://docs.example.com")]
    documentation_url: Option<String>,
}

impl Default for Information {
    fn default() -> Self {
        Self {
            // SAFETY: one is non-zero, so this will always unwrap.
            version: Version::default(),
            last_updated: Utc::now(),
            wiki_url: String::from("https://github.com/CBIIT/ccdi-federation-api/wiki"),
            documentation_url: Some(String::from(
                "https://github.com/CBIIT/ccdi-federation-api#development-process",
            )),
        }
    }
}
