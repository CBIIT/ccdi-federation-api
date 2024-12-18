use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// The method by which data was harmonized.
///
/// **NOTE:** if you find that there are types of harmonization methods that are
/// not captured here, please make an issue on the GitHub repository so we can
/// support the value.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::metadata::field::details::Method)]
pub enum Method {
    /// The data was mapped from its form at the source to match the harmonized
    /// term. This value represents ideas like translating between two diagnosis
    /// ontologies or manually categorizing a free-text field to a controlled
    /// vocabulary.
    Mapped,
}
