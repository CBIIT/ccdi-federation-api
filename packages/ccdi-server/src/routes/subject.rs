//! Routes related to subjects.

use std::sync::Mutex;

use actix_web::get;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use indexmap::IndexMap;
use models::metadata::field::RacesOrNull;
use models::metadata::field::SexOrNull;
use serde_json::Value;

use ccdi_cde as cde;
use ccdi_models as models;

use cde::v1::Identifier;
use models::Subject;

use crate::responses::by;
use crate::responses::Error;
use crate::responses::Subjects;

/// A store for [`Subject`]s.
#[derive(Debug)]
pub struct Store {
    subjects: Mutex<Vec<Subject>>,
}

impl Store {
    /// Creates a new [`Store`] with randomized [`Subject`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::routes::subject::Store;
    ///
    /// let store = Store::random(100);
    /// ```
    pub fn random(count: usize) -> Self {
        Self {
            subjects: Mutex::new(
                (0..count)
                    .map(|i| {
                        // SAFETY: this is manually crafted to never fail, so it can be
                        // unwrapped.
                        let identifier = Identifier::parse(
                            format!("organization:Subject{}", i + 1).as_ref(),
                            ":",
                        )
                        .unwrap();

                        Subject::random(identifier)
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

/// Configures the [`ServiceConfig`] with the subject paths.
pub fn configure(store: Data<Store>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(index)
            .service(show)
            .service(subjects_by_count);
    }
}

/// Gets the subjects known by this server.
#[utoipa::path(
    get,
    path = "/subject",
    tag = "Subject",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Subjects)
    )
)]
#[get("/subject")]
pub async fn index(subjects: Data<Store>) -> impl Responder {
    let subjects = subjects.subjects.lock().unwrap().clone();
    HttpResponse::Ok().json(Subjects::from(subjects))
}

/// Gets the subject matching the provided id (if it exists).
#[utoipa::path(
    get,
    path = "/subject/{namespace}/{name}",
    params(
        (
            "namespace" = String,
            description = "The namespace portion of the subject identifier.",
        ),
        (
            "name" = String,
            description = "The name portion of the subject identifier."
        )
    ),
    tag = "Subject",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Subject),
        (
            status = 404,
            description = "Not found.",
            body = responses::Error,
            example = json!(Error::new("Subject with namespace 'foo' and name 'bar' not found."))
        )
    )
)]
#[get("/subject/{namespace}/{name}")]
pub async fn show(path: Path<(String, String)>, subjects: Data<Store>) -> impl Responder {
    let subjects = subjects.subjects.lock().unwrap();
    let (namespace, name) = path.into_inner();

    subjects
        .iter()
        .find(|subject| subject.id().namespace() == &namespace && subject.id().name() == &name)
        .map(|subject| HttpResponse::Ok().json(subject))
        .unwrap_or_else(|| {
            HttpResponse::NotFound().json(Error::new(format!(
                "Subject with namespace '{}' and name '{}' not found.",
                namespace, name
            )))
        })
}

/// Groups the subjects by the specified metadata field and returns counts.
#[utoipa::path(
    get,
    path = "/subject/by/{field}/count",
    params(
        ("field" = String, description = "The field to group by and count."),
    ),
    tag = "Subject",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Subject),
        (
            status = 422,
            description = "Unsupported field.",
            body = responses::Error,
            example = json!(Error::new("Field 'handedness' is unsupported."))
        ),
    )
)]
#[get("/subject/by/{field}/count")]
pub async fn subjects_by_count(path: Path<String>, subjects: Data<Store>) -> impl Responder {
    let subjects = subjects.subjects.lock().unwrap().clone();
    let field = path.into_inner();

    let values = subjects
        .iter()
        .map(|subject| parse_field(&field, subject))
        .collect::<Option<Vec<_>>>();

    let result = match values {
        Some(values) => values
            .into_iter()
            .map(|value| match value {
                Value::Null => String::from("null"),
                Value::String(s) => s,
                _ => todo!(),
            })
            .fold(IndexMap::new(), |mut map, value| {
                *map.entry(value).or_insert_with(|| 0usize) += 1;
                map
            }),
        None => {
            return HttpResponse::UnprocessableEntity()
                .json(Error::new(format!("Field '{}' is not supported.", field)))
        }
    };

    HttpResponse::Ok().json(by::count::Subjects::from(result))
}

fn parse_field(field: &str, subject: &Subject) -> Option<Value> {
    match field {
        "sex" => match subject.metadata() {
            Some(metadata) => match metadata.sex() {
                SexOrNull::Unowned(unowned) => Some(Value::String(unowned.value().to_string())),
                SexOrNull::Null => Some(Value::Null),
            },
            None => None,
        },
        "race" => match subject.metadata() {
            Some(metadata) => match metadata.race() {
                RacesOrNull::MultipleUnowned(unowned) => Some(Value::String(
                    unowned
                        .iter()
                        .map(|race| race.value().to_string())
                        .collect::<Vec<_>>()
                        .join(" AND "),
                )),
                RacesOrNull::Null => Some(Value::Null),
            },
            None => None,
        },
        _ => None,
    }
}
