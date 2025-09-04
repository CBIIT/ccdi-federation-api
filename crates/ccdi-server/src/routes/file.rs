//! Routes related to files.

use std::sync::Mutex;
use std::sync::MutexGuard;

use actix_web::get;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::web::Query;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use ccdi_cde::v1::file;
use rand::prelude::*;

use ccdi_models as models;

use models::file::Identifier;
use models::File;
use serde_json::Value;

use crate::filter::filter;
use crate::paginate;
use crate::params::filter::File as FilterFileParams;
use crate::params::PaginationParams;
use crate::responses;
use crate::responses::by::count::ValueCount;
use crate::responses::error;
use crate::responses::Errors;
use crate::responses::Files;
use crate::responses::Summary;
use crate::routes::GroupByResults;

/// A store for [`File`]s.
#[derive(Debug)]
pub struct Store {
    /// The inner [`Files`](ccdi_models::File).
    pub files: Mutex<Vec<File>>,
}

impl Store {
    /// Creates a new [`Store`] with randomized [`File`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::routes::file;
    /// use server::routes::sample;
    /// use server::routes::subject;
    ///
    /// let subjects = subject::Store::random(100);
    /// let samples = sample::Store::random(100, subjects.subjects.lock().unwrap());
    /// let files = file::Store::random(100, samples.samples.lock().unwrap());
    /// ```
    pub fn random(count: usize, samples: MutexGuard<'_, Vec<ccdi_models::Sample>>) -> Self {
        Self {
            files: Mutex::new(
                (0..count)
                    .map(|i| {
                        let mut rng = rand::thread_rng();

                        // SAFETY: this should always unwrap because we manually ensure
                        // that subjects is never empty.
                        let sample = samples.choose(&mut rng).unwrap().id().clone();

                        let identifier = Identifier::new(
                            sample.namespace().clone(),
                            file::Name::new(format!("File{}.txt", i + 1)),
                        );

                        File::random(identifier, sample)
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

/// Configures the [`ServiceConfig`] with the file paths.
pub fn configure(store: Data<Store>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(file_index)
            .service(files_by_count)
            .service(file_show)
            .service(file_summary);
    }
}

/// Gets the files known by this server.
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
/// * For single-value metadata field, the file is included in the results if
///   its value _exactly_ matches the query string. Matches are case-sensitive.
/// * For multiple-value metadata fields, the file is included in the results
///   if any of its values for the field _exactly_ match the query string (a
///   logical OR (`||`)). Matches are case-sensitive.
/// * When the metadata field is `null` (in the case of singular or
///   multiple-valued metadata fields) or empty, the file is not included.
/// * When multiple fields are provided as filters, a logical AND (`&&`) strings
///   together the predicates. In other words, all filters must match for a
///   file to be returned. Note that this means that servers do not natively
///   support logical OR (`||`) across multiple fields: that must be done by
///   calling this endpoint with each of your desired queries and performing a
///   set union of those files out of band.
///
/// ### Ordering
///
/// This endpoint has default ordering requirements—those details are documented
/// in the `responses::Files` schema.
#[utoipa::path(
    get,
    path = "/file",
    tag = "File",
    params(
        FilterFileParams,
        (
            "metadata.unharmonized.<field>" = Option<String>,
            Query,
            nullable = false,
            description = "All unharmonized fields should be filterable in the \
            same manner as harmonized fields:\n\n\
            * Filtering on a singular field should include the `File` in \
            the results if the query exactly matches the value of that field \
            for the `File` (case-sensitive).\n\
            * Filtering on field with multiple values should include the \
            `File` in the results if the query exactly matches any of the \
            values of the field for that `File` (case-sensitive).\n\
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
        PaginationParams
    ),
    responses(
        (
            status = 200,
            description = "Successful operation.",
            body = responses::Files,
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
                        String::from("files"),
                        String::from(
                            "Our agreement with data providers prohibits us \
                            from sharing file-level data."
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
#[get("/file")]
pub async fn file_index(
    filter_params: Query<FilterFileParams>,
    pagination_params: Query<PaginationParams>,
    files: Data<Store>,
) -> impl Responder {
    let mut files = files.files.lock().unwrap().clone();

    // See the note in the documentation for this endpoint: the results must be
    // sorted by identifier by default.
    files.sort();

    let files = filter::<File, FilterFileParams>(files, filter_params.0);

    paginate::response::<File, Files>(pagination_params.0, files, "http://localhost:8000/file")
}

/// Gets the file matching the provided name (if the file exists).
#[utoipa::path(
    get,
    path = "/file/{organization}/{namespace}/{name}",
    params(
        (
            "organization" = String,
            description = "The organization identifier of the namespace to which the file belongs.",
        ),
        (
            "namespace" = String,
            description = "The name of the namespace to which the file belongs.",
        ),
        (
            "name" = String,
            description = "The name portion of the file identifier."
        )
    ),
    tag = "File",
    responses(
        (status = 200, description = "Successful operation.", body = responses::File),
        (
            status = 404,
            description = "Not found.\nServers that cannot provide line-level \
            data should use this response rather than Forbidden (403), as \
            there is no level of authorization that would allow one to access \
            the information included in the API.",
            body = responses::Errors,
            example = json!(Errors::from(error::Kind::not_found(
                String::from("File with namespace 'foo' and name 'bar'")
            )))
        )
    )
)]
#[get("/file/{organization}/{namespace}/{name}")]
pub async fn file_show(path: Path<(String, String, String)>, files: Data<Store>) -> impl Responder {
    let files = files.files.lock().unwrap();
    let (organization, namespace, name) = path.into_inner();

    files
        .iter()
        .find(|file| {
            file.id().namespace().organization().as_str() == organization
                && file.id().namespace().name().as_str() == namespace
                && **file.id().name() == name
        })
        .map(|file| HttpResponse::Ok().json(file))
        .unwrap_or_else(|| {
            HttpResponse::NotFound().json(Errors::from(error::Kind::not_found(format!(
                "File with namespace '{namespace}' and name '{name}'"
            ))))
        })
}

/// Groups the files by the specified metadata field and returns counts.
#[utoipa::path(
    get,
    path = "/file/by/{field}/count",
    params(
        ("field" = String, description = "The field to group by and count with."),
    ),
    tag = "File",
    responses(
        (status = 200, description = "Successful operation.", body = responses::by::count::file::Results),
        (
            status = 422,
            description = "Unsupported field.",
            body = responses::Errors,
            example = json!(Errors::from(
                error::Kind::unsupported_field(
                    String::from("handedness"),
                    String::from("This field is not present for files."),
                )
            ))
        ),
    )
)]
#[get("/file/by/{field}/count")]
pub async fn files_by_count(path: Path<String>, files: Data<Store>) -> impl Responder {
    let files = files.files.lock().unwrap().clone();
    let field = path.into_inner();

    let results = group_by(files, &field);

    match results {
        GroupByResults::Supported(results) => HttpResponse::Ok().json(results),
        GroupByResults::Unsupported => {
            HttpResponse::UnprocessableEntity().json(Errors::from(error::Kind::unsupported_field(
                field.to_string(),
                String::from("This field is not present for files."),
            )))
        }
    }
}

fn group_by(files: Vec<File>, field: &str) -> GroupByResults<responses::by::count::file::Results> {
    let values = files
        .iter()
        .map(|file| parse_field(field, file))
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

    GroupByResults::Supported(responses::by::count::file::Results::new(
        result,
        missing_values,
    ))
}

fn parse_field(field: &str, file: &File) -> Option<Option<Value>> {
    match field {
        "type" => match file.metadata() {
            Some(metadata) => Some(
                metadata
                    .r#type()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|r#type| serde_json::to_value(r#type.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "size" => match file.metadata() {
            Some(metadata) => Some(
                metadata
                    .size()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|size| serde_json::to_value(size.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "checksums" => match file.metadata() {
            Some(metadata) => Some(
                metadata
                    .checksums()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|checksum| serde_json::to_value(checksum.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "description" => match file.metadata() {
            Some(metadata) => Some(
                metadata
                    .description()
                    .as_ref()
                    // SAFETY: all metadata fields are able to be represented as
                    // [`serde_json::Value`]s.
                    .map(|description| serde_json::to_value(description.value()).unwrap())
                    .or(Some(Value::Null)),
            ),
            None => Some(None),
        },
        "depositions" => match file.metadata() {
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

/// Reports summary information for the files known by this server.
#[utoipa::path(
    get,
    path = "/file/summary",
    tag = "File",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Summary),
    )
)]
#[get("/file/summary")]
pub async fn file_summary(files: Data<Store>) -> impl Responder {
    let files = files.files.lock().unwrap().clone();
    HttpResponse::Ok().json(Summary::new(files.len()))
}

#[cfg(test)]
mod tests {
    use crate::routes::namespace::random_namespace;

    #[test]
    fn it_generates_a_random_namespace() {
        random_namespace();
    }
}
