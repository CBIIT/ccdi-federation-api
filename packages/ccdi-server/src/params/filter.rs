//! Parameters related to filtering.

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::IntoParams;

/// Parameters for filtering subjects.
///
/// Parameters for filtering subjects are declared as valid JSON values.
/// Importantly, values must appear _exactly_ as they would in a JSON object
/// (so, for example, strings must be included within quotes). Failure to
/// provide filter parameters as a parsable JSON value will result in an error
/// being thrown. When a parameter is provided, the endpoint will filter the
/// results according to the rules described for that parameter.
///
/// When strings are provided, matching is case-sensitive unless otherwise
/// stated. None of the parameters are required to be included.
#[derive(Debug, Default, Deserialize, IntoParams, Introspect, Serialize)]
#[into_params(parameter_in = Query)]
pub struct Subject {
    /// Matches any subject where the `sex` field exactly matches the specified
    /// JSON value.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `sex` are included in the results.
    /// - If a string is provided, all entries where `sex` exactly matches the
    ///   provided string are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?sex="F"` to select all
    /// subjects with a `sex` of `F`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub sex: Option<String>,

    /// Matches subjects to their `race` value(s) according to the rules laid
    /// out below.
    ///
    /// This parameter can either be a JSON string or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `race` are included in the results.
    /// - If a string is provided, all entries where any `race` value exactly
    ///   matches the provided string are included in the results. Note that
    ///   this is effectively a logical OR (`||`) across any of the race values.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?race="American Indian or
    /// Alaska Native"` to select any subjects where any race matches this
    /// string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub race: Option<String>,

    /// Matches any subject where the `ethnicity` field exactly matches the
    /// specified JSON value.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `ethnicity` are included in the results.
    /// - If a string is provided, all entries where `ethnicity` exactly matches
    ///   the provided string are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?ethnicity="Hispanic or
    /// Latino"` to select all subjects with a `ethnicity` of `Hispanic or
    /// Latino`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub ethnicity: Option<String>,

    /// Matches subjects to their `identifiers` value(s) according to the rules
    /// laid out below.
    ///
    /// This parameter can either be a JSON string or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `identifiers` are included in the results.
    /// - If a string is provided, all entries where any `identifiers` have a
    ///   name that exactly matches the provided string are included in the
    ///   results. Note that this is effectively a logical OR (`||`) across any
    ///   of the identifier name values.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter
    /// `?identifier="Subject-OZNL7P47"` to select any subjects where any
    /// identifiers matches this string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub identifier: Option<String>,

    /// Matches any subject where the `vital_status` field exactly matches the
    /// specified JSON value.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `vital_status` are included in the results.
    /// - If a string is provided, all entries where `vital_status` exactly
    ///   matches the provided string are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?vital_status="Alive"` to
    /// select all subjects with a `vital_status` of `Alive`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub vital_status: Option<String>,

    /// Matches any subject where the `age_at_vital_status` field exactly
    /// matches the specified JSON value.
    ///
    /// This parameter can either be a JSON number (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `age_at_vital_status` are included in the results.
    /// - If a number is provided, all entries where `age_at_vital_status`
    ///   exactly matches the provided number are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?age_at_vital_status=365.25`
    /// to select all subjects with a `age_at_vital_status` of `365.25`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub age_at_vital_status: Option<String>,

    /// Matches subjects to their `depositions` value(s) according to the rules
    /// laid out below.
    ///
    /// This parameter can either be a JSON string or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `depositions` are included in the results.
    /// - If a string is provided, all entries where any `depositions` value
    ///   exactly matches the provided string are included in the results. Note
    ///   that this is effectively a logical OR (`||`) across any of the
    ///   deposition values.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter
    /// `?deposition="phs000000.v1.p1"` to select any subjects where any
    /// deposition value matches this string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub deposition: Option<String>,
}

/// Parameters for filtering samples.
///
/// Parameters for filtering samples are declared as valid JSON values.
/// Importantly, values must appear _exactly_ as they would in a JSON object
/// (so, for example, strings must be included within quotes). Failure to
/// provide filter parameters as a parsable JSON value will result in an error
/// being thrown. When a parameter is provided, the endpoint will filter the
/// results according to the rules described for that parameter.
///
/// When strings are provided, matching is case-sensitive unless otherwise
/// stated. None of the parameters are required to be included.
#[derive(Debug, Default, Deserialize, IntoParams, Introspect, Serialize)]
#[into_params(parameter_in = Query)]
pub struct Sample {
    /// Matches any sample where the `age_at_diagnosis` field exactly matches
    /// the specified JSON value.
    ///
    /// This parameter can either be a JSON number or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `age_at_diagnosis` are included in the results.
    /// - If a number is provided, all entries where `age_at_diagnosis` exactly
    ///   matches the provided number are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?age_at_diagnosis=365.25` to
    /// select all samples with a `age_at_diagnosis` of `365.25`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub age_at_diagnosis: Option<String>,

    /// Matches samples to their `diagnosis` value(s) according to the rules
    /// laid out below.
    ///
    /// This parameter can either be a JSON string or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `diagnosis` are included in the results.
    /// - If a string is provided, all entries where any `diagnosis` value
    ///   contains the provided string are included in the results. Note that
    ///   this is effectively a logical OR (`||`) across any of the diagnosis
    ///   values containing the substring.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?diagnosis="sarcoma"` to
    /// select any samples where any diagnosis value contains the string
    /// `sarcoma`.
    ///
    /// **Note:** this filter parameter contains non-standard filtering
    /// criteria—implementors should carefully read the description above.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub diagnosis: Option<String>,

    /// Matches any sample where the `disease_phase` field exactly matches the
    /// specified JSON value.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `disease_phase` are included in the results.
    /// - If a string is provided, all entries where `disease_phase` exactly
    ///   matches the provided string are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?disease_phase="Initial
    /// Diagnosis"` to select all samples with a `disease_phase` of `Initial
    /// Diagnosis`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub disease_phase: Option<String>,

    /// Matches any sample where the `tissue_type` field exactly matches the
    /// specified JSON value.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `tissue_type` are included in the results.
    /// - If a string is provided, all entries where `tissue_type` exactly
    ///   matches the provided string are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?tissue_type="Unspecified"`
    /// to select all samples with a `tissue_type` of `Unspecified`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub tissue_type: Option<String>,

    /// Matches any sample where the `tumor_classification` field exactly
    /// matches the specified JSON value.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `tumor_classification` are included in the
    ///   results.
    /// - If a string is provided, all entries where `tumor_classification`
    ///   exactly matches the provided string are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter
    /// `?tumor_classification="Regional"` to select all samples with a
    /// `tumor_classification` of `Regional`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub tumor_classification: Option<String>,

    /// Matches samples to their `tumor_tissue_morphology` value(s) according to
    /// the rules laid out below.
    ///
    /// This parameter can either be a JSON string or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `tumor_tissue_morphology` are included in the
    ///   results.
    /// - If a string is provided, all entries where any
    ///   `tumor_tissue_morphology` value exactly matches the provided string
    ///   are included in the results. Note that this is effectively a logical
    ///   OR (`||`) across any of the tumor tissue morphology values.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter
    /// `?tumor_tissue_morphology="8000/0"` to select any samples where any
    /// tumor tissue morphology matches this string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub tumor_tissue_morphology: Option<String>,

    /// Matches any sample where the `age_at_collection` field exactly matches
    /// the specified JSON value.
    ///
    /// This parameter can either be a JSON number or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `age_at_collection` are included in the results.
    /// - If a number is provided, all entries where `age_at_collection` exactly
    ///   matches the provided number are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?age_at_collection=365.25` to
    /// select all samples with a `age_at_collection` of `365.25`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub age_at_collection: Option<String>,

    /// Matches any sample where the `library_strategy` field exactly matches
    /// the specified JSON value.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `library_strategy` are included in the results.
    /// - If a string is provided, all entries where `library_strategy` exactly
    ///   matches the provided string are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?library_strategy="DNA-Seq"`
    /// to select all samples with a `library_strategy` of `DNA-Seq`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub library_strategy: Option<String>,

    /// Matches any sample where the `preservation_method` field exactly matches
    /// the specified JSON value.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `preservation_method` are included in the results.
    /// - If a string is provided, all entries where `preservation_method`
    ///   exactly matches the provided string are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter
    /// `?preservation_method="Cryopreserved"` to select all samples with a
    /// `preservation_method` of `Cryopreserved`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub preservation_method: Option<String>,

    /// Matches samples to their `identifiers` value(s) according to the rules
    /// laid out below.
    ///
    /// This parameter can either be a JSON string or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `identifiers` are included in the results.
    /// - If a string is provided, all entries where any `identifiers` have a
    ///   name that exactly matches the provided string are included in the
    ///   results. Note that this is effectively a logical OR (`||`) across any
    ///   of the identifier name values.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter
    /// `?identifier="Sample-F62VO0JV"` to select any samples where any
    /// identifiers matches this string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub identifier: Option<String>,

    /// Matches samples to their `depositions` value(s) according to the rules
    /// laid out below.
    ///
    /// This parameter can either be a JSON string or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `depositions` are included in the results.
    /// - If a string is provided, all entries where any `depositions` value
    ///   exactly matches the provided string are included in the results. Note
    ///   that this is effectively a logical OR (`||`) across any of the
    ///   deposition values.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter
    /// `?deposition="phs000000.v1.p1"` to select any samples where any
    /// deposition value matches this string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub deposition: Option<String>,
}

/// Parameters for filtering files.
///
/// Parameters for filtering files are declared as valid JSON values.
/// Importantly, values must appear _exactly_ as they would in a JSON object
/// (so, for example, strings must be included within quotes). Failure to
/// provide filter parameters as a parsable JSON value will result in an error
/// being thrown. When a parameter is provided, the endpoint will filter the
/// results according to the rules described for that parameter.
///
/// When strings are provided, matching is case-sensitive unless otherwise
/// stated. None of the parameters are required to be included.
#[derive(Debug, Default, Deserialize, IntoParams, Introspect, Serialize)]
#[into_params(parameter_in = Query)]
pub struct File {
    /// Matches any file where the `type` field exactly matches the specified
    /// JSON value.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `type` are included in the results.
    /// - If a string is provided, all entries where `type` exactly matches the
    ///   provided string are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?type="TXT"` to select all
    /// files with a type of `TXT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub r#type: Option<String>,

    /// Matches any file where the `size` field exactly matches the specified
    /// JSON value.
    ///
    /// This parameter can either be a JSON number or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `size` are included in the results.
    /// - If a number is provided, all entries where `size` exactly matches the
    ///   provided number are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?size=12345` to select all
    /// files with a size of `12345`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub size: Option<String>,

    /// Matches files to their `checksums` value(s) according to the rules laid
    /// out below.
    ///
    /// This parameter can either be a JSON string or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `checksums` are included in the results.
    /// - If a string is provided, all entries where any `checksums` value
    ///   exactly matches the provided string are included in the results. Note
    ///   that this is effectively a logical OR (`||`) across any of the
    ///   checksum values.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter
    /// `?checksum="A9781F66C9C5C9DA8837132FFEB08CA"` to select any files where
    /// any checksum matches this string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub checksum: Option<String>,

    /// Matches files to their `description` value according to the rules laid
    /// out below.
    ///
    /// This parameter can either be a JSON string (surrounded by quotes) or
    /// `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `description` are included in the results.
    /// - If a string is provided, all entries where the `description` contains
    ///   the string provided (case-sensitive) are included in the results.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter `?description="hello"` to
    /// select all files where the description includes the substring `hello`.
    ///
    /// **Note:** this filter parameter contains non-standard filtering
    /// criteria—implementors should carefully read the description above.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub description: Option<String>,

    /// Matches files to their `depositions` value(s) according to the rules
    /// laid out below.
    ///
    /// This parameter can either be a JSON string or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `depositions` are included in the results.
    /// - If a string is provided, all entries where any `depositions` value
    ///   exactly matches the provided string are included in the results. Note
    ///   that this is effectively a logical OR (`||`) across any of the
    ///   deposition values.
    ///
    /// If a query parameter value is provided that is not able to be parsed as
    /// a JSON value, an error should be returned indicating this. Further, if a
    /// JSON value is able to be parsed, but the value is not one of the
    /// acceptable value types above, an error should be returned indicating
    /// this.
    ///
    /// For example, you might provide the filter
    /// `?deposition="phs000000.v1.p1"` to select any files where any deposition
    /// value matches this string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[param(required = false)]
    pub deposition: Option<String>,
}
