//! Common pagination utilities.

use std::num::NonZeroUsize;

use actix_web::HttpResponse;
use serde::Serialize;

use crate::params::pagination;
use crate::params::Pagination as PaginationParams;
use crate::responses::error;
use crate::responses::Errors;

pub mod links;

pub use links::Links;
pub use links::Relationship;

pub(crate) fn response<T, R>(
    params: PaginationParams,
    all_entities: Vec<T>,
    base_url: &str,
) -> HttpResponse
where
    T: Clone,
    R: Serialize,
    R: From<(Vec<T>, usize)>,
{
    let page = match NonZeroUsize::try_from(params.page().unwrap_or(pagination::DEFAULT_PAGE)) {
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

    let per_page =
        match NonZeroUsize::try_from(params.per_page().unwrap_or(pagination::DEFAULT_PER_PAGE)) {
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

    if all_entities.is_empty() {
        // If this error occurs, there is likely some misconfiguration that
        // allows zero entities to be generated for the server. This should be
        // caught before we get to this point and reported to the user.
        panic!("there must be at least one entity to paginate.");
    }

    let pages = all_entities.chunks(per_page.get()).collect::<Vec<_>>();

    let links = links::Builder::try_new(base_url, page, per_page, pages.clone())
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

    let this_page_entities = pages.into_iter().nth(page.get() - 1).unwrap_or_default();

    if this_page_entities.is_empty() {
        return HttpResponse::UnprocessableEntity().json(Errors::from(
            error::Kind::invalid_parameters(
                Some(vec![String::from("page"), String::from("per_page")]),
                format!("no {}s selected", stringify!(T).to_lowercase()),
            ),
        ));
    }

    HttpResponse::Ok()
        .insert_header(("link", links.to_string()))
        .json(R::from((this_page_entities.to_vec(), all_entities.len())))
}
