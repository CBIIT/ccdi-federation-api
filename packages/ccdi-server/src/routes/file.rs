//! Routes related to files.

use std::num::NonZeroUsize;
use std::sync::Mutex;

use actix_web::get;
use actix_web::web::Data;
use actix_web::web::Query;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use rand::Rng as _;

use ccdi_models as models;

use models::file::Identifier;
use models::gateway::Link;
use models::gateway::Named;
use models::File;
use models::Gateway;
use models::Url;

use crate::paginate::links;
use crate::paginate::links::Relationship;
use crate::params::pagination;
use crate::params::Pagination as PaginationParams;
use crate::responses::error;
use crate::responses::file::data;
use crate::responses::Errors;
use crate::responses::Files;
use crate::responses::Summary;
use crate::routes::namespace::NAMESPACES;

/// A store for [`File`]s.
#[derive(Debug)]
pub struct Store {
    files: Mutex<Vec<File>>,
}

impl Store {
    /// Creates a new [`Store`] with randomized [`File`]s.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::routes::file::Store;
    ///
    /// let store = Store::random(100, 1000);
    /// ```
    pub fn random(count: usize, number_of_samples: usize) -> Self {
        Self {
            files: Mutex::new(
                (0..count)
                    .map(|i| {
                        let identifier = Identifier::new(
                            // SAFETY: this is hardcoded to work and is tested
                            // statically below.
                            NAMESPACES.get("organization").unwrap(),
                            format!("File{}.txt", i + 1),
                        );

                        let sample = rand::thread_rng().gen_range(0..number_of_samples) + 1;
                        let sample = models::sample::Identifier::new(
                            // SAFETY: this is hardcoded to work and is tested
                            // statically below.
                            NAMESPACES.get("organization").unwrap(),
                            format!("Sample{}", sample),
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
        .to_vec();

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
                        url: Url::try_from("https://example.com").unwrap(),
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
    use super::*;

    #[test]
    fn it_unwraps_the_default_namespace() {
        NAMESPACES.get("organization").unwrap();
    }
}
