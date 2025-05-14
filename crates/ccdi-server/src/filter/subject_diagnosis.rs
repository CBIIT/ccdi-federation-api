//! Filter parameters for [`Subject`]s on the subject-diagnosis endpoint.

use ccdi_models as models;

use models::metadata::common::deposition::Accession;
use models::Subject;

use crate::filter::FilterMetadataField;
use crate::params::filter::SubjectDiagnosis as FilterSubjectDiagnosisParams;

impl FilterMetadataField<Subject, FilterSubjectDiagnosisParams> for Vec<Subject> {
    fn filter_metadata_field(
        self,
        field: String,
        params: &FilterSubjectDiagnosisParams,
    ) -> Vec<Subject> {
        let parameter = match field.as_str() {
            "sex" => params.sex.as_ref(),
            "race" => params.race.as_ref(),
            "ethnicity" => params.ethnicity.as_ref(),
            "identifiers" => params.identifiers.as_ref(),
            "vital_status" => params.vital_status.as_ref(),
            "age_at_vital_status" => params.age_at_vital_status.as_ref(),
            "depositions" => params.depositions.as_ref(),
            "search" => params.search.as_ref(),
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
                if field.as_str() == "search" {
                    if let Some(metadata) = subject.metadata() {
                        if let Some(associated_diagnoses) = metadata.associated_diagnoses() {
                            // Only return the entry if the query is a substring
                            // of at least one of the associated_diagnoses, ignoring case.
                            // Matching on to_lowercase is an approximation and will not cover
                            // all unicode characters.
                            let query_lower = query.to_string().to_lowercase();

                            let diagnoses_lower = associated_diagnoses
                                .iter()
                                .cloned()
                                .map(|r| r.value().to_string().to_lowercase())
                                .collect::<Vec<String>>();

                            return diagnoses_lower
                                .iter()
                                .any(|diagnosis_lower| diagnosis_lower.contains(&query_lower));
                        }
                        // If the metadata doesn't have associated_diagnoses, the entry
                        // should not be included.
                        return false;
                    }
                    // If no metadata is included, this entry should not be
                    // included.
                    false
                } else {
                    // All other "non-search" fields require an exact case-sensitive match.
                    let values: Option<Vec<String>> =
                        match field.as_str() {
                            "sex" => subject
                                .metadata()
                                .and_then(|metadata| metadata.sex())
                                .map(|sex| vec![sex.to_string()]),
                            "race" => subject.metadata().and_then(|metadata| metadata.race()).map(
                                |race| {
                                    race.iter()
                                        .cloned()
                                        .map(|r| r.to_string())
                                        .collect::<Vec<String>>()
                                },
                            ),
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
                            "vital_status" => subject
                                .metadata()
                                .and_then(|metadata| metadata.vital_status())
                                .map(|vital_status| vec![vital_status.to_string()]),
                            "age_at_vital_status" => subject
                                .metadata()
                                .and_then(|metadata| metadata.age_at_vital_status())
                                .map(|age_at_vital_status| vec![age_at_vital_status.to_string()]),
                            "depositions" => subject
                                .metadata()
                                .and_then(|metadata| metadata.common().depositions())
                                .map(|deposition| {
                                    deposition
                                        .iter()
                                        .cloned()
                                        .map(|accession| match accession {
                                            Accession::dbGaP(accession) => accession.to_string(),
                                        })
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
                }
            })
            .collect::<Vec<_>>()
    }
}
