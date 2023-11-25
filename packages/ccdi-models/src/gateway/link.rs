use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Url;

/// A link to an external resource.
///
/// A link communicates information about where a resource is location alongside
/// additional context regarding how the link should be interpreted (via the
/// `kind` field). All [`Link`]s include a `url` field pointing to the external
/// resource. In the case of [`Link::Approximate`] and [`Link::MailTo`], a
/// required `instructions` field is included to instruct the user of what steps
/// to take after the link has been followed (see the definition of
/// [`Link::Approximate`] and [`Link::MailTo`] for more details, respectively).
///
/// **Note:** the context of what resources are desired compared with what
/// resources the link represents is an important consideration when
/// constructing the correct [`Link`]. For example, if the desired resource is a
/// specific file, but the server can only construct a link to a cohort of
/// files, a [`Link::Approximate`] should be used. In contrast, if the desired
/// resource is the entire cohort of files, a [`Link::Direct`] should be used.
///
/// **Note:** the link does not imply the access level or immediate availability
/// of the data—it only points a user to where they can _attempt_ to access the
/// data. Instead, [`Link`]s are always wrapped in a [`Gateway`](super::Gateway)
/// that communicates the access level or requirements. In other words, a
/// [`Link`] can absolutely require authentication or authorization before data
/// becomes accessible (and the corresponding [`Gateway`](super::Gateway) within
/// which the [`Link`] is embedded should reflect this).
///
/// ## Examples
///
/// * If the data is contained within a file where a direct link can be
///   constructed, whether that file is open access or controlled, then a
///   [`Link::Direct`] should be constructed with a link directly to that file.
/// * In the event that study data is deposited as a study within a larger data
///   repository, such as the database of Genotypes and Phenotypes (dbGaP) or
///   the European Genome-phenome Archive (EGA), and the `url` points to the
///   study page:
///     * If the desired resource is a specific file or subset of (but not all)
///       files within the study, a [`Link::Approximate`] should be returned.
///       This is because the link includes more files than what was
///       specifically requested—thus, instructions on how to filter to the
///       files requested must be communicated.
///     * If the desired resource is _every_ file in the study, then a
///       [`Link::Direct`] should be returned pointing to the study page.
/// * If the data is not immediately requestable through a webpage but there
///   exists an informational page on how to request the data using an
///   out-of-band process, then a [`Link::Informational`] should be used.
/// * If the data is available after contacting an email address, then a
///   [`Link::MailTo`] should be used.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(tag = "kind")]
#[schema(as = models::gateway::Link)]
pub enum Link {
    /// A link that points precisely to the desired resource.
    ///
    /// [`Link::Direct`] is used when the resource pointed to by the URL
    /// exactly matches the desired content—no more and no less.
    ///
    /// For example, if a user wants the RNA-Seq BAM files for a particular
    /// sample and the source server can provide a link that filters down to
    /// only those files using query parameters, [`Link::Direct`] should be
    /// used.
    Direct {
        /// The URL.
        #[schema(value_type = models::Url)]
        url: Url,
    },

    /// A link that points _approximately_ to the desired resource.
    ///
    /// [`Link::Approximate`] is used when an direct link to a desired resource
    /// cannot be constructed by the server, but a link to _approximately_ the
    /// desired resource followed by following some manual instructions will
    /// deliver the desired resources.
    ///
    /// For example, if a user wants the RNA-Seq BAM files for a particular
    /// sample, but the server can only construct a link to the particular
    /// sample and file type and/or sequencing type must be filtered manually in
    /// the user interface, then a [`Link::Approximate`] should be constructed
    /// with (a) the `url` field pointing to the sample link and (b) the
    /// `instructions` field telling the end user how to filter to their desired
    /// content within the user interface.
    Approximate {
        /// The URL.
        #[schema(value_type = models::Url)]
        url: Url,

        /// The manual instructions to follow after navigating to the URL.
        ///
        /// As much as is practical, instructions should be as specific as
        /// possible to the desired data. We expect that generating dynamic
        /// instrucions based on the user's selection will require a
        /// non-trivial amount of development effort rather than returning
        /// generic set of instructions. The reason for this is because this
        /// field is intended to be surfaced as tailored instructions with a
        /// user interface detailing to the user _exactly_ how to retrieve their
        /// desired data.
        ///
        /// For example, if a user is requesting whole-genome sequence BAM files
        /// for a particular sample but the server can only construct a link to
        /// _all_ files for the sample, the instructions should specify how to
        /// filter down to only the whole-genome sequence BAM files
        /// specifically—not how to operate filters within the interface
        /// generally).
        instructions: String,
    },

    /// A link that points to information about how to retrieve the desired
    /// resource.
    ///
    /// [`Link::Informational`] is used when a link to the desired content
    /// cannot be constructed by the server, but a link describing how to gain
    /// access to the resource is known. This [`Link`] is reserved for
    /// situations where the data is not accessible except via an out-of-band
    /// process—if the resource can be reached (not necessarily accessed) by
    /// following a URL and a subsequent set of instructions, a
    /// [`Link::Approximate`] should be used instead.
    ///
    /// For example, if a request form exists where, once approved, some data is
    /// sent to the user in a manual fashion, then a [`Link::Informational`]
    /// should be used (the distinguishing factor being that the user cannot
    /// complete the request process without some out-of-band or manual
    /// process).
    ///
    /// **Note:** no further informational field is provided within the API
    /// because the expectation is that the information on what steps should be
    /// taken next will be available at the specified URL.
    Informational {
        /// The URL.
        #[schema(value_type = models::Url)]
        url: Url,
    },

    /// A link that points to an email address to request access to the
    /// resource.
    ///
    /// [`Link::MailTo`] is used when the process of requesting data access is
    /// to email
    MailTo {
        /// A url beginning with `mailto:` to a monitored email address.
        #[schema(value_type = models::Url)]
        url: Url,

        /// The instructions to follow when constructing the email request. At a
        /// minimum, explaining what criteria are used in determining access,
        /// what information is requested, and what to expect after the
        /// email in terms of communication and timeline is recommended.
        instructions: String,
    },
}
