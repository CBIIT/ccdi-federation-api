//! Common filtering utilities.

use introspect::Introspected;

use ccdi_models as models;

use models::Entity;

pub mod sample;
pub mod subject;

/// A trait that defines a method for filtering by metadata values.
///
/// **Note:** can only be implemented for an API [`Entity`].
pub trait FilterMetadataField<T, P>
where
    T: Entity,
{
    /// Filters entities by checking if the value of the provided field name
    /// matches the value of that field within the filter parameters. Matches
    /// are case-sensitive.
    fn filter_metadata_field(self, field: String, filter_params: &P) -> Vec<T>;
}

/// Filters a list of entities based on the provided filter parameters.
///
/// # Examples
///
/// ```
/// use ccdi_cde as cde;
/// use ccdi_models as models;
/// use ccdi_server as server;
///
/// use models::metadata::field::unowned::subject::Race;
/// use models::metadata::field::unowned::subject::Sex;
/// use models::subject::metadata::Builder;
/// use models::subject::Identifier;
/// use models::subject::Kind;
/// use models::Namespace;
/// use models::Subject;
/// use server::filter::filter;
/// use server::params::filter::Subject as SubjectFilterParams;
///
/// let namespace = Namespace::try_new(
///     "organization",
///     "Example Organization",
///     "support@example.com",
///     None,
/// )
/// .unwrap();
///
/// let subjects = vec![
///     // A subject with no metadata.
///     Subject::new(
///         Identifier::new(&namespace, "SubjectName001"),
///         String::from("SubjectName001"),
///         Kind::Participant,
///         Some(Builder::default().build()),
///     ),
///     // A subject with metadata but no specified sex.
///     Subject::new(
///         Identifier::new(&namespace, "SubjectName002"),
///         String::from("SubjectName002"),
///         Kind::Participant,
///         Some(Builder::default().build()),
///     ),
///     // A subject with sex 'F'.
///     Subject::new(
///         Identifier::new(&namespace, "SubjectName003"),
///         String::from("SubjectName003"),
///         Kind::Participant,
///         Some(
///             Builder::default()
///                 .sex(Sex::new(cde::v1::subject::Sex::Female, None, None))
///                 .build(),
///         ),
///     ),
///     // A subject with sex 'F' and race 'Asian'.
///     Subject::new(
///         Identifier::new(&namespace, "SubjectName004"),
///         String::from("SubjectName004"),
///         Kind::Participant,
///         Some(
///             Builder::default()
///                 .sex(Sex::new(cde::v1::subject::Sex::Female, None, None))
///                 .append_race(Race::new(cde::v1::subject::Race::Asian, None, None))
///                 .build(),
///         ),
///     ),
/// ];
///
/// // Filtering of subjects with no parameters.
/// let mut results =
///     filter::<Subject, SubjectFilterParams>(subjects.clone(), SubjectFilterParams::default());
///
/// assert_eq!(results.len(), 4);
///
/// // Filtering of subjects with "F" in sex field.
/// let mut results = filter::<Subject, SubjectFilterParams>(
///     subjects.clone(),
///     SubjectFilterParams {
///         sex: Some(String::from("F")),
///         race: None,
///         ethnicity: None,
///         identifiers: None,
///     },
/// );
///
/// assert_eq!(results.len(), 2);
/// assert_eq!(results.first().unwrap().name(), "SubjectName003");
/// assert_eq!(results.last().unwrap().name(), "SubjectName004");
///
/// // Filtering of subjects with "F" in sex field and "Asi" in race field.
/// let mut results = filter::<Subject, SubjectFilterParams>(
///     subjects.clone(),
///     SubjectFilterParams {
///         sex: Some(String::from("F")),
///         race: Some(String::from("Asian")),
///         ethnicity: None,
///         identifiers: None,
///     },
/// );
///
/// assert_eq!(results.len(), 1);
/// assert_eq!(results.pop().unwrap().name(), "SubjectName004");
///
/// // Filtering of subjects is case-sensitive.
/// let mut results = filter::<Subject, SubjectFilterParams>(
///     subjects.clone(),
///     SubjectFilterParams {
///         sex: Some(String::from("f")),
///         race: None,
///         ethnicity: None,
///         identifiers: None,
///     },
/// );
///
/// assert_eq!(results.len(), 0);
/// ```
pub fn filter<T, P>(mut entities: Vec<T>, filter_params: P) -> Vec<T>
where
    T: Entity,
    Vec<T>: FilterMetadataField<T, P>,
    P: Introspected,
{
    for member in P::introspected_members() {
        let field = match member {
            // SAFETY: parameters will _always_ be expression as a struct with
            // named fields. If they are not, this method will not work.
            introspect::Member::Field(field) => field.identifier().unwrap().to_string(),
            // SAFETY: parameters will never be expressed as an `enum`.
            introspect::Member::Variant(_) => unreachable!(),
        };

        entities = entities.filter_metadata_field(field, &filter_params);
    }

    entities
}