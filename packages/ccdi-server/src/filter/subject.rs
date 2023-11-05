//! Filter parameters for [`Subject`]s.

use ccdi_models as models;

use models::Subject;

use crate::filter::FilterMetadataField;
use crate::params::filter::Subject as FilterSubjectParams;

impl FilterMetadataField<Subject, FilterSubjectParams> for Vec<Subject> {
    fn filter_metadata_field(self, field: String, params: &FilterSubjectParams) -> Vec<Subject> {
        let parameter = match field.as_str() {
            "sex" => params.sex.as_ref(),
            "race" => params.race.as_ref(),
            "ethnicity" => params.ethnicity.as_ref(),
            "identifiers" => params.identifiers.as_ref(),
            _ => unreachable!("unhandled subject metadata field: {field}"),
        };

        let query = match parameter {
            Some(query) => query,
            // If the parameter has no value, just return the original list of
            // subjects, as the user does not want to filter based on that.
            None => return self,
        };

        self.into_iter()
            .filter(|subject| {
                let values: Option<Vec<String>> = match field.as_str() {
                    "sex" => subject
                        .metadata()
                        .and_then(|metadata| metadata.sex())
                        .map(|sex| vec![sex.to_string()]),
                    "race" => subject
                        .metadata()
                        .and_then(|metadata| metadata.race())
                        .map(|race| {
                            race.iter()
                                .cloned()
                                .map(|r| r.to_string())
                                .collect::<Vec<String>>()
                        }),
                    "ethnicity" => subject
                        .metadata()
                        .and_then(|metadata| metadata.ethnicity())
                        .map(|ethnicity| vec![ethnicity.to_string()]),
                    "identifiers" => subject
                        .metadata()
                        .and_then(|metadata| metadata.identifiers())
                        .map(|identifiers| {
                            identifiers
                                .iter()
                                .cloned()
                                .map(|r| r.to_string())
                                .collect::<Vec<String>>()
                        }),
                    _ => unreachable!("unhandled subject metadata field: {field}"),
                };

                match values {
                    Some(values) => values.into_iter().any(|s| s.eq(query)),
                    // Subjects with no values for this field are automatically
                    // filtered as described in the rules for filtering.
                    None => false,
                }
            })
            .collect::<Vec<_>>()
    }
}
