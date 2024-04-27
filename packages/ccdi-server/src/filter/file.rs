//! Filter parameters for [`File`]s.

use ccdi_models as models;

use models::metadata::common::deposition::Accession;
use models::File;
use serde_json::Value;

use crate::filter::FilterMetadataField;
use crate::params::filter::File as FilterFileParams;
use crate::responses::error::Kind;
use crate::responses::Errors;

impl FilterMetadataField<File, FilterFileParams> for Vec<File> {
    fn filter_metadata_field(
        self,
        field: String,
        params: &FilterFileParams,
    ) -> Result<Vec<File>, Errors> {
        let parameter = match field.as_str() {
            "type" => &params.r#type,
            "size" => &params.size,
            "checksum" => &params.checksum,
            "description" => &params.description,
            "deposition" => &params.deposition,
            _ => unreachable!("unhandled file metadata field: {field}"),
        };

        let parameter = match parameter {
            Some(parameter) => match parameter.parse::<Value>() {
                Ok(value) => value,
                Err(_) => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![field.to_string()]),
                        String::from("Parameter was not a valid JSON value."),
                    )]));
                }
            },
            None => {
                // If the parameter has no value, just return the original list of
                // files, as the user does not want to filter based on that.
                return Ok(self);
            }
        };

        let parameter = match field.as_str() {
            "type" => match parameter {
                Value::Null => Value::Null,
                Value::String(value) => Value::String(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("type")]),
                        String::from("Parameter was not a string or null."),
                    )]));
                }
            },
            "size" => match parameter {
                Value::Null => Value::Null,
                Value::Number(value) => Value::Number(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("size")]),
                        String::from("Parameter was not a number or null."),
                    )]));
                }
            },
            "checksum" => match parameter {
                Value::Null => Value::Null,
                Value::String(value) => Value::String(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("checksum")]),
                        String::from("Parameter was not a string or null."),
                    )]));
                }
            },
            "description" => match parameter {
                Value::Null => Value::Null,
                Value::String(value) => Value::String(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("description")]),
                        String::from("Parameter was not a string or null."),
                    )]));
                }
            },
            "deposition" => match parameter {
                Value::Null => Value::Null,
                Value::String(value) => Value::String(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("deposition")]),
                        String::from("Parameter was not a string or null."),
                    )]));
                }
            },
            _ => unreachable!("unhandled file metadata field: {field}"),
        };

        self.into_iter()
            .filter_map(|file| {
                if field.as_str() == "description" {
                    // Descriptions need to be treated specially, as _any_
                    // substring in the description should be able to be
                    // matched.
                    match &parameter {
                        Value::Null => {
                            if let Some(metadata) = file.metadata() {
                                if metadata.description().is_none() {
                                    // The user is requesting all files with a
                                    // `null` description, and the metadata
                                    // block is provided, but the value of the
                                    // description is `null`. Thus, this file
                                    // should be included.
                                    Some(Ok(file))
                                } else {
                                    // The user is requesting all files with a
                                    // `null` description, and the metadata
                                    // block is provided + the value of the
                                    // description is not `null`. Thus, this
                                    // file should **not** be included.
                                    None
                                }
                            } else {
                                // The user is requesting all files with a
                                // `null` description, and this file includes
                                // no metadata at all, so the file should be
                                // included.
                                Some(Ok(file))
                            }
                        }
                        Value::String(query) => {
                            if let Some(metadata) = file.metadata() {
                                if let Some(description) = metadata.description() {
                                    if description.to_string().contains(query) {
                                        // The user is requesting all files with
                                        // a particular substring, and this file
                                        // includes a metadata block + it has a
                                        // description + the description
                                        // contains the desired substring. Thus,
                                        // it is included.
                                        Some(Ok(file))
                                    } else {
                                        // The user is requesting all files with
                                        // a particular substring, and this file
                                        // includes a metadata block + it has a
                                        // description, but the description does
                                        // not contain the desired substring.
                                        // Thus, it is not included.
                                        None
                                    }
                                } else {
                                    // The user is requesting all files with
                                    // a particular substring, and this file
                                    // includes a metadata block, but it has
                                    // no description. Thus, the file is not
                                    // included.
                                    None
                                }
                            } else {
                                // The user is requesting all files with a
                                // particular substring, but this file doesn't
                                // have a metadata block at all, so it is not
                                // included.
                                None
                            }
                        }

                        // SAFETY: we filter any non-null and non-string values above.
                        _ => unreachable!(),
                    }
                } else {
                    // All other non-description fields.
                    let values: Option<Vec<Value>> = match field.as_str() {
                        "type" => file
                            .metadata()
                            .map(|metadata| {
                                // SAFETY: metadata values should always be
                                // convertable to [`serde_json::Value`]s.
                                serde_json::to_value(metadata.r#type().map(|r#type| r#type.value()))
                                    .unwrap()
                            })
                            .map(|r#type| vec![r#type]),
                        "size" => file
                            .metadata()
                            .map(|metadata| {
                                // SAFETY: metadata values should always be
                                // convertable to [`serde_json::Value`]s.
                                serde_json::to_value(metadata.size().map(|size| size.value()))
                                    .unwrap()
                            })
                            .map(|size| vec![size]),
                        "checksum" => file.metadata().map(|metadata| match metadata.checksums() {
                            Some(checksums) => checksums
                                .value()
                                .as_map()
                                .into_values()
                                .map(Value::String)
                                .collect::<Vec<_>>(),
                            None => vec![Value::Null],
                        }),
                        "deposition" => file.metadata().map(|metadata| {
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
                        _ => unreachable!("unhandled file metadata field: {field}"),
                    };

                    match &parameter {
                        Value::Null => {
                            if let Some(values) = values {
                                if values.into_iter().any(|s| s.eq(&Value::Null)) {
                                    // The user is requesting all files where the
                                    // associated field is `null`. This file has a
                                    // metadata block, and at least one of the
                                    // values provided is `Value::Null`, so we
                                    // include the file in the results.
                                    Some(Ok(file))
                                } else {
                                    // The user is requesting all files where
                                    // the associated field is `null`. This file
                                    // has a metadata block, but none of the
                                    // values provided is `Value::Null`, the
                                    // file is **not** included in the results.
                                    None
                                }
                            } else {
                                // The user is requesting all files where the
                                // associated field is `null`, and this file has
                                // no metadata block, so the file is included in
                                // the results.
                                Some(Ok(file))
                            }
                        }
                        value => {
                            if let Some(values) = values {
                                if values.into_iter().any(|s| s.eq(value)) {
                                    // The user is requesting all files where
                                    // the associated field has some value. This
                                    // file has a metadata block, and at least
                                    // one of the values provided matches the
                                    // desired value, so the file is included in
                                    // the results.
                                    Some(Ok(file))
                                } else {
                                    // The user is requesting all files where
                                    // the associated field has some value. This
                                    // file has a metadata block, and at least
                                    // one of the values provided matches the
                                    // desired value, so the file is **not**
                                    // included in the results.
                                    None
                                }
                            } else {
                                // The user is requesting all files where the
                                // associated field has some value, but this
                                // file has no metadata block, so the file is
                                // **not** included in the results.
                                None
                            }
                        }
                    }
                }
            })
            .collect::<Result<Vec<_>, Errors>>()
    }
}
