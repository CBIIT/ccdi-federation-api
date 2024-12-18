//! Information regarding the server itself.

use clap::crate_version;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// Information that is specific to the server itself.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::info::server::Information)]
pub struct Information {
    /// The name of this server (if it has one).
    ///
    /// This is a free-text field describing the name of this server, if it has
    /// one. The intention is to be able to describe the proper name of the
    /// application.
    #[schema(example = "Example Server")]
    name: Option<String>,

    /// The version of this server (if it has one).
    ///
    /// Though there is explicitly no versioning dictated by the specification,
    /// we recommend [Semantic Versioning v2.0](https://semver.org/) in the
    /// absence of better options to align with the scheme used by the API. Note
    /// that using the same versioning scheme does not mean that the version of
    /// your server is recommended to be the same version as the API.
    #[schema(example = "v1.22")]
    version: Option<String>,

    /// A free-text string describing the owner of the namespace.
    ///
    /// This field is intended to be the proper name of the organization that
    /// owns and operates the server. That said, we have intentionally not
    /// required this restriction, as there may be exceptions to this guideline.
    /// We recommend that you use an organization name here if you can, but you
    /// may put whatever value is appropriate to describe the owner of the
    /// server.
    ///
    /// It is recommended that you use title case for this field, though that is
    /// not strictly required.
    #[schema(example = "Example Organization")]
    owner: String,

    /// A support email address for the server.
    ///
    /// This field is required to be a valid email address (both in format and
    /// in terms of the email address being actively monitored).
    #[schema(example = "support@example.com")]
    contact_email: String,

    /// If desired, a link to a page intended to be consumed by a web browser
    /// that describes more about the owner. This can be a link to your
    /// organization's main web page or a link to a webpage describing the
    /// project.
    #[schema(example = "https://example.com")]
    about_url: Option<String>,

    /// If your code base is open source and you want to advertise that, a link
    /// to the repository where the code is stored.
    #[schema(example = "https://github.com/CBIIT/ccdi-federation-api")]
    repository_url: Option<String>,

    /// If available, a URL where users can report issues.
    #[schema(example = "https://github.com/CBIIT/ccdi-federation-api/issues")]
    issues_url: Option<String>,
}

impl Default for Information {
    fn default() -> Self {
        Self {
            name: Some(String::from("Example Server")),
            version: Some(format!("v{}", crate_version!())),
            owner: String::from(
                "Childhood Cancer Data Initiative (CCDI) API Federation Working Group",
            ),
            contact_email: String::from("NCIChildhoodCancerDataInitiative@mail.nih.gov"),
            about_url: Some(String::from(
                "https://www.cancer.gov/research/areas/childhood/childhood-cancer-data-initiative",
            )),
            repository_url: Some(String::from("https://github.com/CBIIT/ccdi-federation-api")),
            issues_url: Some(String::from(
                "https://github.com/CBIIT/ccdi-federation-api/issues",
            )),
        }
    }
}
