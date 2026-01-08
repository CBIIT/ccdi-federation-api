use std::ops::Deref;
use std::ops::DerefMut;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// The associated diagnosis categories for a [`Subject`](crate::Subject).
///
/// These values correspond to the CCDI diagnosis *categories* and are restricted
/// to the controlled vocabulary defined by the CCDI CDEs.
///
/// Unlike [`AssociatedDiagnoses`], which is free-text, this field is strongly
/// typed and backed by the `DiagnosisCategory` enum.
#[derive(
    Clone, Debug, Deserialize, Eq, Introspect, Ord, PartialEq, PartialOrd, Serialize, ToSchema,
)]
#[schema(as = models::subject::metadata::AssociatedDiagnosisCategories)]
pub struct AssociatedDiagnosisCategories(Vec<cde::v1::sample::DiagnosisCategory>,);

impl From<Vec<cde::v1::sample::DiagnosisCategory>> for AssociatedDiagnosisCategories {
    fn from(value: Vec<cde::v1::sample::DiagnosisCategory>) -> Self {
        Self(value)
    }
}

impl Deref for AssociatedDiagnosisCategories {
    type Target = Vec<cde::v1::sample::DiagnosisCategory>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AssociatedDiagnosisCategories {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for AssociatedDiagnosisCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self
            .0
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", joined)
    }
}
