//! Routes related to subjects.

use std::sync::Mutex;

use actix_web::get;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::web::Query;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use serde_json::Value;

use ccdi_cde as cde;
use ccdi_models as models;

use models::subject::Identifier;
use models::Subject;

use crate::filter::filter;
use crate::paginate;
use crate::params::filter::Subject as FilterSubjectParams;
use crate::params::PaginationParams;
use crate::responses;
use crate::responses::by::count::ValueCount;
use crate::responses::error;
use crate::responses::Errors;
use crate::responses::Subjects;
use crate::responses::Summary;
use crate::routes::namespace::random_namespace;
use crate::routes::GroupByResults;

/// A store for [`Subject`]s.
#[derive(Debug)]
pub struct Store {
    /// The inner [`Subjects`](ccdi_models::Subject).
    pub subjects: Mutex<Vec<Subject>>,
}

impl Store {
    /// Creates a new [`Store`] with randomized [`Subject`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::routes::subject;
    ///
    /// let subjects = subject::Store::random(100);
    /// ```
    pub fn random(count: usize) -> Self {
        Self {
            subjects: Mutex::new(
                (0..count)
                    .map(|i| {
                        let identifier = Identifier::new(
                            random_namespace().id().clone(),
                            cde::v1::subject::Name::new(format!("Subject{}", i + 1)),
                        );

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
            .service(subject_index)
            .service(subjects_by_count)
            .service(subject_show)
            .service(subject_summary);
    }
}

/// Gets the subjects known by this server.
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
/// `metadata.unharmonized` key) metadata fields are filterable. To achieve
/// this, you can provide the field name as a [`String`]. Filtering follows the
/// following rules:
///
/// * For single-value metadata field, the subject is included in the results if
///   its value _exactly_ matches the query string. Matches are case-sensitive.
/// * For multiple-value metadata fields, the subject is included in the results
///   if any of its values for the field _exactly_ match the query string (a
///   logical OR [`||`]). Matches are case-sensitive.
/// * When the metadata field is `null` (in the case of singular or
///   multiple-valued metadata fields) or empty, the subject is not included.
/// * When multiple fields are provided as filters, a logical AND (`&&`) strings
///   together the predicates. In other words, all filters must match for a
///   subject to be returned. Note that this means that servers do not natively
///   support logical OR (`||`) across multiple fields: that must be done by
///   calling this endpoint with each of your desired queries and performing a
///   set union of those subjects out of band.
///
/// ### Ordering
///
/// This endpoint has default ordering requirements—those details are documented
/// in the `responses::Subjects` schema.
#[utoipa::path(
    get,
    path = "/subject",
    tag = "Subject",
    params(
        FilterSubjectParams,
        (
            "metadata.unharmonized.<field>" = Option<String>,
            Query,
            nullable = false,
            description = "All unharmonized fields should be filterable in the \
            same manner as harmonized fields:\n\n\
            * Filtering on a singular field should include the `Subject` in \
            the results if the query exactly matches the value of that field \
            for the `Subject` (case-sensitive).\n\
            * Filtering on field with multiple values should include the \
            `Subject` in the results if the query exactly matches any of the \
            values of the field for that `Subject` (case-sensitive).\n\
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
            body = responses::Subjects,
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
                        String::from("subjects"),
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
#[get("/subject")]
pub async fn subject_index(
    filter_params: Query<FilterSubjectParams>,
    pagination_params: Query<PaginationParams>,
    subjects: Data<Store>,
) -> impl Responder {
    let mut subjects = subjects.subjects.lock().unwrap().clone();

    // See the note in the documentation for this endpoint: the results must be
    // sorted by identifier by default.
    subjects.sort();

    let subjects = filter::<Subject, FilterSubjectParams>(subjects, filter_params.0);

    paginate::response::<Subject, Subjects>(
        pagination_params.0,
        subjects,
        "http://localhost:8000/subject",
    )
}

/// Gets the subject matching the provided id (if the subject exists).
#[utoipa::path(
    get,
    path = "/subject/{organization}/{namespace}/{name}",
    params(
        (
            "organization" = String,
            description = "The organization identifier of the namespace to which the subject belongs.",
        ),
        (
            "namespace" = String,
            description = "The name of the namespace to which the subject belongs.",
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
            description = "Not found.\nServers that cannot provide line-level \
            data should use this response rather than Forbidden (403), as \
            there is no level of authorization that would allow one to access \
            the information included in the API.",
            body = responses::Errors,
            example = json!(Errors::from(error::Kind::not_found(String::from("Subjects"))))
        )
    )
)]
#[get("/subject/{organization}/{namespace}/{name}")]
pub async fn subject_show(
    path: Path<(String, String, String)>,
    subjects: Data<Store>,
) -> impl Responder {
    let subjects = subjects.subjects.lock().unwrap();
    let (organization, namespace, name) = path.into_inner();

    subjects
        .iter()
        .find(|subject| {
            subject.id().namespace().organization().as_str() == organization
                && subject.id().namespace().name().as_str() == namespace
                && subject.id().name().as_str() == name
        })
        .map(|subject| HttpResponse::Ok().json(subject))
        .unwrap_or_else(|| {
            HttpResponse::NotFound().json(Errors::from(error::Kind::not_found(format!(
                "Subject with namespace '{namespace}' and name '{name}'"
            ))))
        })
}

/// Groups the subjects by the specified metadata field and returns counts.
#[utoipa::path(
    get,
    path = "/subject/by/{field}/count",
    params(
        ("field" = String, description = "The field to group by and count with."),
    ),
    tag = "Subject",
    responses(
        (status = 200, description = "Successful operation.", body = responses::by::count::subject::Results),
        (
            status = 422,
            description = "Unsupported field.",
            body = responses::Errors,
            example = json!(Errors::from(
                error::Kind::unsupported_field(
                    String::from("handedness"),
                    String::from("This field is not present for subjects."),
                )
            ))
        ),
    )
)]
#[get("/subject/by/{field}/count")]
pub async fn subjects_by_count(path: Path<String>, subjects: Data<Store>) -> impl Responder {
    let subjects = subjects.subjects.lock().unwrap().clone();
    let field = path.into_inner();

    let results = group_by(subjects, &field);

    match results {
        GroupByResults::Supported(results) => HttpResponse::Ok().json(results),
        GroupByResults::Unsupported => {
            HttpResponse::UnprocessableEntity().json(Errors::from(error::Kind::unsupported_field(
                field.to_string(),
                String::from("This field is not present for subjects."),
            )))
        }
    }
}

fn group_by(
    subjects: Vec<Subject>,
    field: &str,
) -> GroupByResults<responses::by::count::subject::Results> {
    let values = subjects
        .iter()
        .map(|subject| parse_field(field, subject))
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
    let result = values
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

    GroupByResults::Supported(responses::by::count::subject::Results::new(
        result,
        missing_values,
    ))
}

fn parse_field(field: &str, subject: &Subject) -> Option<Option<Value>> {
    match field {
        "sex" => match subject.metadata() {
            Some(metadata) => Some(
                metadata
                    .sex()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|sex| serde_json::to_value(sex.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "race" => match subject.metadata() {
            Some(metadata) => Some(
                metadata
                    .race()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|race| serde_json::to_value(race).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "ethnicity" => match subject.metadata() {
            Some(metadata) => Some(
                metadata
                    .ethnicity()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|ethnicity| serde_json::to_value(ethnicity.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "identifiers" => match subject.metadata() {
            Some(metadata) => Some(
                metadata
                    .identifiers()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|identifiers| serde_json::to_value(identifiers).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "vital_status" => match subject.metadata() {
            Some(metadata) => Some(
                metadata
                    .vital_status()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|vital_status| serde_json::to_value(vital_status.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "age_at_vital_status" => match subject.metadata() {
            Some(metadata) => Some(
                metadata
                    .age_at_vital_status()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|age_at_vital_status| {
                        serde_json::to_value(age_at_vital_status.value()).unwrap()
                    })
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "associated_diagnoses" => match subject.metadata() {
            Some(metadata) => Some(
                metadata
                    .associated_diagnoses()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|associated_diagnoses| serde_json::to_value(associated_diagnoses).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "depositions" => match subject.metadata() {
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

/// Reports summary information for the subjects known by this server.
#[utoipa::path(
    get,
    path = "/subject/summary",
    tag = "Subject",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Summary),
    )
)]
#[get("/subject/summary")]
pub async fn subject_summary(subjects: Data<Store>) -> impl Responder {
    let subjects = subjects.subjects.lock().unwrap().clone();
    HttpResponse::Ok().json(Summary::new(subjects.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_a_random_namespace() {
        random_namespace();
    }
}
