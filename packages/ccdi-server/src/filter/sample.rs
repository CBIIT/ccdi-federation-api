//! Filter parameters for [`Sample`]s.

use ccdi_models as models;

use models::metadata::common::deposition::Accession;
use models::sample::identifier::referenced::Identifier::Linked;
use models::sample::identifier::referenced::Identifier::Unlinked;
use models::Sample;
use serde_json::Value;

use crate::filter::FilterMetadataField;
use crate::params::filter::Sample as FilterSampleParams;
use crate::responses::Errors;

impl FilterMetadataField<Sample, FilterSampleParams> for Vec<Sample> {
    fn filter_metadata_field(
        self,
        field: String,
        params: &FilterSampleParams,
    ) -> Result<Vec<Sample>, Errors> {
        let parameter = match field.as_str() {
            "age_at_diagnosis" => match &params.age_at_diagnosis {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "diagnosis" => match &params.diagnosis {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "disease_phase" => match &params.disease_phase {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "tissue_type" => match &params.tissue_type {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "tumor_classification" => match &params.tumor_classification {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "tumor_tissue_morphology" => match &params.tumor_tissue_morphology {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "age_at_collection" => match &params.age_at_collection {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "library_strategy" => match &params.library_strategy {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "preservation_method" => match &params.preservation_method {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "identifier" => match &params.identifier {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "deposition" => match &params.deposition {
                Some(value) => serde_json::to_value(value.to_owned()).unwrap(),
                None => return Ok(self),
            },
            "unharmonized" => match &params.unharmonized {
                Some(_) => {
                    // NOTE: this server does not support filtering by unharmonized data
                    // because it does not produce any yet.
                    todo!("this server does not yet support filtering unharmonized data")
                }
                None => return Ok(self),
            },
            _ => unreachable!("unhandled sample metadata field: {field}"),
        };

        self.into_iter()
            .filter_map(|sample| {
                if field.as_str() == "diagnosis" {
                    // Diagnosis values need to be treated specially, as an
                    // entry containing the provided value as a substring should
                    // be included in the results.
                    match &parameter {
                        Value::Null => {
                            if let Some(metadata) = sample.metadata() {
                                if metadata.diagnosis().is_none() {
                                    // The user is requesting all samples with a
                                    // `null` diagnosis, and the metadata block
                                    // is provided, but the value of the
                                    // diagnosis is `null`. Thus, this sample
                                    // should be included.
                                    Some(Ok(sample))
                                } else {
                                    // The user is requesting all samples with a
                                    // `null` diagnosis, and the metadata block
                                    // is provided + the value of the diagnosis
                                    // is not `null`. Thus, this sample should
                                    // **not** be included.
                                    None
                                }
                            } else {
                                // The user is requesting all samples with a
                                // `null` diagnosis, and this sample includes no
                                // metadata at all, so the sample should be
                                // included.
                                Some(Ok(sample))
                            }
                        }
                        Value::String(query) => {
                            if let Some(metadata) = sample.metadata() {
                                if let Some(diagnosis) = metadata.diagnosis() {
                                    if diagnosis
                                        .iter()
                                        .any(|value| value.to_string().contains(query))
                                    {
                                        // The user is requesting all samples
                                        // with a particular substring, and
                                        // this sample includes a metadata block
                                        // + it has a diagnosis + the diagnosis
                                        // contains the desired substring. Thus,
                                        // it is included.
                                        Some(Ok(sample))
                                    } else {
                                        // The user is requesting all samples
                                        // with a particular substring, and
                                        // this sample includes a metadata block
                                        // + it has a diagnosis, but the
                                        // diagnosis does not contain the
                                        // desired substring. Thus, it is not
                                        // included.
                                        None
                                    }
                                } else {
                                    // The user is requesting all samples with a
                                    // particular substring, and this sample
                                    // includes a metadata block, but it has no
                                    // diagnosis. Thus, the sample is not
                                    // included.
                                    None
                                }
                            } else {
                                // The user is requesting all samples with a
                                // particular substring, but this sample doesn't
                                // have a metadata block at all, so it is not
                                // included.
                                None
                            }
                        }

                        // SAFETY: we filter any non-null and non-string values
                        // above.
                        _ => unreachable!(),
                    }
                } else {
                    // All other non-diagnosis fields.
                    let values: Option<Vec<Value>> =
                        match field.as_str() {
                            "age_at_diagnosis" => sample
                                .metadata()
                                .map(|metadata| {
                                    // SAFETY: metadata values should always be
                                    // convertable to [`serde_json::Value`]s.
                                    serde_json::to_value(
                                        metadata.age_at_diagnosis().map(|field| field.value()),
                                    )
                                    .unwrap()
                                })
                                .map(|age_at_diagnosis| vec![age_at_diagnosis]),
                            "disease_phase" => sample
                                .metadata()
                                .map(|metadata| {
                                    // SAFETY: metadata values should always be
                                    // convertable to [`serde_json::Value`]s.
                                    serde_json::to_value(
                                        metadata.disease_phase().map(|field| field.value()),
                                    )
                                    .unwrap()
                                })
                                .map(|disease_phase| vec![disease_phase]),
                            "tissue_type" => sample
                                .metadata()
                                .map(|metadata| {
                                    // SAFETY: metadata values should always be
                                    // convertable to [`serde_json::Value`]s.
                                    serde_json::to_value(
                                        metadata.tissue_type().map(|field| field.value()),
                                    )
                                    .unwrap()
                                })
                                .map(|tissue_type| vec![tissue_type]),
                            "tumor_classification" => sample
                                .metadata()
                                .map(|metadata| {
                                    // SAFETY: metadata values should always be
                                    // convertable to [`serde_json::Value`]s.
                                    serde_json::to_value(
                                        metadata.tumor_classification().map(|field| field.value()),
                                    )
                                    .unwrap()
                                })
                                .map(|tumor_classification| vec![tumor_classification]),
                            "tumor_tissue_morphology" => sample.metadata().map(|metadata| {
                                match metadata.tumor_tissue_morphology() {
                                    Some(tumor_tissue_morphology) => tumor_tissue_morphology
                                        .value()
                                        .as_map()
                                        .into_values()
                                        .map(Value::String)
                                        .collect::<Vec<_>>(),
                                    None => vec![Value::Null],
                                }
                            }),
                            "age_at_collection" => sample
                                .metadata()
                                .map(|metadata| {
                                    // SAFETY: metadata values should always be
                                    // convertable to [`serde_json::Value`]s.
                                    serde_json::to_value(
                                        metadata.age_at_collection().map(|field| field.value()),
                                    )
                                    .unwrap()
                                })
                                .map(|age_at_collection| vec![age_at_collection]),
                            "library_strategy" => sample
                                .metadata()
                                .map(|metadata| {
                                    // SAFETY: metadata values should always be
                                    // convertable to [`serde_json::Value`]s.
                                    serde_json::to_value(
                                        metadata.library_strategy().map(|field| field.value()),
                                    )
                                    .unwrap()
                                })
                                .map(|library_strategy| vec![library_strategy]),
                            "preservation_method" => sample
                                .metadata()
                                .map(|metadata| {
                                    // SAFETY: metadata values should always be
                                    // convertable to [`serde_json::Value`]s.
                                    serde_json::to_value(
                                        metadata.preservation_method().map(|field| field.value()),
                                    )
                                    .unwrap()
                                })
                                .map(|preservation_method| vec![preservation_method]),
                            "identifier" => {
                                sample.metadata().map(|metadata| {
                                    match metadata.identifiers().cloned() {
                                        Some(identifier) => identifier
                                            .into_iter()
                                            // SAFETY: metadata values should always be
                                            // convertable to [`serde_json::Value`]s.
                                            .map(|identifier| {
                                                let name = match identifier.value() {
                                                    Linked(identifier) => {
                                                        identifier.inner().name().to_string()
                                                    }
                                                    Unlinked(identifier) => identifier.to_string(),
                                                };

                                                Value::String(name)
                                            })
                                            .collect::<Vec<_>>(),
                                        None => vec![Value::Null],
                                    }
                                })
                            }
                            "deposition" => sample.metadata().map(|metadata| {
                                match metadata.common().depositions().cloned() {
                                    Some(depositions) => depositions
                                        .into_iter()
                                        .map(|accession| match accession {
                                            Accession::dbGaP(deposition) => {
                                                Value::String(deposition.into_inner())
                                            }
                                        })
                                        .collect::<Vec<_>>(),
                                    None => vec![Value::Null],
                                }
                            }),
                            _ => unreachable!("unhandled sample metadata field: {field}"),
                        };

                    match &parameter {
                        Value::Null => {
                            if let Some(values) = values {
                                if values.into_iter().any(|s| s.eq(&Value::Null)) {
                                    // The user is requesting all samples where
                                    // the associated field is `null`. This
                                    // sample has a metadata block, and at least
                                    // one of the values provided is
                                    // `Value::Null`, so we include the sample
                                    // in the results.
                                    Some(Ok(sample))
                                } else {
                                    // The user is requesting all samples where
                                    // the associated field is `null`.
                                    // Thissample has a metadata block, but none
                                    // of the values provided is `Value::Null`,
                                    // the sample is **not** included in the
                                    // results.
                                    None
                                }
                            } else {
                                // The user is requesting all samples where the
                                // associated field is `null`, and this sample
                                // has no metadata block, so the sample is
                                // included in the results.
                                Some(Ok(sample))
                            }
                        }
                        value => {
                            if let Some(values) = values {
                                if values.into_iter().any(|s| s.eq(value)) {
                                    // The user is requesting all samples where
                                    // the associated field has some value. This
                                    // sample has a metadata block, and at least
                                    // one of the values provided matches the
                                    // desired value, so the sample is included
                                    // in the results.
                                    Some(Ok(sample))
                                } else {
                                    // The user is requesting all samples where
                                    // the associated field has some value. This
                                    // sample has a metadata block, and at least
                                    // one of the values provided matches the
                                    // desired value, so the sample is **not**
                                    // included in the results.
                                    None
                                }
                            } else {
                                // The user is requesting all samples where the
                                // associated field has some value, but this
                                // sample has no metadata block, so the sample
                                // is **not** included in the results.
                                None
                            }
                        }
                    }
                }
            })
            .collect::<Result<Vec<_>, Errors>>()
    }
}
