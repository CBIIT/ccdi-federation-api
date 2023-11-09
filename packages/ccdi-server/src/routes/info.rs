//! Routes related to server information.

use actix_web::get;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::responses::Information;

/// Configures the [`ServiceConfig`] with the info paths.
pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(info_index);
    }
}

/// Gets the info for this server.
#[utoipa::path(
    get,
    path = "/info",
    tag = "Info",
    responses(
        (
            status = 200,
            description = "Successful operation.",
            body = responses::Information,
        ),
    )
)]
#[get("/info")]
pub async fn info_index() -> impl Responder {
    HttpResponse::Ok().json(Information::default())
}
