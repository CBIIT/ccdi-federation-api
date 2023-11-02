//! Routes related to samples.

use std::sync::Mutex;

use actix_web::get;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use indexmap::IndexMap;
use models::sample::Identifier;
use serde_json::Value;

use ccdi_models as models;

use models::Sample;

use crate::responses::by;
use crate::responses::Error;
use crate::responses::Samples;
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

                        Sample::random(identifier)
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
            .service(samples_by_count);
    }
}

/// Gets the samples known by this server.
#[utoipa::path(
    get,
    path = "/sample",
    tag = "Sample",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Samples)
    )
)]
#[get("/sample")]
pub async fn sample_index(samples: Data<Store>) -> impl Responder {
    let samples = samples.samples.lock().unwrap().clone();
    HttpResponse::Ok().json(Samples::from(samples))
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
            description = "Not found.",
            body = responses::Error,
            example = json!(Error::new("Sample with namespace 'foo' and name 'bar' not found."))
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
            HttpResponse::NotFound().json(Error::new(format!(
                "Sample with namespace '{}' and name '{}' not found.",
                namespace, name
            )))
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
            body = responses::Error,
            example = json!(Error::new("Field 'handedness' is unsupported."))
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
        return HttpResponse::UnprocessableEntity()
            .json(Error::new(format!("Field '{}' is not supported.", field)));
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
