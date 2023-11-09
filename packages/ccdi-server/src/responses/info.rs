//! Responses related to server info.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod api;
pub mod data;
pub mod server;

/// A response for information regarding the server.
#[derive(Debug, Default, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Information)]
pub struct Information {
    /// Information regarding the server itself.
    #[schema(value_type = responses::info::server::Information)]
    server: server::Information,

    /// Information regarding the API implemented by the server.
    #[schema(value_type = responses::info::api::Information)]
    api: api::Information,

    /// Information regarding data contained within the server.
    #[schema(value_type = responses::info::data::Information)]
    data: data::Information,
}
