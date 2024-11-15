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
        title = "CCDI Data Federation: Participating Nodes API",
        description = "This is the concrete OpenAPI specification for the CCDI
        Data Federation API—though this document contains all of the API calls
        that CCDI Federation nodes must implement, it does not outline the
        complete specification.

* Visit the [documentation homepage](https://cbiit.github.io/ccdi-federation-api)
  to view the complete set of requirements to maintain and deploy a CCDI
  Federation node.
* Additionally, you can view the Swagger specification in a more traditional
  theme by visiting
  [this link](https://editor.swagger.io/?url=https://cbiit.github.io/ccdi-federation-api/swagger.yml).",
        contact(
            name = "Childhood Cancer Data Initiative support email",
            email = "NCIChildhoodCancerDataInitiative@mail.nih.gov",
        ),
        version = "v1.0.0",
    ),
    external_docs(
        description = "Learn more about the Childhood Cancer Data Initiative",
        url = "https://www.cancer.gov/research/areas/childhood/childhood-cancer-data-initiative"
    ),
    servers(
        (
            url = "https://ccdi.stjude.cloud/api/v1",
            description = "St. Jude Children's Research Hospital CCDI API server"
        ),
        (
            url = "https://ccdifederation.pedscommons.org/api/v1",
            description = "Pediatric Cancer Data Commons CCDI API server"
        ),
        (
            url = "https://ccdi.treehouse.gi.ucsc.edu/api/v1",
            description = "UCSC Treehouse CCDI API server"
        ),
        (
            url = "https://ccdi.kidsfirstdrc.org/api/v1",
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
            name = "File",
            description = "Files within the CCDI federated ecosystem."
        ),
        (
            name = "Metadata",
            description = "List and describe provided metadata fields."
        ),
        (
            name = "Namespace",
            description = "List and describe namespaces known by this server."
        ),
        (
            name = "Organization",
            description = "List and describe organizations known by this server."
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

        // File routes.
        server::routes::file::file_index,
        server::routes::file::file_show,
        server::routes::file::files_by_count,
        server::routes::file::file_summary,

        // Metadata.
        server::routes::metadata::metadata_fields_subject,
        server::routes::metadata::metadata_fields_sample,
        server::routes::metadata::metadata_fields_file,

        // Namespaces.
        server::routes::namespace::namespace_index,
        server::routes::namespace::namespace_show,

        // Organizations.
        server::routes::organization::organization_index,
        server::routes::organization::organization_show,

        // Information.
        server::routes::info::info_index,
    ),
    components(schemas(
        // Harmonized common metadata elements.
        models::metadata::common::Metadata,
        cde::v1::deposition::DbgapPhsAccession,
        models::metadata::common::deposition::Accession,

        // Harmonized subject metadata elements.
        cde::v1::subject::Race,
        cde::v1::subject::Sex,
        cde::v2::subject::Ethnicity,
        cde::v1::subject::Name,
        cde::v1::subject::VitalStatus,
        models::subject::metadata::AgeAtVitalStatus,

        // Harmonized sample metadata elements.
        models::sample::metadata::AgeAtDiagnosis,
        models::sample::metadata::AnatomicalSite,
        models::sample::metadata::Diagnosis,
        cde::v1::sample::DiseasePhase,
        cde::v1::sample::LibraryStrategy,
        cde::v1::sample::LibrarySourceMaterial,
        cde::v2::sample::PreservationMethod,
        cde::v1::sample::SpecimenMolecularAnalyteType,
        cde::v2::sample::TissueType,
        cde::v1::sample::TumorClassification,
        cde::v1::sample::TumorTissueMorphology,
        models::sample::metadata::AgeAtCollection,

        // Harmonized file metadata elements.
        cde::v1::file::Name,
        cde::v1::file::Type,
        cde::v1::file::Size,
        models::file::metadata::Checksums,
        cde::v1::file::checksum::MD5,
        cde::v1::file::Description,

        // General harmonized field concepts.
        field::Details,
        field::details::Harmonizer,
        field::details::Method,

        // Harmonized namespace metadata elements.
        cde::v1::namespace::StudyFundingId,
        cde::v1::namespace::StudyId,
        cde::v1::namespace::StudyName,
        cde::v1::namespace::StudyShortTitle,

        // Harmonized organization metadata elements.
        cde::v1::organization::Institution,

        // Harmonized subject fields.
        field::unowned::subject::Sex,
        field::unowned::subject::Race,
        field::unowned::subject::Ethnicity,
        field::unowned::subject::Identifier,
        field::unowned::subject::VitalStatus,
        field::unowned::subject::AgeAtVitalStatus,

        // Harmonized sample fields.
        field::unowned::sample::AgeAtDiagnosis,
        field::unowned::sample::AnatomicalSite,
        field::unowned::sample::Diagnosis,
        field::unowned::sample::DiseasePhase,
        field::unowned::sample::LibraryStrategy,
        field::unowned::sample::LibrarySourceMaterial,
        field::unowned::sample::PreservationMethod,
        field::unowned::sample::SpecimenMolecularAnalyteType,
        field::unowned::sample::TissueType,
        field::unowned::sample::TumorClassification,
        field::unowned::sample::TumorTissueMorphology,
        field::unowned::sample::AgeAtCollection,
        field::unowned::sample::Identifier,

        // Harmonized file fields.
        field::unowned::file::Type,
        field::unowned::file::Size,
        field::unowned::file::Checksums,
        field::unowned::file::Description,

        // Harmonized namespace fields.
        field::unowned::namespace::StudyFundingId,
        field::unowned::namespace::StudyId,
        field::unowned::namespace::StudyName,
        field::unowned::namespace::StudyShortTitle,

        // Harmonized organization fields.
        field::unowned::organization::Institution,

        // Unharmonized fields.
        field::owned::Field,
        field::unowned::Field,
        field::UnharmonizedField,
        fields::Unharmonized,

        // Subject models.
        models::Subject,
        models::subject::identifier::referenced::Identifier,
        models::subject::identifier::linked::Identifier,
        models::subject::identifier::unlinked::Identifier,
        models::subject::Identifier,
        models::subject::Kind,
        models::subject::Metadata,

        // Sample models.
        models::Sample,
        models::sample::identifier::referenced::Identifier,
        models::sample::identifier::linked::Identifier,
        models::sample::identifier::unlinked::Identifier,
        models::sample::Identifier,
        models::sample::Metadata,

        // File models.
        models::File,
        models::file::Identifier,
        models::file::Metadata,

        // Gateway models.
        models::gateway::Link,
        models::gateway::AnonymousOrReference,
        models::gateway::closed::Status,
        models::gateway::Closed,
        models::gateway::Named,
        models::Gateway,

        // Metadata models.
        models::metadata::field::Description,
        models::metadata::field::description::Harmonized,
        models::metadata::field::description::Unharmonized,
        models::metadata::field::description::harmonized::Standard,

        // Namespace models.
        models::Namespace,
        models::namespace::identifier::Name,
        models::namespace::Identifier,
        models::namespace::Description,
        models::namespace::Metadata,

        // Organization models.
        models::Organization,
        models::organization::Identifier,
        models::organization::Name,
        models::organization::Metadata,

        // Url model.
        models::Url,

        // General responses.
        responses::Errors,

        // Summary responses.
        responses::summary::Counts,
        responses::Summary,

        // Cross-entity responses.
        responses::entity::Summary,
        responses::entity::Counts,

        // Count by response components.
        responses::by::count::ValueCount,

        // Subject responses.
        responses::Subject,
        responses::Subjects,
        responses::by::count::subject::Results,

        // Sample responses.
        responses::Sample,
        responses::Samples,
        responses::by::count::sample::Results,

        // File responses.
        responses::File,
        responses::Files,
        responses::by::count::file::Results,

        // Metadata responses.
        responses::metadata::FieldDescriptions,

        // Namespace responses.
        responses::Namespace,
        responses::Namespaces,

        // Organization responses.
        responses::Organization,
        responses::Organizations,

        // Information responses.
        responses::Information,
        responses::info::api::Information,
        responses::info::data::Information,
        responses::info::data::Version,
        responses::info::data::version::About,
        responses::info::server::Information,

        // Error responses.
        responses::error::Kind,
        responses::Errors
    )),
    modifiers(
        &RemoveLicense,
    )
)]
pub struct Api;

pub struct RemoveLicense;

impl Modify for RemoveLicense {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        openapi.info.license = None;
    }
}
