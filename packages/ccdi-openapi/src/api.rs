use models::metadata::field;
use utoipa::Modify;
use utoipa::OpenApi;

use ccdi_cde as cde;
use ccdi_models as models;
use ccdi_server as server;

use server::responses;
use utoipa::openapi;

/// The OpenAPI specification.
#[derive(Debug, OpenApi)]
#[openapi(
    info(
        title = "CCDI Pediatric Cancer Data Catalog",
        description = "The CCDI Pediatric Cancer Data Catalog is an API that supports the querying
of federated pediatric cancer within the broader community. The goal of the
API is to support identification of pediatric cancer samples of interest via
a variety of query parameters.",
        contact(
            name = "Childhood Cancer Data Initiative support email",
            email = "NCIChildhoodCancerDataInitiative@mail.nih.gov",
            url = "https://www.cancer.gov/research/areas/childhood/childhood-cancer-data-initiative",
        ),
        version = "0.1",
    ),
    external_docs(
        description = "Learn more about the Childhood Cancer Data Initiative",
        url = "https://www.cancer.gov/research/areas/childhood/childhood-cancer-data-initiative"
    ),
    servers(
        (
            url = "https://ccdi.stjude.cloud/api/v0",
            description = "St. Jude Children's Research Hospital CCDI API server"
        ),
        (
            url = "https://ccdifederation.pedscommons.org/api/v0",
            description = "UCSC Treehouse CCDI API server"
        ),
        (
            url = "https://ccdi.treehouse.gi.ucsc.edu/api/v0",
            description = "Pediatric Cancer Data Commons CCDI API server"
        ),
    ),
    tags(
        (
            name = "Info",
            description = "Information about the API implementation itself."
        ),
        (
            name = "Subject",
            description = "Subjects within the CCDI federated ecosystem."
        )
    ),
    paths(
        // Metadata.
        server::routes::metadata::metadata_fields_subject,

        // Subject.
        server::routes::subject::index,
        server::routes::subject::show,
        server::routes::subject::subjects_by_count,
    ),
    components(schemas(
        // Common data elements.
        cde::v1::Race,
        cde::v1::Sex,
        cde::v2::Ethnicity,
        cde::v1::Identifier,

        // Fields.
        field::Sex,
        field::Race,
        field::Ethnicity,
        field::Identifier,

        // Fields or null.
        field::SexOrNull,
        field::RacesOrNull,
        field::EthnicityOrNull,
        field::IdentifiersOrNull,

        // Models.
        models::Subject,
        models::subject::Kind,
        models::subject::Metadata,

        models::metadata::field::Description,
        models::metadata::field::description::Harmonized,
        models::metadata::field::description::Unharmonized,

        // Counts.
        models::count::Total,

        // Responses.
        responses::Error,
        responses::Subject,
        responses::Subjects,
        responses::metadata::FieldDescriptions,
    )),
    modifiers(&RemoveLicense)
)]
pub struct Api;

pub struct RemoveLicense;

impl Modify for RemoveLicense {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        openapi.info.license = None;
    }
}
