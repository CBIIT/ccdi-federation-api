//! Responses related to metadata fields.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

use models::metadata::field::Description;

/// A response for describing metadata fields for a subject.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::metadata::FieldDescriptions)]
pub struct FieldDescriptions {
    /// Field descriptions.
    #[schema(value_type = Vec<models::metadata::field::Description>)]
    fields: Vec<Description>,
}

impl From<Vec<Description>> for FieldDescriptions {
    fn from(fields: Vec<Description>) -> Self {
        Self { fields }
    }
}
