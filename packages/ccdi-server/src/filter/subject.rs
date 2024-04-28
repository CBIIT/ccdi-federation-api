//! Filter parameters for [`Subject`]s.

use ccdi_models as models;

use models::metadata::common::deposition::Accession;
use models::subject::identifier::referenced::Identifier::Linked;
use models::subject::identifier::referenced::Identifier::Unlinked;
use models::Subject;
use serde_json::Value;

use crate::filter::FilterMetadataField;
use crate::params::filter::Subject as FilterSubjectParams;
use crate::responses::error::Kind;
use crate::responses::Errors;

impl FilterMetadataField<Subject, FilterSubjectParams> for Vec<Subject> {
    fn filter_metadata_field(
        self,
        field: String,
        params: &FilterSubjectParams,
    ) -> Result<Vec<Subject>, Errors> {
        let parameter = match field.as_str() {
            "sex" => &params.sex,
            "race" => &params.race,
            "ethnicity" => &params.ethnicity,
            "identifier" => &params.identifier,
            "vital_status" => &params.vital_status,
            "age_at_vital_status" => &params.age_at_vital_status,
            "deposition" => &params.deposition,
            _ => unreachable!("unhandled subject metadata field: {field}"),
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
                // If the parameter has no value, just return the original list
                // of subjects, as the user does not want to filter based on
                // that.
                return Ok(self);
            }
        };

        let parameter = match field.as_str() {
            "sex" => match parameter {
                Value::Null => Value::Null,
                Value::String(value) => Value::String(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("sex")]),
                        String::from("Parameter was not a string or null."),
                    )]));
                }
            },
            "race" => match parameter {
                Value::Null => Value::Null,
                Value::String(value) => Value::String(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("race")]),
                        String::from("Parameter was not a string or null."),
                    )]));
                }
            },
            "ethnicity" => match parameter {
                Value::Null => Value::Null,
                Value::String(value) => Value::String(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("ethnicity")]),
                        String::from("Parameter was not a string or null."),
                    )]));
                }
            },
            "identifier" => match parameter {
                Value::Null => Value::Null,
                Value::String(value) => Value::String(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("identifier")]),
                        String::from("Parameter was not a string or null."),
                    )]));
                }
            },
            "vital_status" => match parameter {
                Value::Null => Value::Null,
                Value::String(value) => Value::String(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("vital_status")]),
                        String::from("Parameter was not a string or null."),
                    )]));
                }
            },
            "age_at_vital_status" => match parameter {
                Value::Null => Value::Null,
                Value::Number(value) => Value::Number(value.to_owned()),
                _ => {
                    return Err(Errors::new(vec![Kind::invalid_parameters(
                        Some(vec![String::from("age_at_vital_status")]),
                        String::from("Parameter was not a number or null."),
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
            _ => unreachable!("unhandled subject metadata field: {field}"),
        };

        self.into_iter()
            .filter_map(|subject| {
                let values: Option<Vec<Value>> = match field.as_str() {
                    "sex" => subject
                        .metadata()
                        .map(|metadata| {
                            // SAFETY: metadata values should always be
                            // convertable to [`serde_json::Value`]s.
                            serde_json::to_value(metadata.sex().map(|field| field.value())).unwrap()
                        })
                        .map(|sex| vec![sex]),
                    "race" => subject
                        .metadata()
                        .map(|metadata| match metadata.race().cloned() {
                            Some(race) => race
                                .into_iter()
                                // SAFETY: metadata values should always be
                                // convertable to [`serde_json::Value`]s.
                                .map(|race| serde_json::to_value(race.value()).unwrap())
                                .collect::<Vec<_>>(),
                            None => vec![Value::Null],
                        }),
                    "ethnicity" => subject
                        .metadata()
                        .map(|metadata| {
                            // SAFETY: metadata values should always be
                            // convertable to [`serde_json::Value`]s.
                            serde_json::to_value(metadata.ethnicity().map(|field| field.value()))
                                .unwrap()
                        })
                        .map(|ethnicity| vec![ethnicity]),
                    "identifier" => {
                        subject
                            .metadata()
                            .map(|metadata| match metadata.identifiers().cloned() {
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
                            })
                    }
                    "vital_status" => subject
                        .metadata()
                        .map(|metadata| {
                            // SAFETY: metadata values should always be
                            // convertable to [`serde_json::Value`]s.
                            serde_json::to_value(metadata.vital_status().map(|field| field.value()))
                                .unwrap()
                        })
                        .map(|vital_status| vec![vital_status]),
                    "age_at_vital_status" => subject
                        .metadata()
                        .map(|metadata| {
                            // SAFETY: metadata values should always be
                            // convertable to [`serde_json::Value`]s.
                            serde_json::to_value(
                                metadata.age_at_vital_status().map(|field| field.value()),
                            )
                            .unwrap()
                        })
                        .map(|age_at_vital_status| vec![age_at_vital_status]),
                    "deposition" => subject.metadata().map(|metadata| {
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
                    _ => unreachable!("unhandled subject metadata field: {field}"),
                };

                match &parameter {
                    Value::Null => {
                        if let Some(values) = values {
                            if values.into_iter().any(|s| s.eq(&Value::Null)) {
                                // The user is requesting all subjects where the
                                // associated field is `null`. This subject has
                                // a metadata block, and at least one of the
                                // values provided is `Value::Null`, so we
                                // include the subject in the results.
                                Some(Ok(subject))
                            } else {
                                // The user is requesting all subjects where the
                                // associated field is `null`. Thissubject has a
                                // metadata block, but none of the values
                                // provided is `Value::Null`, the subject is
                                // **not** included in the results.
                                None
                            }
                        } else {
                            // The user is requesting all subjects where the
                            // associated field is `null`, and this subject has
                            // no metadata block, so the subject is included in
                            // the results.
                            Some(Ok(subject))
                        }
                    }
                    value => {
                        if let Some(values) = values {
                            if values.into_iter().any(|s| s.eq(value)) {
                                // The user is requesting all subjects where the
                                // associated field has some value. This subject
                                // has a metadata block, and at least one of the
                                // values provided matches the desired value, so
                                // the subject is included in the results.
                                Some(Ok(subject))
                            } else {
                                // The user is requesting all subjects where the
                                // associated field has some value. This subject
                                // has a metadata block, and at least one of the
                                // values provided matches the desired value, so
                                // the subject is **not** included in the
                                // results.
                                None
                            }
                        } else {
                            // The user is requesting all subjects where the
                            // associated field has some value, but this subject
                            // has no metadata block, so the subject is **not**
                            // included in the results.
                            None
                        }
                    }
                }
            })
            .collect::<Result<Vec<_>, Errors>>()
    }
}
