//! Routes related to namespaces.

use actix_web::get;
use actix_web::web::Path;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use ccdi_cde as cde;
use ccdi_models as models;

use models::metadata::field;
use models::namespace;
use rand::distributions::Distribution as _;
use rand::distributions::Uniform;
use rand::thread_rng;

use crate::responses::error;
use crate::responses::Errors;
use crate::responses::Namespace;
use crate::responses::Namespaces;
use crate::routes::organization::ORGANIZATIONS;

lazy_static! {
    /// Namespaces supported by this server.
    pub static ref NAMESPACES: IndexMap<&'static str, models::Namespace> = {
        let mut hm = IndexMap::new();

        hm.insert(
            "example-organization-namespace-one",
            // SAFETY: this is manually crafted to unwrap every time, as the
            // organization name conforms to the correct pattern.
            models::Namespace::new(
                namespace::Identifier::new(
                    ORGANIZATIONS.get("example-organization").unwrap().id().clone(),
                    namespace::identifier::Name::try_new("ExampleNamespaceOne").unwrap(),
                ),
                "support@example.com",
                Some(
                "The first example namespace owned by Example Organization."
                        .parse::<namespace::Description>()
                        .unwrap(),
                ),
                Some(namespace::metadata::Builder::default()
                    .study_short_title(
                        field::unowned::namespace::StudyShortTitle::new(
                            cde::v1::namespace::StudyShortTitle::from(
                                String::from("A study short title")
                            ),
                            None, None, None)
                    ).build())
            )
        );

        hm.insert(
            "example-organization-namespace-two",
            // SAFETY: this is manually crafted to unwrap every time, as the
            // organization name conforms to the correct pattern.
            models::Namespace::new(
                namespace::Identifier::new(
                    ORGANIZATIONS.get("example-organization").unwrap().id().clone(),
                    namespace::identifier::Name::try_new("ExampleNamespaceTwo").unwrap(),
                ),
                "support@example.com",
                Some(
                "The second example namespace owned by Example Organization."
                        .parse::<namespace::Description>()
                        .unwrap(),
                ),
                Some(namespace::metadata::Builder::default()
                    .study_short_title(
                        field::unowned::namespace::StudyShortTitle::new(
                            cde::v1::namespace::StudyShortTitle::from(
                                String::from("A study short title")
                            ),
                            None, None, None)
                    ).build())
            )
        );

        hm
    };
}

/// Picks a random namespace from the provided [`Namespaces`](ccdi_models::Namespace);
///
/// # Examples
///
/// ```
/// use ccdi_server as server;
///
/// use server::routes::namespace::random_namespace;
///
/// let ns = random_namespace();
/// ```
pub fn random_namespace() -> &'static ccdi_models::Namespace {
    let mut rng = thread_rng();
    let index_dist = Uniform::from(0..NAMESPACES.len());
    let index = index_dist.sample(&mut rng);

    // SAFETY: this is manually crafted to always return an element.
    let (_, namespace) = NAMESPACES.get_index(index).unwrap();
    namespace
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
    path = "/namespace/{organization}/{namespace}",
    params(
        (
            "organization" = String,
            description = "The organization of the namespace.",
        ),
        (
            "namespace" = String,
            description = "The name of the namespace.",
        ),
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
#[get("/namespace/{organization}/{namespace}")]
pub async fn namespace_show(path: Path<(String, String)>) -> impl Responder {
    let (organization, namespace_name) = path.into_inner();

    NAMESPACES
        .iter()
        .find(|(_, namespace)| {
            namespace.id().organization().as_str() == organization
                && namespace.id().name().as_str() == namespace_name
        })
        .map(|(_, namespace)| HttpResponse::Ok().json(Namespace::from(namespace.clone())))
        .unwrap_or_else(|| {
            HttpResponse::NotFound().json(Errors::from(error::Kind::not_found(format!(
                "Namespace with organization '{}' and name '{}'",
                organization, namespace_name
            ))))
        })
}
