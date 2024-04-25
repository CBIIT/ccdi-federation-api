//! Filter parameters for [`File`]s.

use ccdi_models as models;

use models::metadata::common::deposition::Accession;
use models::File;

use crate::filter::FilterMetadataField;
use crate::params::filter::File as FilterFileParams;

impl FilterMetadataField<File, FilterFileParams> for Vec<File> {
    fn filter_metadata_field(self, field: String, params: &FilterFileParams) -> Vec<File> {
        let parameter = match field.as_str() {
            "type" => params.r#type.as_ref(),
            "size" => params.size.as_ref(),
            "checksums" => params.checksums.as_ref(),
            "description" => params.description.as_ref(),
            "depositions" => params.depositions.as_ref(),
            _ => unreachable!("unhandled file metadata field: {field}"),
        };

        let query = match parameter {
            Some(query) => query,
            // If the parameter has no value, just return the original list of
            // files, as the user does not want to filter based on that.
            None => return self,
        };

        self.into_iter()
            .filter(|file| {
                if field.as_str() == "description" {
                    if let Some(metadata) = file.metadata() {
                        if let Some(description) = metadata.description() {
                            // Only return the entry if the query is a substring
                            // of the description.
                            return description.to_string().contains(query);
                        }

                        // If the metadata doesn't have a description, the entry
                        // should not be included.
                        return false;
                    }

                    // If no metadata is included, this entry should not be
                    // included.
                    false
                } else {
                    // All other "non-description" fields.
                    let values: Option<Vec<String>> = match field.as_str() {
                        "type" => file
                            .metadata()
                            .and_then(|metadata| metadata.r#type())
                            .map(|r#type| vec![r#type.to_string()]),
                        "size" => file
                            .metadata()
                            .and_then(|metadata| metadata.size())
                            .map(|size| vec![size.to_string()]),
                        "checksums" => file
                            .metadata()
                            .and_then(|metadata| metadata.checksums())
                            .map(|checksums| {
                                checksums
                                    .value()
                                    .as_map()
                                    .into_values()
                                    .map(|r| r.to_string())
                                    .collect::<Vec<String>>()
                            }),
                        "depositions" => file
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
                        _ => unreachable!("unhandled file metadata field: {field}"),
                    };

                    match values {
                        Some(values) => values.into_iter().any(|s| s.eq(query)),
                        // Files with no values for this field are automatically
                        // filtered as described in the rules for filtering.
                        None => false,
                    }
                }
            })
            .collect::<Vec<_>>()
    }
}
