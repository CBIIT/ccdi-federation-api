//! The `data` key in a [`Files`](super::Files) response.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod files;
mod gateways;

pub use files::Files;
pub use gateways::Gateways;

/// The `data` key within a [`Files`](super::Files)
/// response.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::file::Data)]
pub struct Data {
    /// The files.
    #[serde(flatten)]
    #[schema(value_type = responses::file::data::Files)]
    files: Files,

    /// The gateways.
    #[serde(flatten)]
    #[schema(value_type = responses::file::data::Gateways)]
    gateways: Gateways,
}

impl Data {
    /// Creates a new [`Data`].
    pub fn new(files: Files, gateways: Gateways) -> Self {
        Self { files, gateways }
    }
}
