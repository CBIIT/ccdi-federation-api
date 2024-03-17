//! Parameters related to partitioning.

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::IntoParams;
use utoipa::ToSchema;

/// The different methods a result can be partitioned by.
#[derive(Debug, Default, Deserialize, Introspect, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(as = params::Partitionable)]
pub enum Partitionable {
    /// Partition results by namespace.
    #[default]
    Namespace,
}

/// Parameters for partitioning results.
#[derive(Debug, Default, Deserialize, IntoParams, Introspect, Serialize, ToSchema)]
#[into_params(parameter_in = Query)]
#[schema(as = params::PartitionParams)]

pub struct PartitionParams {
    /// The method to partition results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(
        required = false,
        nullable = false,
        value_type = Option<params::Partitionable>
    )]
    pub partition: Option<Partitionable>,
}
