//! Routes related to organizations.

use actix_web::get;
use actix_web::web::Path;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use ccdi_models as models;

use models::organization;
use rand::distributions::Distribution as _;
use rand::distributions::Uniform;
use rand::thread_rng;

use crate::responses::error;
use crate::responses::Errors;
use crate::responses::Organization;
use crate::responses::Organizations;

lazy_static! {
    /// Organizations supported by this server.
    pub static ref ORGANIZATIONS: IndexMap<&'static str, models::Organization> = {
        let mut hm = IndexMap::new();

        hm.insert(
            "example-organization",
            models::Organization::new(
         "example-organization".parse::<organization::Identifier>().unwrap(),
         "Example Organization".parse::<organization::Name>().unwrap(),
            )
        );

        hm
    };
}

/// Picks a random organization from the provided [`Organizations`](ccdi_models::Organization);
///
/// # Examples
///
/// ```
/// use ccdi_server as server;
///
/// use server::routes::organization::random_organization;
///
/// let ns = random_organization();
/// ```
pub fn random_organization() -> &'static ccdi_models::Organization {
    let mut rng = thread_rng();
    let index_dist = Uniform::from(0..ORGANIZATIONS.len());
    let index = index_dist.sample(&mut rng);

    // SAFETY: this is manually crafted to always return an element.
    let (_, organization) = ORGANIZATIONS.get_index(index).unwrap();
    organization
}

/// Configures the [`ServiceConfig`] with the organization paths.
pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(organization_index)
            .service(organization_show);
    }
}

/// Gets the organizations known by this server.
#[utoipa::path(
    get,
    path = "/organization",
    tag = "Organization",
    responses(
        (
            status = 200,
            description = "Successful operation.",
            body = responses::Organizations,
        ),
    )
)]
#[get("/organization")]
pub async fn organization_index() -> impl Responder {
    HttpResponse::Ok().json(Organizations::from(
        ORGANIZATIONS.clone().into_values().collect::<Vec<_>>(),
    ))
}

/// Gets the organization matching the provided name (if it exists).
#[utoipa::path(
    get,
    path = "/organization/{name}",
    params(
        (
            "name" = String,
            description = "The name of the organization.",
        ),
    ),
    tag = "Organization",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Organization),
        (
            status = 404,
            description = "Not found.",
            body = responses::Errors,
            example = json!(Errors::from(error::Kind::not_found(String::from("Organizations"))))
        )
    )
)]
#[get("/organization/{name}")]
pub async fn organization_show(path: Path<String>) -> impl Responder {
    let organization_name = path.into_inner();

    ORGANIZATIONS
        .iter()
        .find(|(_, organization)| organization.id().as_str() == organization_name)
        .map(|(_, organization)| HttpResponse::Ok().json(Organization::from(organization.clone())))
        .unwrap_or_else(|| {
            HttpResponse::NotFound().json(Errors::from(error::Kind::not_found(format!(
                "Organization with name '{}'",
                organization_name
            ))))
        })
}
