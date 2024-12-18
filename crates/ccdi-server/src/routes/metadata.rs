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
        config.service(metadata_fields_namespace);
        config.service(metadata_fields_organization);
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

/// Gets the metadata fields for namespaces that are supported by this server.
#[utoipa::path(
    get,
    path = "/metadata/fields/namespace",
    tag = "Metadata",
    responses(
        (status = 200, description = "Successful operation.", body = responses::metadata::FieldDescriptions)
    )
)]
#[get("/metadata/fields/namespace")]
pub async fn metadata_fields_namespace() -> impl Responder {
    HttpResponse::Ok().json(FieldDescriptions::from(
        models::metadata::field::description::harmonized::namespace::get_field_descriptions(),
    ))
}

/// Gets the metadata fields for organizations that are supported by this server.
#[utoipa::path(
    get,
    path = "/metadata/fields/organization",
    tag = "Metadata",
    responses(
        (status = 200, description = "Successful operation.", body = responses::metadata::FieldDescriptions)
    )
)]
#[get("/metadata/fields/organization")]
pub async fn metadata_fields_organization() -> impl Responder {
    HttpResponse::Ok().json(FieldDescriptions::from(
        models::metadata::field::description::harmonized::organization::get_field_descriptions(),
    ))
}
