//! Routes related to files.

use std::num::NonZeroUsize;
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
use indexmap::IndexMap;
use itertools::Itertools as _;
use models::namespace;
use rand::prelude::*;

use ccdi_models as models;

use models::file::Identifier;
use models::gateway::Link;
use models::gateway::Named;
use models::File;
use models::Gateway;
use models::Url;
use serde_json::Value;

use crate::paginate::links;
use crate::paginate::links::Relationship;
use crate::params::pagination;
use crate::params::PaginationParams;
use crate::params::PartitionParams;
use crate::params::Partitionable;
use crate::responses;
use crate::responses::by;
use crate::responses::by::count::file::NamespacePartitionedResult;
use crate::responses::error;
use crate::responses::file::data;
use crate::responses::Errors;
use crate::responses::Files;
use crate::responses::Summary;
use crate::routes::MISSING_GROUP_BY_KEY;
use crate::routes::NULL_GROUP_BY_KEY;

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
            .service(file_show)
            .service(files_by_count)
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
/// ### Ordering
///
/// This endpoint has default ordering requirements—those details are documented
/// in the `responses::Files` schema.
#[utoipa::path(
    get,
    path = "/file",
    tag = "File",
    params(PaginationParams),
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
    pagination_params: Query<PaginationParams>,
    files: Data<Store>,
) -> impl Responder {
    let mut all_files = files.files.lock().unwrap().clone();

    // See the note in the documentation for this endpoint: the results must be
    // sorted by identifier by default.
    all_files.sort();

    let page = match NonZeroUsize::try_from(
        pagination_params
            .0
            .page()
            .unwrap_or(pagination::DEFAULT_PAGE),
    ) {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::UnprocessableEntity().json(Errors::from(
                error::Kind::invalid_parameters(
                    Some(vec![String::from("page")]),
                    String::from("must be a non-zero usize"),
                ),
            ))
        }
    };

    let per_page = match NonZeroUsize::try_from(
        pagination_params
            .0
            .per_page()
            .unwrap_or(pagination::DEFAULT_PER_PAGE),
    ) {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::UnprocessableEntity().json(Errors::from(
                error::Kind::invalid_parameters(
                    Some(vec![String::from("per_page")]),
                    String::from("must be a non-zero usize"),
                ),
            ))
        }
    };

    if all_files.is_empty() {
        // If this error occurs, there is likely some misconfiguration that
        // allows zero entities to be generated for the server. This should be
        // caught before we get to this point and reported to the user.
        panic!("there must be at least one entity to paginate.");
    }

    let pages = all_files.chunks(per_page.get()).collect::<Vec<_>>();

    let links =
        links::Builder::try_new("http://localhost:8000/file", page, per_page, pages.clone())
            .unwrap_or_else(|err| {
                match err {
                    links::builder::Error::ParseError(err) => {
                        match err {
                            links::builder::ParseError::UrlParseError(_) => {
                                // If this error occurs, there is something wrong
                                // with the code that generates the base URL for the
                                // links. This cannot be a user issue.
                                panic!("provided URL is not parsable")
                            }
                        }
                    }
                }
            })
            .insert_link(Relationship::First)
            .insert_link(Relationship::Prev)
            .insert_link(Relationship::Next)
            .insert_link(Relationship::Last)
            .build();

    let this_page_files = pages
        .into_iter()
        .nth(page.get() - 1)
        .unwrap_or_default()
        .iter()
        .cloned()
        .map(crate::responses::File::from)
        .collect::<Vec<_>>();

    if this_page_files.is_empty() {
        return HttpResponse::UnprocessableEntity().json(Errors::from(
            error::Kind::invalid_parameters(
                Some(vec![String::from("page"), String::from("per_page")]),
                format!("no {}s selected", stringify!(T).to_lowercase()),
            ),
        ));
    }

    let files = data::Files::from((this_page_files, all_files.len()));
    let gateways = files
        .expected_gateways()
        .into_iter()
        .map(|name| {
            Named::new(
                name,
                Gateway::Open {
                    link: Link::Direct {
                        // SAFETY: this is manually crafted to unwrap
                        // successfully every time.
                        url: "https://example.com".parse::<Url>().unwrap(),
                    },
                },
            )
        })
        .collect::<Vec<_>>();

    let mut response = &mut HttpResponse::Ok();
    response = response.insert_header(("link", links.to_string()));

    // SAFETY: this will only error when the named gateways and references to
    // gateways from files do not perfectly match (i.e., there is a reference to
    // a named gateway that is not included in the response _or_ there is a
    // named gatway in the response that is not referred to by any file). Above,
    // we craft the response to _exactly_ match these gateways. Thus, this will
    // always unwrap.
    let result = Files::try_new(files, gateways).unwrap();
    response.json(result)
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
                "File with namespace '{}' and name '{}'",
                namespace, name
            ))))
        })
}

/// Groups the files by the specified metadata field and returns counts.
#[utoipa::path(
    get,
    path = "/file/by/{field}/count",
    params(
        ("field" = String, description = "The field to group by and count with."),
        PartitionParams,
    ),
    tag = "File",
    responses(
        (status = 200, description = "Successful operation.", body = responses::by::count::file::Response),
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
pub async fn files_by_count(
    path: Path<String>,
    partition_params: Query<PartitionParams>,
    files: Data<Store>,
) -> impl Responder {
    let files = files.files.lock().unwrap().clone();
    let field = path.into_inner();

    if let Some(Partitionable::Namespace) = partition_params.0.partition {
        let namespaces = files
            .iter()
            .map(|s| s.id().namespace())
            .unique()
            .cloned()
            .sorted()
            .collect::<Vec<namespace::Identifier>>();

        let mut results = Vec::new();

        for namespace in namespaces {
            let namespace_files = files
                .iter()
                .filter(|s| s.id().namespace() == &namespace)
                .cloned()
                .collect::<Vec<File>>();

            let namespace_results = group_by(namespace_files, &field);

            if namespace_results.values.len() == 1
                && namespace_results.values.get(MISSING_GROUP_BY_KEY).is_some()
            {
                return HttpResponse::UnprocessableEntity().json(Errors::from(
                    error::Kind::unsupported_field(
                        field.to_string(),
                        String::from("This field is not present for files."),
                    ),
                ));
            }

            let namespace_partitioned_result =
                NamespacePartitionedResult::new(namespace.clone(), namespace_results);

            results.push(namespace_partitioned_result);
        }

        HttpResponse::Ok().json(by::count::file::Response::NamespacePartitioned(
            by::count::file::NamespacePartitionedResults::from(results),
        ))
    } else {
        let results = group_by(files, &field);

        if results.values.len() == 1 && results.values.get(MISSING_GROUP_BY_KEY).is_some() {
            return HttpResponse::UnprocessableEntity().json(Errors::from(
                error::Kind::unsupported_field(
                    field.to_string(),
                    String::from("This field is not present for files."),
                ),
            ));
        }

        HttpResponse::Ok().json(by::count::file::Response::Unpartitioned(results))
    }
}

fn group_by(files: Vec<File>, field: &str) -> responses::by::count::file::Results {
    let values = files
        .iter()
        .map(|file| parse_field(field, file))
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

    responses::by::count::file::Results::from(result)
}

fn parse_field(field: &str, file: &File) -> Option<Value> {
    match field {
        "type" => match file.metadata() {
            Some(metadata) => metadata
                .r#type()
                .as_ref()
                .map(|r#type| Value::String(r#type.value().to_string())),
            None => None,
        },
        "size" => match file.metadata() {
            Some(metadata) => metadata
                .size()
                .as_ref()
                .map(|size| Value::String(size.value().to_string())),
            None => None,
        },
        "checksums" => match file.metadata() {
            Some(metadata) => metadata
                .checksums()
                .as_ref()
                .map(|value| Value::String(value.value().to_string())),
            None => None,
        },
        "description" => match file.metadata() {
            Some(metadata) => metadata
                .description()
                .as_ref()
                .map(|value| Value::String(value.value().to_string())),
            None => None,
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
