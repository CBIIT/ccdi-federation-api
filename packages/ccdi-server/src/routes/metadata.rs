//! Routes related to metadata.

use actix_web::get;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;

use ccdi_cde as cde;
use ccdi_models as models;

use models::metadata::field::description::r#trait::Description as _;

use crate::responses::metadata::FieldDescriptions;

/// Configures the [`ServiceConfig`] with the metadata paths.
pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(metadata_fields_subject);
    }
}

/// Gets the metadata fields for subjects that are supported by this server.
#[utoipa::path(
    get,
    path = "/metadata/fields/subject",
    tag = "Metadata",
    responses(
        (status = 200, description = "Successful operation.", body = [responses::Subjects])
    )
)]
#[get("/metadata/fields/subject")]
pub async fn metadata_fields_subject() -> impl Responder {
    HttpResponse::Ok().json(FieldDescriptions::from(vec![
        cde::v1::Sex::description(),
        cde::v1::Race::description(),
        cde::v2::Ethnicity::description(),
        cde::v1::Identifier::description(),
    ]))
}
