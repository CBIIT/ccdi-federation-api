use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// A statement on the expertise of the individual (or individuals) who are
/// assigning the harmonized values. This information can help data receivers
/// understand the context within which the data was harmonized (particularly in
/// data fields that may be constantly evolving or changing—for instance,
/// diagnosis).
///
/// **NOTE:** if you find that there are types of harmonizers that are not
/// captured here, please make an issue on the GitHub repository so we can
/// support the value.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::metadata::field::details::Harmonizer)]
pub enum Harmonizer {
    /// An individual who is reputed to be an expert in the domain of the
    /// harmonized value and the data source itself. Typically, these
    /// individuals are involved on the project from whence the data was
    /// generated.
    DomainExpert,

    /// An individual on the data curation team for the source server. These
    /// individuals are generally knowledgable about the subject area but, at
    /// times, lack context particular to the project itself. They are generally
    /// _not_ reputed to be an expert in the domain of the harmonized value and
    /// the data source itself
    CurationTeamMember,
}
