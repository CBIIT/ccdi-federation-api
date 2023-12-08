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
        description = "The CCDI Pediatric Cancer Data Catalog is an API that
supports the querying of federated pediatric cancer within the broader
community. The goal of the API is to support identification of pediatric cancer
samples of interest via a variety of query parameters.

## Definitions

**Authentication** is defined as being identified in any manner.
Authentication, by definition, requires prior registration of identifiable
characteristics. Typically, this is in the form of an account, though the
definition of authentication includes any condition under which you are
registered and identified (e.g., allowlisted via IP address).

**Authorization** is defined as permission explicitly granted or withheld from
an authenticated individual by a controlling entity based on a set of
non-authentication-based criteria (irrespective of the time period for which
access is granted or denied). Note that this definition of authorization always
requires prior authentication, so simply requiring authentication to gain
access to a resource is not considered authorization. For example, gaining
permission to a dataset via an explicit decision from a data access committee
is considered authorization while making data available after simply completing
a universally accessible account registration process is not.

## Invalid Routes

All responses that do not match an endpoint below should return a Not Found 
(`404`) response. The body of this response should be the `responses.Errors` 
JSON object with one `responses.error.Kind` where the `Kind` matches the 
`InvalidRoute` error.

## Accessing External Files

A **gateway** notifies end users of resources that exist outside of the API
along with the conditions under which those resources may or may not be
accessed. Gateways can be open access (open to anyone—even anonymously),
registered access (requires authentication but not authorization),
controlled access (requires both authentication and authorization), or
closed access (not available). Gateways do not, in and of themselves,
communicate the location of or mechanism(s) by which resources can be accessed.
Instead, gateways wrap `Link`s to communicate that information.

A **link** defines the mechanism for locating (referred to here as \"navigating
to\") an external resource. Links can be direct (for navigating to precisely the
requested resource(s)—no more and no less), approximate (a link which, when
followed, requires prior manual intervention in the form of instructions to
proceed to the otherwise immediately navigable desired resource(s)),
informational (a link that does not navigate to a desired resource directly but
can be followed to find out more information on how to access the desired
resource(s) out-of-band), or mailing (an email address to contact, accompanied
by instructions to access the desired resource(s)) in nature.
Again, a link does not, in and of itself, communicate the requirements to
access desired resources—it must be used in conjunction with a gateway to
communicate that information.

Put simply, a **link** tells you where you need to go to attempt to access a
desired resource and the **gateway** wrapping the link tells you what the
requirements for accessing the resource are once you get there. By separating
the concepts of the requirements to access a desired resource (gateways) from
the mechanism to access the desired resource (links), we create an expressive,
combinatorial system for capturing a broad spectrum of situations.

**Note:** a _closed_ gateway is special and does not include links. Instead, 
its purpose is to describe where a resource originated from and to communicate
that the resource is otherwise unavailable. Various closed gateway statuses are
provided to indicate if and when the resources will become available.

### Examples

Below are some examples using pseudocode to illustrate these concepts. Note
that some fields have been left out of the definitions for brevity.

* A `Gateway::Controlled { Link::Exact { url: 
  \"https://example.com/files?sample=Sample001\" } }` communicates \"all of the
  resources you requested are available at 
  https://example.com/files?sample=Sample001, but be advised that the data found
  at that link is controlled access and requires authorization\".
* A `Gateway::Open { Link::MailTo { uri: \"mailto:data@example.com\" } }`
  communicates \"anyone can request the resource by emailing the provided email
  address—even if we haven't identified (authenticated) the individual
  requesting the data. In contrast, if the data provider required
  identification of the individual before sending the data (say, via a verified
  university email address), then a `Gateway::Registered` should be used
  instead. 
* A `Gateway::Registered { Link::Approximate { url: 
  \"https://example.com/data\", instructions: \"Filter data by ...\" } }`
  communicates \"the data is available to anyone who registers an account at
  https://example.com/data, but manual filtering (by following the provided
  instructions) is required to select the exact subset of desired data\".",
        contact(
            name = "Childhood Cancer Data Initiative support email",
            email = "NCIChildhoodCancerDataInitiative@mail.nih.gov",
        ),
        version = "v0.5.0",
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
            name = "File",
            description = "Files within the CCDI federated ecosystem."
        ),
        (
            name = "Metadata",
            description = "List and describe provided metadata fields."
        ),
        (
            name = "Namespace",
            description = "List and describe namespaces supported by this server."
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
        server::routes::file::file_summary,

        // Metadata.
        server::routes::metadata::metadata_fields_subject,
        server::routes::metadata::metadata_fields_sample,

        // Namespaces.
        server::routes::namespace::namespace_index,
        server::routes::namespace::namespace_show,

        // Information.
        server::routes::info::info_index,
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
        models::subject::Identifier,
        models::subject::Kind,
        models::subject::Metadata,

        // Sample models.
        models::Sample,
        models::sample::Identifier,
        models::sample::Metadata,

        // File models.
        models::File,
        models::file::Identifier,

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

        // Namespace models.
        models::Namespace,
        models::namespace::Name,
        models::namespace::Description,

        // Url model.
        models::Url,


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

        // File responses.
        responses::Files,
        responses::file::Data,
        responses::file::data::Files,
        responses::file::data::Gateways,

        // Metadata responses.
        responses::metadata::FieldDescriptions,

        // Namespace responses.
        responses::Namespace,
        responses::Namespaces,

        // Namespace responses.
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
    modifiers(&RemoveLicense)
)]
pub struct Api;

pub struct RemoveLicense;

impl Modify for RemoveLicense {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        openapi.info.license = None;
    }
}
