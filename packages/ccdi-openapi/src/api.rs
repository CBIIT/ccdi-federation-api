use models::metadata::field;
use models::metadata::fields;
use server::params;
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
        title = "CCDI Data Federation API",
        description = "The CCDI Data Federation API supports the querying of
federated pediatric cancer within the broader community. The goal of the API
is to support identification of pediatric cancer samples of interest via a
variety of query parameters.

### Definitions

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

**Primary Entities** are defined as classes of information for which the API
specification was created to share. In other words, the sharing of primary 
entities is the most important goal of the specification.

**Supporting Entities** are defined as classes of information that are necessary
to share alongside to make sense of primary entities. Sharing information on
these entities is not a top-level goal of the API specification.

### Security Requirements

All API endpoints must be served over HTTPS (port 443) with a certificate signed
by a recognized certificate authority. In particular, self-signed certificates
are not permitted. Further, while an API _may_ be available over HTTP (port 80),
HTTPS must always be available. We highly recommend servers redirect HTTP to
HTTPS rather than serve your API on two separate ports.

### Invalid Routes

All responses that do not match an endpoint below should return a Not Found 
(`404`) response. The body of this response should be the `responses.Errors` 
JSON object with one `responses.error.Kind` where the `Kind` matches the 
`InvalidRoute` error.

## Primary Entities

Primary entities represent information that this API specification was created
to share as a top-level goal. Primary entities have a common API surface and,
generally, will work relatively similar to one another within the specification.
All primary entities are referred to by their _identifier_, which is the
combination of (a) a namespace identifier pointing to the namespace that owns
this entity along with (b) a name for the entity.

The following entities are considered primary entities within the API
specification.

- Subjects
- Samples
- Files


In addition to merely existing in a common level of prominence within the API,
primary entities have a hierarchical structure following these rules.

- Subjects are the highest-level primary entity within the API specification.
- Samples **must** be associated with one and only one subject.
- Files **must** be associated with one or more samples.

### Subjects

TODO

### Samples

TODO

### Files

TODO

#### Accessing External Files

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

##### Examples

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
  instructions) is required to select the exact subset of desired data\".

## Supporting Entities

Supporting entities provide supporting information necessary to make sense of
the primary entities supported by the API. Supporting entities are not, in and
of themselves, a primary sharing goal for the API.

The following entities are considered supporting entities within the API
specification.

- Organizations
- Namespaces

### Organizations

Organizations are self-reported, non-authoritative descriptions of organizations
that are sharing data through an API endpoint. There is no formal definition or
criteria for what constitutes an organization in this context. Some examples of
what an organization might represent include (but are not limited to) 
for-profit companies, non-profit organizations, consortiums, informal bodies, or
any combination of these concepts.

### Namespaces

Namespaces represent top-level governance groupings of primary entities within
the CCDI Federation API. Each namespace is owned by an existing
organization entity, contains information about the governance unit, and 
provides information on how to contact the body that governs this namespace.

### Assigning Organizations and Namespaces

When assigning namespaces within a source server, one should consider making a
namespace entity for each grouping of primary entities that are governed under
a common model. 

Here are some common situations followed by instructive examples of how you
partition primary entities to a set of namespaces under that situation:

- If all of the primary entities within your source server are governed by a
  singular governing body (say, a single data access committee), then you may
  only need one namespace for all of the primaries entities within your server.
- If you have multiple data access committees governing different groupings of
  primary entities from the same institution, you should create multiple 
  namespaces that are backed by a common organization.
- If your server serves data from various governing bodies spread across
  multiple, independent organizations, you should create multiple namespaces
  backed by multiple organizations.",
        contact(
            name = "Childhood Cancer Data Initiative support email",
            email = "NCIChildhoodCancerDataInitiative@mail.nih.gov",
        ),
        version = "v0.6.1",
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
        server::routes::file::file_summary,

        // Metadata.
        server::routes::metadata::metadata_fields_subject,
        server::routes::metadata::metadata_fields_sample,

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
        // Harmonized subject metadata elements.
        cde::v1::subject::Race,
        cde::v1::subject::Sex,
        cde::v2::subject::Ethnicity,
        cde::v1::subject::Name,
        cde::v1::subject::VitalStatus,
        models::subject::metadata::AgeAtVitalStatus,

        // Harmonized sample metadata elements.
        models::sample::metadata::AgeAtDiagnosis,
        cde::v1::sample::DiseasePhase,
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

        // Harmonized subject fields.
        field::unowned::subject::Sex,
        field::unowned::subject::Race,
        field::unowned::subject::Ethnicity,
        field::unowned::subject::Identifier,
        field::unowned::subject::VitalStatus,
        field::unowned::subject::AgeAtVitalStatus,

        // Harmonized sample fields.
        field::unowned::sample::AgeAtDiagnosis,
        field::unowned::sample::DiseasePhase,
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
        models::namespace::Identifier,
        models::namespace::Description,

        // Organization models.
        models::Organization,
        models::organization::Identifier,
        models::organization::Name,

        // Url model.
        models::Url,

        // Params.
        params::Partitionable,

        // General responses.
        responses::Errors,

        // Summary responses.
        responses::summary::Counts,
        responses::Summary,

        // Cross-entity responses.
        responses::entity::Summary,
        responses::entity::Counts,

        // Subject responses.
        responses::Subject,
        responses::Subjects,
        responses::by::count::subject::Results,
        responses::by::count::subject::NamespacePartitionedResult,
        responses::by::count::subject::NamespacePartitionedResults,
        responses::by::count::subject::Response,

        // Sample responses.
        responses::Sample,
        responses::Samples,
        responses::by::count::sample::Results,
        responses::by::count::sample::NamespacePartitionedResult,
        responses::by::count::sample::NamespacePartitionedResults,
        responses::by::count::sample::Response,

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
    modifiers(&RemoveLicense)
)]
pub struct Api;

pub struct RemoveLicense;

impl Modify for RemoveLicense {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        openapi.info.license = None;
    }
}
