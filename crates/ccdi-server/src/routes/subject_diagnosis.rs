//! Routes related to the experimental subject-diagnosis endpoint.

use actix_web::get;
use actix_web::web::Data;
use actix_web::web::Query;
use actix_web::web::ServiceConfig;
use actix_web::Responder;

use ccdi_models as models;

use models::Subject;

use crate::filter::filter;
use crate::paginate;
use crate::params::filter::SubjectDiagnosis as FilterSubjectDiagnosisParams;
use crate::params::PaginationParams;
use crate::responses::error;
use crate::responses::Errors;
use crate::responses::Subjects;

use crate::routes::subject::Store;

/// Configures the [`ServiceConfig`] with the subject paths.
pub fn configure(store: Data<Store>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.app_data(store).service(subject_diagnosis_index);
    }
}

/// Experimental: Filter the subjects known by this server by free-text diagnosis search.
///
/// ### Diagnosis Filtering
///
/// This endpoint supports the experimental `search` parameter.
/// For this parameter, the subject is included in the results if the value of its
/// its `associated_diagnoses` field has at least one diagnosis which _contains_ the query string.
/// Matches are case-insensitive.
///
/// ### Pagination
///
/// This endpoint is paginated. Users may override the default pagination
/// parameters by providing one or more of the pagination-related query
/// parameters below.
///
/// ### Additional Filtering
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
///
/// Note: This API is experimental and is subject to change without being considered
/// as a breaking change.
#[utoipa::path(
    get,
    path = "/subject-diagnosis",
    tag = "Experimental",
    params(
        FilterSubjectDiagnosisParams,
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
#[get("/subject-diagnosis")]
pub async fn subject_diagnosis_index(
    filter_params: Query<FilterSubjectDiagnosisParams>,
    pagination_params: Query<PaginationParams>,
    subjects: Data<Store>,
) -> impl Responder {
    let mut subjects = subjects.subjects.lock().unwrap().clone();

    // See the note in the documentation for this endpoint: the results must be
    // sorted by identifier by default.
    subjects.sort();

    let subjects = filter::<Subject, FilterSubjectDiagnosisParams>(subjects, filter_params.0);

    paginate::response::<Subject, Subjects>(
        pagination_params.0,
        subjects,
        "http://localhost:8000/subject",
    )
}
