//! Representations of subjects.

use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod identifier;
mod kind;
pub mod metadata;

pub use identifier::Identifier;
pub use kind::Kind;
pub use metadata::Metadata;

use crate::Entity;

/// A subject.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::Subject)]
pub struct Subject {
    /// The primary identifier used by the site for this [`Subject`].
    ///
    /// This identifier should **ALWAYS** be included in the `identifiers` key
    /// under `metadata`, should that key exist.
    ///
    /// This namespace pointed to by this identifier must also **ALWAYS** be
    /// included in results provided by the `/namespace` endpoint (and the
    /// subsequent `/namespace/<name>` endpoint). Failure to include the
    /// namespace in the results at that endpoint signifies non-compliance with
    /// the API.
    #[schema(value_type = models::subject::Identifier)]
    id: Identifier,

    /// The kind of [`Subject`].
    #[schema(value_type = models::subject::Kind)]
    kind: Kind,

    /// Harmonized metadata associated with this [`Subject`].
    #[schema(
        value_type = Option<models::subject::Metadata>,
        nullable = true
    )]
    metadata: Option<Metadata>,
}

impl Subject {
    /// Creates a new [`Subject`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Subject;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    ///     None,
    /// );
    ///
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    ///
    /// Subject::new(
    ///     subject_id,
    ///     Kind::Participant,
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(id: Identifier, kind: Kind, metadata: Option<Metadata>) -> Self {
        Self { id, kind, metadata }
    }

    /// Gets the identifier for this [`Subject`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Subject;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    ///     None,
    /// );
    ///
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    ///
    /// let subject = Subject::new(
    ///     subject_id,
    ///     Kind::Participant,
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// assert_eq!(subject.id().namespace().name().as_str(), "ExampleNamespace");
    /// assert_eq!(subject.id().name().as_str(), "SubjectName001");
    /// ```
    pub fn id(&self) -> &Identifier {
        &self.id
    }

    /// Gets the kind for this [`Subject`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Subject;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    ///     None,
    /// );
    ///
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    ///
    /// let subject = Subject::new(
    ///     subject_id,
    ///     Kind::Participant,
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// assert_eq!(subject.kind(), &Kind::Participant);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Gets the metadata for this [`Subject`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Subject;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    ///     None,
    /// );
    ///
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    /// let metadata = Builder::default().build();
    /// let subject = Subject::new(subject_id, Kind::Participant, Some(metadata.clone()));
    ///
    /// assert_eq!(subject.metadata(), Some(&metadata));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Generates a random [`Subject`] based on a particular [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Subject;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    ///     None,
    /// );
    ///
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    /// let subject = Subject::random(subject_id);
    /// ```
    pub fn random(identifier: Identifier) -> Self {
        let mut rng = thread_rng();

        Self {
            id: identifier.clone(),
            kind: rand::random(),
            metadata: match rng.gen_bool(0.7) {
                true => Some(Metadata::random(identifier)),
                false => None,
            },
        }
    }
}

impl Entity for Subject {}

impl PartialOrd for Subject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Subjects are sorted purely by identifier: the values contained _within_ a
// [`Subject`] are not relevant to the sort order. They are, however, relevant
// to equality—thus, why [`Eq`] and [`PartialEq`] are derived.
impl Ord for Subject {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use ccdi_cde::v1::subject::Name;

    use crate::namespace;
    use crate::organization;
    use crate::Namespace;

    use super::*;

    #[test]
    fn it_orders_samples_correctly() {
        // SAFETY: this is manually crafted to unwrap every time, as the
        // organization name conforms to the correct pattern.
        let namespace = Namespace::new(
            namespace::Identifier::new(
                organization::Identifier::try_new("example-organization").unwrap(),
                namespace::identifier::Name::try_new("ExampleNamespace").unwrap(),
            ),
            "support@example.com",
            Some(
                "ExampleNamespace"
                    .parse::<namespace::Description>()
                    .unwrap(),
            ),
            None,
        );

        let a = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("A")),
            Kind::Participant,
            None,
        );
        let b = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("B")),
            Kind::Participant,
            None,
        );

        assert_eq!(a.cmp(&b), Ordering::Less);

        let c = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("C")),
            Kind::Participant,
            None,
        );
        let b = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("B")),
            Kind::Participant,
            None,
        );

        assert_eq!(c.cmp(&b), Ordering::Greater);

        let foo = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("Name")),
            Kind::Participant,
            None,
        );
        let bar = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("Name")),
            Kind::Participant,
            None,
        );

        assert_eq!(foo.cmp(&bar), Ordering::Equal);
    }

    #[test]
    fn it_tests_equality_correctly() {
        // SAFETY: this is manually crafted to unwrap every time, as the
        // organization name conforms to the correct pattern.
        let namespace = Namespace::new(
            namespace::Identifier::new(
                organization::Identifier::try_new("example-organization").unwrap(),
                namespace::identifier::Name::try_new("ExampleNamespace").unwrap(),
            ),
            "support@example.com",
            Some(
                "ExampleNamespace"
                    .parse::<namespace::Description>()
                    .unwrap(),
            ),
            None,
        );

        let foo = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("Name")),
            Kind::Participant,
            None,
        );
        let bar = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("Name")),
            Kind::Participant,
            None,
        );

        assert!(foo == bar);

        let foo = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("A")),
            Kind::Participant,
            None,
        );
        let bar = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("B")),
            Kind::Participant,
            None,
        );

        assert!(foo != bar);

        let foo = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("Name")),
            Kind::Participant,
            None,
        );
        let bar = Subject::new(
            Identifier::new(namespace.id().clone(), Name::new("Name")),
            Kind::Participant,
            Some(metadata::Builder::default().build()),
        );

        assert!(foo != bar);
    }
}
