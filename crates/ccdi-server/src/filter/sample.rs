//! Filter parameters for [`Sample`]s.

use ccdi_models as models;

use models::metadata::common::deposition::Accession;
use models::Sample;

use crate::filter::FilterMetadataField;
use crate::params::filter::Sample as FilterSampleParams;

impl FilterMetadataField<Sample, FilterSampleParams> for Vec<Sample> {
    fn filter_metadata_field(self, field: String, params: &FilterSampleParams) -> Vec<Sample> {
        let parameter = match field.as_str() {
            "anatomical_sites" => params.anatomical_sites.as_ref(),
            "disease_phase" => params.disease_phase.as_ref(),
            "library_selection_method" => params.library_selection_method.as_ref(),
            "library_strategy" => params.library_strategy.as_ref(),
            "library_source_material" => params.library_source_material.as_ref(),
            "preservation_method" => params.preservation_method.as_ref(),
            "tumor_grade" => params.tumor_grade.as_ref(),
            "specimen_molecular_analyte_type" => params.specimen_molecular_analyte_type.as_ref(),
            "tissue_type" => params.tissue_type.as_ref(),
            "tumor_classification" => params.tumor_classification.as_ref(),
            "age_at_diagnosis" => params.age_at_diagnosis.as_ref(),
            "age_at_collection" => params.age_at_collection.as_ref(),
            "tumor_tissue_morphology" => params.tumor_tissue_morphology.as_ref(),
            "depositions" => params.depositions.as_ref(),
            "diagnosis" => params.diagnosis.as_ref(),
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
                    "anatomical_sites" => sample
                        .metadata()
                        .and_then(|metadata| metadata.anatomical_sites())
                        .map(|sites| {
                            sites
                                .iter()
                                .map(|site| site.to_string())
                                .collect::<Vec<_>>()
                        }),
                    "disease_phase" => sample
                        .metadata()
                        .and_then(|metadata| metadata.disease_phase())
                        .map(|disease_phase| vec![disease_phase.to_string()]),
                    "library_selection_method" => sample
                        .metadata()
                        .and_then(|metadata| metadata.library_selection_method())
                        .map(|library_selection_method| vec![library_selection_method.to_string()]),
                    "library_strategy" => sample
                        .metadata()
                        .and_then(|metadata| metadata.library_strategy())
                        .map(|library_strategy| vec![library_strategy.to_string()]),
                    "library_source_material" => sample
                        .metadata()
                        .and_then(|metadata| metadata.library_source_material())
                        .map(|library_source_material| vec![library_source_material.to_string()]),
                    "preservation_method" => sample
                        .metadata()
                        .and_then(|metadata| metadata.preservation_method())
                        .map(|preservation_method| vec![preservation_method.to_string()]),
                    "tumor_grade" => sample
                        .metadata()
                        .and_then(|metadata| metadata.tumor_grade())
                        .map(|tumor_grade| vec![tumor_grade.to_string()]),
                    "specimen_molecular_analyte_type" => sample
                        .metadata()
                        .and_then(|metadata| metadata.specimen_molecular_analyte_type())
                        .map(|specimen_molecular_analyte_type| {
                            vec![specimen_molecular_analyte_type.to_string()]
                        }),
                    "tissue_type" => sample
                        .metadata()
                        .and_then(|metadata| metadata.tissue_type())
                        .map(|tissue_type| vec![tissue_type.to_string()]),
                    "tumor_classification" => sample
                        .metadata()
                        .and_then(|metadata| metadata.tumor_classification())
                        .map(|tumor_classification| vec![tumor_classification.to_string()]),
                    "age_at_diagnosis" => sample
                        .metadata()
                        .and_then(|metadata| metadata.age_at_diagnosis())
                        .map(|age_at_diagnosis| vec![age_at_diagnosis.to_string()]),
                    "age_at_collection" => sample
                        .metadata()
                        .and_then(|metadata| metadata.age_at_collection())
                        .map(|age_at_collection| vec![age_at_collection.to_string()]),
                    "tumor_tissue_morphology" => sample
                        .metadata()
                        .and_then(|metadata| metadata.tumor_tissue_morphology())
                        .map(|tumor_tissue_morphology| vec![tumor_tissue_morphology.to_string()]),
                    "depositions" => sample
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
                    "diagnosis" => sample
                        .metadata()
                        .and_then(|metadata| metadata.diagnosis())
                        .map(|diagnosis| vec![diagnosis.to_string()]),
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
