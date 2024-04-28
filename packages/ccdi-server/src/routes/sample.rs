//! Routes related to samples.

use std::sync::Mutex;
use std::sync::MutexGuard;

use actix_web::get;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::web::Query;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::ResponseError;
use itertools::Itertools as _;
use models::namespace;
use models::sample::Identifier;
use rand::prelude::*;
use serde_json::Value;

use ccdi_models as models;

use models::Sample;

use crate::filter::filter;
use crate::paginate;
use crate::params::filter::Sample as FilterSampleParams;
use crate::params::PaginationParams;
use crate::params::PartitionParams;
use crate::params::Partitionable;
use crate::responses;
use crate::responses::by;
use crate::responses::by::count::sample::NamespacePartitionedResult;
use crate::responses::by::count::ValueCount;
use crate::responses::error;
use crate::responses::Errors;
use crate::responses::Samples;
use crate::responses::Summary;
use crate::routes::GroupByResults;

/// A store for [`Sample`]s.
#[derive(Debug)]
pub struct Store {
    /// The inner [`Samples`](ccdi_models::Sample).
    pub samples: Mutex<Vec<Sample>>,
}

impl Store {
    /// Creates a new [`Store`] with randomized [`Sample`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::routes::sample;
    /// use server::routes::subject;
    ///
    /// let subjects = subject::Store::random(100);
    /// let samples = sample::Store::random(100, subjects.subjects.lock().unwrap());
    /// ```
    pub fn random(count: usize, subjects: MutexGuard<'_, Vec<ccdi_models::Subject>>) -> Self {
        Self {
            samples: Mutex::new(
                (0..count)
                    .map(|i| {
                        let mut rng = rand::thread_rng();

                        // SAFETY: this should always unwrap because we manually
                        // ensure that subjects is never empty.
                        let subject = subjects.choose(&mut rng).unwrap().id().clone();

                        let identifier = Identifier::new(
                            subject.namespace().clone(),
                            format!("Sample{}", i + 1),
                        );

                        Sample::random(identifier, subject)
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

/// Configures the [`ServiceConfig`] with the sample paths.
pub fn configure(store: Data<Store>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(sample_index)
            .service(samples_by_count)
            .service(sample_show)
            .service(sample_summary);
    }
}

/// Gets the samples known by this server.
///
/// ### Pagination
///
/// This endpoint is paginated. Users may override the default pagination
/// parameters by providing one or more of the pagination-related query
/// parameters below.
///
/// ### Filtering
///
/// All harmonized (top-level) and unharmonized (nested under the
/// `metadata.unharmonized` key) metadata fields are filterable. Filtering is
/// achieved by assigning a valid JSON value to a query parameter named after
/// the field you want to filter by. The specific behavior of how the filter
/// works is field-specific and is defined in the query parameter descriptions
/// below.
///
/// ### Ordering
///
/// This endpoint has default ordering requirements—those details are documented
/// in the `responses::Samples` schema.
#[utoipa::path(
    get,
    path = "/sample",
    tag = "Sample",
    params(
        FilterSampleParams,
        (
            "metadata.unharmonized.<field>" = Option<String>,
            Query,
            nullable = false,
            description = "All unharmonized fields should be filterable as
            well:\n\n\
            * Filtering on a singular field should include the `Sample` in \
            the results if the query exactly matches the value of that field \
            for the `Sample` (case-sensitive).\n\
            * Filtering on field with multiple values should include the \
            `Sample` in the results if the query exactly matches any of the \
            values of the field for that `Sample` (case-sensitive).\n\
            * Unlike harmonized fields, unharmonized fields must be prefixed \
            with `metadata.unharmonized`.\n\n\
            **Note:** this query parameter is intended to be symbolic of any \
            unharmonized field. Because of limitations within Swagger UI, it \
            will show up as a query parameter that can be optionally be \
            submitted as part of a request within Swagger UI. Please keep in \
            mind that the literal query parameter \
            `?metadata.unharmonized.<field>=value` is not supported, so \
            attempting to use it within Swagger UI will not work!"
        ),
        PaginationParams,
    ),
    responses(
        (
            status = 200,
            description = "Successful operation.",
            body = responses::Samples,
            headers(
                (
                    "link" = String,
                    description = "Links to URLs that may be of interest \
                    when paging through paginated responses. This header \
                    contains two or more links of interest. The format of the \
                    field is as follows: \
                    \n\
                    \n`Link: <URL>; rel=\"REL\"` \
                    \n\
                    ### Relationships\n\n\
                    In the format above, `URL` represents a valid URL for \
                    the link of interest and `REL` is one of four values: \n\
                    - `first` (_Required_). A link to the first page in the \
                    results (can be the same as `last` if there is only one \
                    page).\n\
                    - `last` (_Required_). A link to the first page in the \
                    results (can be the same as `first` if there is only one \
                    page).\n\
                    - `next` (_Optional_). A link to the next page (if it \
                    exists).\n\
                    - `prev` (_Optional_). A link to the previous page (if it \
                    exists).\n\n\
                    ### Requirements\n\n\
                    - This header _must_ provide links for at least the `first` \
                    and `last` rels.\n \
                    - The `prev` and `next` links must exist only (a) when there \
                    are multiple pages in the result page set and (b) when the \
                    current page is not the first or last page, respectively.\n\
                    - This list of links is unordered.\n\n \
                    ### Notes\n\n\
                    - HTTP 1.1 and HTTP 2.0 dictate that response \
                    headers are case insensitive. Though not required, we \
                    recommend an all lowercase name of `link` for this \
                    response header."
                )
            )
        ),
        (
            status = 404,
            description = "Not found.\nServers that cannot provide line-level \
            data should use this response rather than Forbidden (403), as \
            there is no level of authorization that would allow one to access \
            the information included in the API.",
            body = responses::Errors,
            example = json!(
                Errors::from(
                    error::Kind::unshareable_data(
                        String::from("samples"),
                        String::from(
                            "Our agreement with data providers prohibits us from sharing \
                            line-level data."
                        ),
                    )
                )
            )
        ),
        (
            status = 422,
            description = "Invalid query or path parameters.",
            body = responses::Errors,
            example = json!(Errors::from(error::Kind::invalid_parameters(
                Some(vec![String::from("page"), String::from("per_page")]),
                String::from("unable to calculate offset")
            )))
        ),
    )
)]
#[get("/sample")]
pub async fn sample_index(
    filter_params: Query<FilterSampleParams>,
    pagination_params: Query<PaginationParams>,
    samples: Data<Store>,
) -> impl Responder {
    let mut samples = samples.samples.lock().unwrap().clone();

    // See the note in the documentation for this endpoint: the results must be
    // sorted by identifier by default.
    samples.sort();

    let samples = match filter::<Sample, FilterSampleParams>(samples, filter_params.0) {
        Ok(samples) => samples,
        Err(err) => return err.error_response(),
    };

    paginate::response::<Sample, Samples>(
        pagination_params.0,
        samples,
        "http://localhost:8000/sample",
    )
}

/// Gets the sample matching the provided name (if the sample exists).
#[utoipa::path(
    get,
    path = "/sample/{organization}/{namespace}/{name}",
    params(
        (
            "organization" = String,
            description = "The organization identifier of the namespace to which the sample belongs.",
        ),
        (
            "namespace" = String,
            description = "The name of the namespace to which the sample belongs.",
        ),
        (
            "name" = String,
            description = "The name portion of the sample identifier."
        )
    ),
    tag = "Sample",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Sample),
        (
            status = 404,
            description = "Not found.\nServers that cannot provide line-level \
            data should use this response rather than Forbidden (403), as \
            there is no level of authorization that would allow one to access \
            the information included in the API.",
            body = responses::Errors,
            example = json!(Errors::from(error::Kind::not_found(
                String::from("Sample with namespace 'foo' and name 'bar'")
            )))
        )
    )
)]
#[get("/sample/{organization}/{namespace}/{name}")]
pub async fn sample_show(
    path: Path<(String, String, String)>,
    samples: Data<Store>,
) -> impl Responder {
    let samples = samples.samples.lock().unwrap();
    let (organization, namespace, name) = path.into_inner();

    samples
        .iter()
        .find(|sample| {
            sample.id().namespace().organization().as_str() == organization
                && sample.id().namespace().name().as_str() == namespace
                && sample.id().name() == name
        })
        .map(|sample| HttpResponse::Ok().json(sample))
        .unwrap_or_else(|| {
            HttpResponse::NotFound().json(Errors::from(error::Kind::not_found(format!(
                "Sample with namespace '{}' and name '{}'",
                namespace, name
            ))))
        })
}

/// Groups the samples by the specified metadata field and returns counts.
#[utoipa::path(
    get,
    path = "/sample/by/{field}/count",
    params(
        ("field" = String, description = "The field to group by and count with."),
        PartitionParams,
    ),
    tag = "Sample",
    responses(
        (status = 200, description = "Successful operation.", body = responses::by::count::sample::Response),
        (
            status = 422,
            description = "Unsupported field.",
            body = responses::Errors,
            example = json!(Errors::from(
                error::Kind::unsupported_field(
                    String::from("handedness"),
                    String::from("This field is not present for samples."),
                )
            ))
        ),
    )
)]
#[get("/sample/by/{field}/count")]
pub async fn samples_by_count(
    path: Path<String>,
    partition_params: Query<PartitionParams>,
    samples: Data<Store>,
) -> impl Responder {
    let samples = samples.samples.lock().unwrap().clone();
    let field = path.into_inner();

    if let Some(Partitionable::Namespace) = partition_params.0.partition {
        let namespaces = samples
            .iter()
            .map(|s| s.id().namespace())
            .unique()
            .cloned()
            .sorted()
            .collect::<Vec<namespace::Identifier>>();

        let mut results = Vec::new();

        for namespace in namespaces {
            let namespace_samples = samples
                .iter()
                .filter(|s| s.id().namespace() == &namespace)
                .cloned()
                .collect::<Vec<Sample>>();

            let namespace_results = group_by(namespace_samples, &field);

            match namespace_results {
                GroupByResults::Supported(namespace_results) => {
                    let namespace_partitioned_result =
                        NamespacePartitionedResult::new(namespace.clone(), namespace_results);

                    results.push(namespace_partitioned_result);
                }
                GroupByResults::Unsupported => {
                    return HttpResponse::UnprocessableEntity().json(Errors::from(
                        error::Kind::unsupported_field(
                            field.to_string(),
                            String::from("This field is not present for samples."),
                        ),
                    ));
                }
            }
        }

        HttpResponse::Ok().json(by::count::sample::Response::NamespacePartitioned(
            by::count::sample::NamespacePartitionedResults::from(results),
        ))
    } else {
        let results = group_by(samples, &field);

        match results {
            GroupByResults::Supported(results) => {
                HttpResponse::Ok().json(by::count::sample::Response::Unpartitioned(results))
            }
            GroupByResults::Unsupported => HttpResponse::UnprocessableEntity().json(Errors::from(
                error::Kind::unsupported_field(
                    field.to_string(),
                    String::from("This field is not present for samples."),
                ),
            )),
        }
    }
}

fn group_by(
    samples: Vec<Sample>,
    field: &str,
) -> GroupByResults<responses::by::count::sample::Results> {
    let values: Vec<Option<Option<Value>>> = samples
        .iter()
        .map(|sample| parse_field(field, sample))
        .collect::<Vec<_>>();

    if values.iter().any(|value| value.is_none()) {
        return GroupByResults::Unsupported;
    }

    let values = values
        .into_iter()
        // SAFETY: we just checked above to ensure that none of the values are
        // [`None`].
        .map(|value| value.unwrap())
        .collect::<Vec<_>>();

    let mut missing_values = 0usize;
    let mut result = values
        .into_iter()
        .flat_map(|value| match value {
            Some(value) => Some(value),
            None => {
                missing_values += 1;
                None
            }
        })
        .fold(Vec::new(), |mut acc: Vec<ValueCount>, value| {
            match acc.iter_mut().find(|result| result.value == value) {
                Some(result) => result.count += 1,
                None => acc.push(ValueCount { value, count: 1 }),
            }
            acc
        });

    // NOTE: the `std::cmp::Reverse` here is used to sort the values in
    // descending order.
    result.sort_by_key(|value| std::cmp::Reverse(value.count));

    GroupByResults::Supported(responses::by::count::sample::Results::new(
        result,
        missing_values,
    ))
}

fn parse_field(field: &str, sample: &Sample) -> Option<Option<Value>> {
    match field {
        "age_at_diagnosis" => match sample.metadata() {
            Some(metadata) => Some(
                metadata
                    .age_at_diagnosis()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|age_at_diagnosis| serde_json::to_value(age_at_diagnosis.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "age_at_collection" => match sample.metadata() {
            Some(metadata) => Some(
                metadata
                    .age_at_collection()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|age_at_collection| {
                        serde_json::to_value(age_at_collection.value()).unwrap()
                    })
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "disease_phase" => match sample.metadata() {
            Some(metadata) => Some(
                metadata
                    .disease_phase()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|disease_phase| serde_json::to_value(disease_phase.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "library_strategy" => match sample.metadata() {
            Some(metadata) => Some(
                metadata
                    .library_strategy()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|library_strategy| serde_json::to_value(library_strategy.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "preservation_method" => match sample.metadata() {
            Some(metadata) => Some(
                metadata
                    .preservation_method()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|preservation_method| {
                        serde_json::to_value(preservation_method.value()).unwrap()
                    })
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "tissue_type" => match sample.metadata() {
            Some(metadata) => Some(
                metadata
                    .tissue_type()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|tissue_type| serde_json::to_value(tissue_type.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "tumor_classification" => match sample.metadata() {
            Some(metadata) => Some(
                metadata
                    .tumor_classification()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|tumor_classification| {
                        serde_json::to_value(tumor_classification.value()).unwrap()
                    })
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "tumor_tissue_morphology" => match sample.metadata() {
            Some(metadata) => Some(
                metadata
                    .tumor_tissue_morphology()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|tumor_tissue_morphology| {
                        serde_json::to_value(tumor_tissue_morphology.value()).unwrap()
                    })
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "depositions" => match sample.metadata() {
            Some(metadata) => Some(
                metadata
                    .common()
                    .depositions()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|deposition| serde_json::to_value(deposition).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        _ => None,
    }
}

/// Reports summary information for the samples known by this server.
#[utoipa::path(
    get,
    path = "/sample/summary",
    tag = "Sample",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Summary),
    )
)]
#[get("/sample/summary")]
pub async fn sample_summary(samples: Data<Store>) -> impl Responder {
    let samples = samples.samples.lock().unwrap().clone();
    HttpResponse::Ok().json(Summary::new(samples.len()))
}

#[cfg(test)]
mod tests {
    use crate::routes::namespace::random_namespace;

    #[test]
    fn it_generates_a_random_namespace() {
        random_namespace();
    }
}
