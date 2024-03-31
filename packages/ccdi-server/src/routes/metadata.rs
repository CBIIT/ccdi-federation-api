//! Routes related to metadata.

use actix_web::get;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;

use ccdi_models as models;

use crate::responses::metadata::FieldDescriptions;

/// Configures the [`ServiceConfig`] with the metadata paths.
pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(metadata_fields_subject);
        config.service(metadata_fields_sample);
        config.service(metadata_fields_file);
    }
}

/// Gets the metadata fields for subjects that are supported by this server.
#[utoipa::path(
    get,
    path = "/metadata/fields/subject",
    tag = "Metadata",
    responses(
        (status = 200, description = "Successful operation.", body = responses::metadata::FieldDescriptions)
    )
)]
#[get("/metadata/fields/subject")]
pub async fn metadata_fields_subject() -> impl Responder {
    HttpResponse::Ok().json(FieldDescriptions::from(
        models::metadata::field::description::harmonized::subject::get_field_descriptions(),
    ))
}

/// Gets the metadata fields for samples that are supported by this server.
#[utoipa::path(
    get,
    path = "/metadata/fields/sample",
    tag = "Metadata",
    responses(
        (status = 200, description = "Successful operation.", body = responses::metadata::FieldDescriptions)
    )
)]
#[get("/metadata/fields/sample")]
pub async fn metadata_fields_sample() -> impl Responder {
    HttpResponse::Ok().json(FieldDescriptions::from(
        models::metadata::field::description::harmonized::sample::get_field_descriptions(),
    ))
}

/// Gets the metadata fields for files that are supported by this server.
#[utoipa::path(
    get,
    path = "/metadata/fields/file",
    tag = "Metadata",
    responses(
        (status = 200, description = "Successful operation.", body = responses::metadata::FieldDescriptions)
    )
)]
#[get("/metadata/fields/file")]
pub async fn metadata_fields_file() -> impl Responder {
    HttpResponse::Ok().json(FieldDescriptions::from(
        models::metadata::field::description::harmonized::file::get_field_descriptions(),
    ))
}
