//! Routes related to samples.

use std::sync::Mutex;

use actix_web::get;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::web::Query;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use indexmap::IndexMap;
use models::sample::Identifier;
use serde_json::Value;

use ccdi_cde as cde;
use ccdi_models as models;

use models::Sample;

use crate::paginate;
use crate::params::Pagination as PaginationParams;
use crate::responses::by;
use crate::responses::error;
use crate::responses::Errors;
use crate::responses::Samples;
use crate::responses::Summary;
use crate::routes::MISSING_GROUP_BY_KEY;
use crate::routes::NULL_GROUP_BY_KEY;

/// A store for [`Sample`]s.
#[derive(Debug)]
pub struct Store {
    samples: Mutex<Vec<Sample>>,
}

impl Store {
    /// Creates a new [`Store`] with randomized [`Sample`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::routes::sample::Store;
    ///
    /// let store = Store::random(100);
    /// ```
    pub fn random(count: usize) -> Self {
        Self {
            samples: Mutex::new(
                (0..count)
                    .map(|i| {
                        // SAFETY: this is manually crafted to never fail, so it can be
                        // unwrapped.
                        let identifier = Identifier::parse(
                            format!("organization:Sample{}", i + 1).as_ref(),
                            ":",
                        )
                        .unwrap();
                        let subject = cde::v1::subject::Identifier::parse(
                            format!("organization:Subject{}", i + 1).as_ref(),
                            ":",
                        )
                        .unwrap();

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
            .service(sample_show)
            .service(samples_by_count)
            .service(sample_summary);
    }
}

/// Gets the samples known by this server.
///
/// This endpoint supports pagination. Pagination is enabled by providing one of
/// the pagination-related query parameters below.
///
/// **Note:** please read the provided details on default sort order within the
/// `responses::Samples` schema, as that is a requirement of this endpoint.
#[utoipa::path(
    get,
    path = "/sample",
    tag = "Sample",
    params(PaginationParams),
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
                    - This header is required to exist when pagination is \
                    enabled.\n\
                    - It is also required _not_ to exist \
                    when pagination is not enabled.\n\
                    - When the header is present, this header must provide \
                    links for at least the `first` and `last` rels.\n \
                    - When the header is present, the `prev` and `next` links \
                    must exist only (a) when there are multiple pages in the \
                    result page set and (b) when the current page is not the \
                    first or last page, respectively.\n\
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
            example = json!(Errors::from(error::Kind::not_found(String::from("Samples"))))
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
pub async fn sample_index(params: Query<PaginationParams>, samples: Data<Store>) -> impl Responder {
    let mut samples = samples.samples.lock().unwrap().clone();

    // See the note in the documentation for this endpoint: the results must be
    // sorted by identifier by default.
    samples.sort();

    if params.0.page().is_some() || params.0.per_page().is_some() {
        paginate::response::<Sample, Samples>(params.0, samples, "http://localhost:8000/sample")
    } else {
        HttpResponse::Ok().json(Samples::from(samples))
    }
}

/// Gets the sample matching the provided name (if the sample exists).
#[utoipa::path(
    get,
    path = "/sample/{namespace}/{name}",
    params(
        (
            "namespace" = String,
            description = "The namespace portion of the sample identifier.",
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
#[get("/sample/{namespace}/{name}")]
pub async fn sample_show(path: Path<(String, String)>, samples: Data<Store>) -> impl Responder {
    let samples = samples.samples.lock().unwrap();
    let (namespace, name) = path.into_inner();

    samples
        .iter()
        .find(|sample| sample.id().namespace() == &namespace && sample.id().name() == &name)
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
        ("field" = String, description = "The field to group by and count."),
    ),
    tag = "Sample",
    responses(
        (status = 200, description = "Successful operation.", body = responses::by::count::Samples),
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
pub async fn samples_by_count(path: Path<String>, samples: Data<Store>) -> impl Responder {
    let samples = samples.samples.lock().unwrap().clone();
    let field = path.into_inner();

    let values = samples
        .iter()
        .map(|sample| parse_field(&field, sample))
        .collect::<Vec<_>>();

    let result = values
        .into_iter()
        .map(|value| match value {
            Some(Value::Null) => String::from(NULL_GROUP_BY_KEY),
            Some(Value::String(s)) => s,
            None => String::from(MISSING_GROUP_BY_KEY),
            _ => todo!(),
        })
        .fold(IndexMap::new(), |mut map, value| {
            *map.entry(value).or_insert_with(|| 0usize) += 1;
            map
        });

    if result.len() == 1 && result.get(MISSING_GROUP_BY_KEY).is_some() {
        return HttpResponse::UnprocessableEntity().json(Errors::from(
            error::Kind::unsupported_field(
                field,
                String::from("This field is not present for samples."),
            ),
        ));
    }

    HttpResponse::Ok().json(by::count::Samples::from(result))
}

fn parse_field(field: &str, sample: &Sample) -> Option<Value> {
    match field {
        "disease_phase" => match sample.metadata() {
            Some(metadata) => metadata
                .disease_phase()
                .as_ref()
                .map(|value| Value::String(value.value().to_string())),
            None => None,
        },
        "tissue_type" => match sample.metadata() {
            Some(metadata) => metadata
                .tissue_type()
                .as_ref()
                .map(|value| Value::String(value.value().to_string())),
            None => None,
        },
        "tumor_classification" => match sample.metadata() {
            Some(metadata) => metadata
                .tumor_classification()
                .as_ref()
                .map(|value| Value::String(value.value().to_string())),
            None => None,
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
