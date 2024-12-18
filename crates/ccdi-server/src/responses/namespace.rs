//! Responses related to namespaces.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

/// A response for describing a namespace.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Namespace)]
pub struct Namespace(models::Namespace);

impl From<models::Namespace> for Namespace {
    fn from(namespace: models::Namespace) -> Self {
        Self(namespace)
    }
}

/// A response for describing namespaces.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Namespaces)]
pub struct Namespaces(Vec<models::Namespace>);

impl From<Vec<models::Namespace>> for Namespaces {
    fn from(namespaces: Vec<models::Namespace>) -> Self {
        Self(namespaces)
    }
}
