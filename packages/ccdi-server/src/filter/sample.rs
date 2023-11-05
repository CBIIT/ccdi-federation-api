//! Filter parameters for [`Sample`]s.

use ccdi_models as models;

use models::Sample;

use crate::filter::FilterMetadataField;
use crate::params::filter::Sample as FilterSampleParams;

impl FilterMetadataField<Sample, FilterSampleParams> for Vec<Sample> {
    fn filter_metadata_field(self, field: String, params: &FilterSampleParams) -> Vec<Sample> {
        let parameter = match field.as_str() {
            "disease_phase" => params.disease_phase.as_ref(),
            "tissue_type" => params.tissue_type.as_ref(),
            "tumor_classification" => params.tumor_classification.as_ref(),
            _ => unreachable!("unhandled sample metadata field: {field}"),
        };

        let query = match parameter {
            Some(query) => query,
            // If the parameter has no value, just return the original list of
            // samples, as the user does not want to filter based on that.
            None => return self,
        };

        self.into_iter()
            .filter(|sample| {
                let values: Option<Vec<String>> = match field.as_str() {
                    "disease_phase" => sample
                        .metadata()
                        .and_then(|metadata| metadata.disease_phase())
                        .map(|disease_phase| vec![disease_phase.to_string()]),
                    "tissue_type" => sample
                        .metadata()
                        .and_then(|metadata| metadata.tissue_type())
                        .map(|tissue_type| vec![tissue_type.to_string()]),
                    "tumor_classification" => sample
                        .metadata()
                        .and_then(|metadata| metadata.tumor_classification())
                        .map(|tumor_classification| vec![tumor_classification.to_string()]),
                    _ => unreachable!("unhandled sample metadata field: {field}"),
                };

                match values {
                    Some(values) => values.into_iter().any(|s| s.eq(query)),
                    // Samples with no values for this field are automatically
                    // filtered as described in the rules for filtering.
                    None => false,
                }
            })
            .collect::<Vec<_>>()
    }
}
