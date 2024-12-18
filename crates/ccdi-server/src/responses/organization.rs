//! Responses related to organizations.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

/// A response for describing a organization.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Organization)]
pub struct Organization(models::Organization);

impl From<models::Organization> for Organization {
    fn from(organization: models::Organization) -> Self {
        Self(organization)
    }
}

/// A response for describing organizations.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Organizations)]
pub struct Organizations(Vec<models::Organization>);

impl From<Vec<models::Organization>> for Organizations {
    fn from(organizations: Vec<models::Organization>) -> Self {
        Self(organizations)
    }
}
