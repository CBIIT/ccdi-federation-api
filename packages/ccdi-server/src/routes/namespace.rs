//! Routes related to namespaces.

use actix_web::get;
use actix_web::web::Path;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use ccdi_models as models;

use models::namespace::Description;

use crate::responses::error;
use crate::responses::Errors;
use crate::responses::Namespace;
use crate::responses::Namespaces;

lazy_static! {
    /// Namespaces supported by this server.
    pub static ref NAMESPACES: IndexMap<&'static str, models::Namespace> = {
        let mut hm = IndexMap::new();
        hm.insert(
            "organization",
            // SAFETY: this is manually crafted to unwrap every time, as the
            // organization name conforms to the correct pattern.
            models::Namespace::try_new(
                "organization",
                "Example Organization",
                "support@example.com",
                Some(
                    "An example organization owned by Example Organization."
                        .parse::<Description>()
                        .unwrap(),
                ),
            ).unwrap(),
        );
        hm
    };
}

/// Configures the [`ServiceConfig`] with the namespace paths.
pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(namespace_index).service(namespace_show);
    }
}

/// Gets the namespaces known by this server.
#[utoipa::path(
    get,
    path = "/namespace",
    tag = "Namespace",
    responses(
        (
            status = 200,
            description = "Successful operation.",
            body = responses::Namespaces,
        ),
    )
)]
#[get("/namespace")]
pub async fn namespace_index() -> impl Responder {
    HttpResponse::Ok().json(Namespaces::from(
        NAMESPACES.clone().into_values().collect::<Vec<_>>(),
    ))
}

/// Gets the namespace matching the provided name (if it exists).
#[utoipa::path(
    get,
    path = "/namespace/{name}",
    params(
        (
            "name" = String,
            description = "The name of the namespace."
        )
    ),
    tag = "Namespace",
    responses(
        (status = 200, description = "Successful operation.", body = responses::Namespace),
        (
            status = 404,
            description = "Not found.",
            body = responses::Errors,
            example = json!(Errors::from(error::Kind::not_found(String::from("Namespaces"))))
        )
    )
)]
#[get("/namespace/{name}")]
pub async fn namespace_show(path: Path<String>) -> impl Responder {
    let name = path.into_inner();

    NAMESPACES
        .iter()
        .find(|(_, namespace)| namespace.name() == name)
        .map(|(_, namespace)| HttpResponse::Ok().json(Namespace::from(namespace.clone())))
        .unwrap_or_else(|| {
            HttpResponse::NotFound().json(Errors::from(error::Kind::not_found(format!(
                "Namespace with name '{}'",
                name
            ))))
        })
}
