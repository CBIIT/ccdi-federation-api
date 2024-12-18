use introspect::Introspect;
use ordered_float::OrderedFloat;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// The approximate age of diagnosis in days.
///
/// * When the age at diagnosis is collected by the source server in days, the
///   number of days is reported directly.
/// * When the age at diagnosis is collected by the source server in years, the
///   number of years is multiplied by 365.25 to arrive at an approximate number
///   of days.
#[derive(
    Clone, Debug, Deserialize, Eq, Introspect, Ord, PartialEq, PartialOrd, Serialize, ToSchema,
)]
#[schema(as = models::sample::metadata::AgeAtDiagnosis, value_type = f32)]
pub struct AgeAtDiagnosis(OrderedFloat<f32>);

impl From<OrderedFloat<f32>> for AgeAtDiagnosis {
    fn from(value: OrderedFloat<f32>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for AgeAtDiagnosis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
