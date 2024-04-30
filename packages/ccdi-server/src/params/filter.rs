//! Parameters related to filtering.

use std::collections::HashMap;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

/// Parameters for filtering subjects.
///
/// Parameters for filtering subjects are declared as JSON objects. For each key
/// provided that matches a valid metadata field in a subject, the endpoint will
/// filter the results according to the rules described for each field and
/// intersect the results.
///
/// Note that, when strings are provided, matching is case-sensitive unless
/// otherwise stated. At least one valid parameter for subjects must be included
/// in the `/subject/filter` request.
#[derive(Debug, Default, Deserialize, Introspect, Serialize, ToSchema)]
#[schema(as = server::params::filter::Subject)]
pub struct Subject {
    /// Matches any subject where the `sex` field exactly matches the specified
    /// value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "sex": "F" }` to select all
    /// subjects with a `sex` of `F`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub sex: Option<Option<String>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "race": "American Indian or
    /// Alaska Native" }` to select any subjects where any race matches this
    /// string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub race: Option<Option<String>>,

    /// Matches any subject where the `ethnicity` field exactly matches the
    /// specified value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "ethnicity": "Hispanic or
    /// Latino" }` to select all subjects with a `ethnicity` of `Hispanic or
    /// Latino`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub ethnicity: Option<Option<String>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "identifier":
    /// "Subject-OZNL7P47" }` to select any subjects where any identifiers
    /// matches this string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub identifier: Option<Option<String>>,

    /// Matches any subject where the `vital_status` field exactly matches the
    /// specified value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "vital_status": "Alive" }`
    /// to select all subjects with a `vital_status` of `Alive`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub vital_status: Option<Option<String>>,

    /// Matches any subject where the `age_at_vital_status` field exactly
    /// matches the specified value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "age_at_vital_status":
    /// 365.25 }` to select all subjects with a `age_at_vital_status` of
    /// `365.25`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub age_at_vital_status: Option<Option<f64>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "deposition":
    /// "phs000000.v1.p1" }` to select any subjects where any deposition value
    /// matches this string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub deposition: Option<Option<String>>,

    /// All unharmonized fields should be filterable as well:
    ///
    /// * Filtering on a singular field should include the `Subject` in the
    /// results if the query exactly matches the value of that field for the
    /// `Subject` (case-sensitive).
    /// * Filtering on field with multiple values should include the `Subject`
    /// in the results if the query exactly matches any of the values of the
    /// field for that `Subject` (case-sensitive).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub unharmonized: Option<Option<HashMap<String, Value>>>,
}

impl Subject {
    /// Checks to see if the parameters are empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    /// use server::params::filter::Subject;
    ///
    /// // This is intentionally not default to ensure all fields are
    /// // accounted for explicitly.
    /// let params = Subject {
    ///     sex: None,
    ///     race: None,
    ///     ethnicity: None,
    ///     identifier: None,
    ///     vital_status: None,
    ///     age_at_vital_status: None,
    ///     deposition: None,
    ///     unharmonized: None,
    /// };
    ///
    /// assert!(params.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.sex.is_none()
            && self.race.is_none()
            && self.ethnicity.is_none()
            && self.identifier.is_none()
            && self.vital_status.is_none()
            && self.age_at_vital_status.is_none()
            && self.deposition.is_none()
            && self.unharmonized.is_none()
    }
}

/// Parameters for filtering samples.
///
/// Parameters for filtering samples are declared as JSON objects. For each key
/// provided that matches a valid metadata field in a sample, the endpoint will
/// filter the results according to the rules described for each field and
/// intersect the results.
///
/// Note that, when strings are provided, matching is case-sensitive unless
/// otherwise stated. At least one valid parameter for samples must be included
/// in the `/sample/filter` request.
#[derive(Debug, Default, Deserialize, Introspect, Serialize, ToSchema)]
#[schema(as = server::params::filter::Sample)]
pub struct Sample {
    /// Matches any sample where the `age_at_diagnosis` field exactly matches
    /// the specified value.
    ///
    /// This parameter can either be a JSON number or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `age_at_diagnosis` are included in the results.
    /// - If a number is provided, all entries where `age_at_diagnosis` exactly
    ///   matches the provided number are included in the results.
    ///
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "age_at_diagnosis": 365.25
    /// }` to select all samples with a `age_at_diagnosis` of `365.25`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub age_at_diagnosis: Option<Option<f64>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "diagnosis": "sarcoma" }`
    /// to select any samples where any diagnosis value contains the string
    /// `sarcoma`.
    ///
    /// **Note:** this filter parameter contains non-standard filtering
    /// criteria—implementors should carefully read the description above.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub diagnosis: Option<Option<String>>,

    /// Matches any sample where the `disease_phase` field exactly matches the
    /// specified value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "disease_phase": "Initial
    /// Diagnosis" }` to select all samples with a `disease_phase` of `Initial
    /// Diagnosis`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub disease_phase: Option<Option<String>>,

    /// Matches any sample where the `tissue_type` field exactly matches the
    /// specified value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "tissue_type":
    /// "Unspecified" }` to select all samples with a `tissue_type` of
    /// `Unspecified`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub tissue_type: Option<Option<String>>,

    /// Matches any sample where the `tumor_classification` field exactly
    /// matches the specified value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "tumor_classification":
    /// "Regional" }` to select all samples with a `tumor_classification` of
    /// `Regional`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub tumor_classification: Option<Option<String>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "tumor_tissue_morphology":
    /// "8000/0" }` to select any samples where any tumor tissue morphology
    /// matches this string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub tumor_tissue_morphology: Option<Option<String>>,

    /// Matches any sample where the `age_at_collection` field exactly matches
    /// the specified value.
    ///
    /// This parameter can either be a JSON number or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `age_at_collection` are included in the results.
    /// - If a number is provided, all entries where `age_at_collection` exactly
    ///   matches the provided number are included in the results.
    ///
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "age_at_collection": 365.25
    /// }` to select all samples with a `age_at_collection` of `365.25`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub age_at_collection: Option<Option<f64>>,

    /// Matches any sample where the `library_strategy` field exactly matches
    /// the specified value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "library_strategy":
    /// "DNA-Seq" }` to select all samples with a `library_strategy` of
    /// `DNA-Seq`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub library_strategy: Option<Option<String>>,

    /// Matches any sample where the `preservation_method` field exactly matches
    /// the specified value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "preservation_method":
    /// "Cryopreserved" }` to select all samples with a `preservation_method` of
    /// `Cryopreserved`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub preservation_method: Option<Option<String>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "identifier":
    /// "Sample-F62VO0JV" }` to select any samples where any identifiers matches
    /// this string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub identifier: Option<Option<String>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "deposition":
    /// "phs000000.v1.p1" }` to select any samples where any deposition value
    /// matches this string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub deposition: Option<Option<String>>,

    /// All unharmonized fields should be filterable as well:
    ///
    /// * Filtering on a singular field should include the `Sample` in the
    /// results if the query exactly matches the value of that field for the
    /// `Sample` (case-sensitive).
    /// * Filtering on field with multiple values should include the `Sample` in
    /// the results if the query exactly matches any of the values of the field
    /// for that `Sample` (case-sensitive).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub unharmonized: Option<Option<HashMap<String, Value>>>,
}

impl Sample {
    /// Checks to see if the parameters are empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    /// use server::params::filter::Sample;
    ///
    /// // This is intentionally not default to ensure all fields are
    /// // accounted for explicitly.
    /// let params = Sample {
    ///     age_at_diagnosis: None,
    ///     diagnosis: None,
    ///     disease_phase: None,
    ///     tissue_type: None,
    ///     tumor_classification: None,
    ///     tumor_tissue_morphology: None,
    ///     age_at_collection: None,
    ///     library_strategy: None,
    ///     preservation_method: None,
    ///     identifier: None,
    ///     deposition: None,
    ///     unharmonized: None,
    /// };
    ///
    /// assert!(params.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.age_at_diagnosis.is_none()
            && self.diagnosis.is_none()
            && self.disease_phase.is_none()
            && self.tissue_type.is_none()
            && self.tumor_classification.is_none()
            && self.tumor_tissue_morphology.is_none()
            && self.age_at_collection.is_none()
            && self.library_strategy.is_none()
            && self.preservation_method.is_none()
            && self.identifier.is_none()
            && self.deposition.is_none()
            && self.unharmonized.is_none()
    }
}

/// Parameters for filtering files.
///
/// Parameters for filtering files are declared as JSON objects. For each key
/// provided that matches a valid metadata field in a file, the endpoint will
/// filter the results according to the rules described for each field and
/// intersect the results.
///
/// Note that, when strings are provided, matching is case-sensitive unless
/// otherwise stated. At least one valid parameter for files must be included in
/// the `/file/filter` request.
#[derive(Debug, Default, Deserialize, Introspect, Serialize, ToSchema)]
#[schema(as = server::params::filter::File)]
pub struct File {
    /// Matches any file where the `type` field exactly matches the specified
    /// value.
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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "type": "TXT" }` to select
    /// all files with a type of `TXT`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub r#type: Option<Option<String>>,

    /// Matches any file where the `size` field exactly matches the specified
    /// value.
    ///
    /// This parameter can either be a JSON number or `null`.
    ///
    /// - If `null` is provided as the value, all entries that are either (a)
    ///   missing a metadata block or (b) contain a metadata block but are
    ///   missing a value for `size` are included in the results.
    /// - If a number is provided, all entries where `size` exactly matches the
    ///   provided number are included in the results.
    ///
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "size": 12345 }` to select
    /// all files with a size of `12345`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub size: Option<Option<usize>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "checksum":
    /// "A9781F66C9C5C9DA8837132FFEB08CA" }` to select any files where any
    /// checksum matches this string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub checksum: Option<Option<String>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "description": "hello" }`
    /// to select all files where the description includes the substring
    /// `hello`.
    ///
    /// **Note:** this filter parameter contains non-standard filtering
    /// criteria—implementors should carefully read the description above.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub description: Option<Option<String>>,

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
    /// If the value is not one of the acceptable value types above, an error
    /// should be returned indicating this.
    ///
    /// For example, you might provide the filter `{ "deposition":
    /// "phs000000.v1.p1" }` to select any files where any deposition value
    /// matches this string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schema(required = false)]
    pub deposition: Option<Option<String>>,

    /// All unharmonized fields should be filterable as well:
    ///
    /// * Filtering on a singular field should include the `File` in the results
    /// if the query exactly matches the value of that field for the `File`
    /// (case-sensitive).
    /// * Filtering on field with multiple values should include the `File` in
    /// the results if the query exactly matches any of the values of the field
    /// for that `File` (case-sensitive).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub unharmonized: Option<Option<HashMap<String, Value>>>,
}

impl File {
    /// Checks to see if the parameters are empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    /// use server::params::filter::File;
    ///
    /// // This is intentionally not default to ensure all fields are
    /// // accounted for explicitly.
    /// let params = File {
    ///     r#type: None,
    ///     size: None,
    ///     checksum: None,
    ///     description: None,
    ///     deposition: None,
    ///     unharmonized: None,
    /// };
    ///
    /// assert!(params.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.r#type.is_none()
            && self.size.is_none()
            && self.checksum.is_none()
            && self.description.is_none()
            && self.deposition.is_none()
            && self.unharmonized.is_none()
    }
}
