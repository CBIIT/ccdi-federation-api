use ccdi_models as models;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// Named gateways within a [`Data`](super::Data) response.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::file::data::Gateways)]
pub struct Gateways {
    /// The gateways.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    gateways: Vec<models::gateway::Named>,
}

impl std::ops::Deref for Gateways {
    type Target = Vec<models::gateway::Named>;

    fn deref(&self) -> &Self::Target {
        &self.gateways
    }
}

impl From<Vec<models::gateway::Named>> for Gateways {
    fn from(gateways: Vec<models::gateway::Named>) -> Self {
        Self { gateways }
    }
}
