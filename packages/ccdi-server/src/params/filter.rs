//! Parameters related to filtering.

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::IntoParams;

/// Parameters for filtering subjects.
///
/// None of the parameters are required, but they may be provided as a
/// [`String`]. When a parameter is provided, the endpoint will filter the
/// results to only include [`Subject`]s where the value for the key exactly
/// matches the value provided for the parameter (i.e., matching is done by
/// looking for the provided parameter as a substring). Matches are
/// case-sensitive.
#[derive(Debug, Default, Deserialize, IntoParams, Introspect, Serialize)]
#[into_params(parameter_in = Query)]
pub struct Subject {
    /// Matches any subject where the `sex` field matches the string provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false, nullable = false)]
    pub sex: Option<String>,

    /// Matches any subject where any member of the `race` fieldmatches
    /// the string provided.
    ///
    /// **Note:** a logical OR (`||`) is performed across the values when
    /// determining whether the subject should be included in the results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false, nullable = false)]
    pub race: Option<String>,

    /// matches any subject where the `ethnicity` field matches the string provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false, nullable = false)]
    pub ethnicity: Option<String>,

    /// Matches any subject where any member of the `identifiers` fieldmatches
    /// the string provided.
    ///
    /// **Note:** a logical OR (`||`) is performed across the values when
    /// determining whether the subject should be included in the results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false, nullable = false)]
    pub identifiers: Option<String>,
}

/// Parameters for filtering subjects.
///
/// None of the parameters are required, but they may be provided as a
/// [`String`]. When a parameter is provided, the endpoint will filter the
/// results to only include [`Subject`]s where the value for the key exactly
/// matches the value provided for the parameter (i.e., matching is done by
/// looking for the provided parameter as a substring). Matches are
/// case-sensitive.
#[derive(Debug, Default, Deserialize, IntoParams, Introspect, Serialize)]
#[into_params(parameter_in = Query)]
pub struct Sample {
    /// Matches any subject where the `disease_phase` field matches the string
    /// provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false, nullable = false)]
    pub disease_phase: Option<String>,

    /// Matches any subject where the `tissue_type` field matches the string
    /// provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false, nullable = false)]
    pub tissue_type: Option<String>,

    /// Matches any subject where the `tumor_classification` field matches the
    /// string provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false, nullable = false)]
    pub tumor_classification: Option<String>,
}