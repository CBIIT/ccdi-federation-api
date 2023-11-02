use models::metadata::field;
use models::metadata::fields;
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
        ),
        version = "0.3",
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
            description = "Pediatric Cancer Data Commons CCDI API server"
        ),
        (
            url = "https://ccdi.treehouse.gi.ucsc.edu/api/v0",
            description = "UCSC Treehouse CCDI API server"
        ),
        (
            url = "https://ccdi.kidsfirstdrc.org/api/v0",
            description = "KidsFirst CCDI API server"
        ),
    ),
    tags(
        (
            name = "Subject",
            description = "Subjects within the CCDI federated ecosystem."
        ),
        (
            name = "Sample",
            description = "Samples within the CCDI federated ecosystem."
        ),
        (
            name = "Metadata",
            description = "List and describe provided metadata fields."
        ),
        (
            name = "Info",
            description = "Information about the API implementation itself."
        ),
    ),
    paths(
        // Subject routes.
        server::routes::subject::subject_index,
        server::routes::subject::subject_show,
        server::routes::subject::subjects_by_count,
        server::routes::subject::subject_summary,

        // Sample routes.
        server::routes::sample::sample_index,
        server::routes::sample::sample_show,
        server::routes::sample::samples_by_count,
        server::routes::sample::sample_summary,

        // Metadata.
        server::routes::metadata::metadata_fields_subject,
        server::routes::metadata::metadata_fields_sample,
    ),
    components(schemas(
        // Subject common data elements (CDEs).
        cde::v1::subject::Race,
        cde::v1::subject::Sex,
        cde::v2::subject::Ethnicity,
        cde::v1::subject::Identifier,

        // Sample common data elements (CDEs).
        cde::v1::sample::DiseasePhase,
        cde::v2::sample::TissueType,
        cde::v1::sample::TumorClassification,

        // Harmonized subject fields.
        field::unowned::subject::Sex,
        field::unowned::subject::Race,
        field::unowned::subject::Ethnicity,
        field::owned::subject::Identifier,

        // Harmonized sample fields.
        field::unowned::sample::DiseasePhase,
        field::unowned::sample::TissueType,
        field::unowned::sample::TumorClassification,

        // Unharmonized fields.
        field::owned::Field,
        field::unowned::Field,
        field::UnharmonizedField,
        fields::Unharmonized,

        // Subject models.
        models::Subject,
        models::subject::Kind,
        models::subject::Metadata,

        // Sample models.
        models::Sample,
        models::sample::Identifier,
        models::sample::Metadata,

        // Metadata models.
        models::metadata::field::Description,
        models::metadata::field::description::Harmonized,
        models::metadata::field::description::Unharmonized,

        // General responses.
        responses::Errors,

        // Summary responses.
        responses::summary::Counts,
        responses::Summary,

        // Subject responses.
        responses::Subject,
        responses::Subjects,
        responses::by::count::Subjects,

        // Sample responses.
        responses::Sample,
        responses::Samples,
        responses::by::count::Samples,

        // Metadata responses.
        responses::metadata::FieldDescriptions,

        // Error responses.
        responses::error::Kind,
        responses::Errors
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
