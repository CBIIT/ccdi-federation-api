//! Representations of samples.

use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod identifier;
pub mod metadata;

pub use identifier::Identifier;
pub use metadata::Metadata;

use crate::Entity;

/// A sample.
///
/// **Note:** the `subject` identifier **must** match a
/// [`Subject`](super::Subject) that both (a) is listed in the
/// [`Subject`](super::Subject) index endpoint and (b) is able to be shown with
/// the [`Subject`](super::Subject) show endpoint.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::Sample)]
pub struct Sample {
    /// The primary identifier used by the site for this [`Sample`].
    ///
    /// This namespace pointed to by this identifier must also **ALWAYS** be
    /// included in results provided by the `/namespace` endpoint (and the
    /// subsequent `/namespace/<name>` endpoint). Failure to include the
    /// namespace there signifies non-compliance with the API.
    #[schema(value_type = models::sample::Identifier)]
    id: Identifier,

    /// The identifier for the subject from which this sample was derived.
    ///
    /// This identifier **must** match a [`Subject`](super::Subject) that both
    /// (a) is listed in the [`Subject`](super::Subject) index endpoint and (b)
    /// is able to be shown with the [`Subject`](super::Subject) show endpoint.
    #[schema(value_type = models::subject::Identifier)]
    subject: crate::subject::Identifier,

    /// Metadata associated with this [`Sample`].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<models::sample::Metadata>)]
    metadata: Option<Metadata>,
}

impl Sample {
    /// Creates a new [`Sample`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::sample::metadata::Builder;
    /// use models::sample::Identifier;
    /// use models::Namespace;
    /// use models::Sample;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let sample = Sample::new(
    ///     Identifier::new(&namespace, "SampleName001"),
    ///     models::subject::Identifier::new(&namespace, "SubjectName001"),
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        id: Identifier,
        subject: crate::subject::Identifier,
        metadata: Option<Metadata>,
    ) -> Self {
        Self {
            id,
            subject,
            metadata,
        }
    }

    /// Gets the name for this [`Sample`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::sample::metadata::Builder;
    /// use models::sample::Identifier;
    /// use models::Namespace;
    /// use models::Sample;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let sample = Sample::new(
    ///     Identifier::new(&namespace, "SampleName001"),
    ///     models::subject::Identifier::new(&namespace, "SubjectName001"),
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// assert_eq!(sample.id().namespace(), &String::from("organization"));
    /// assert_eq!(sample.id().name(), &String::from("SampleName001"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn id(&self) -> &Identifier {
        &self.id
    }

    /// Gets the identifier for the [`Subject`](super::Subject) from which this
    /// [`Sample`] was derived (by reference).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::sample::metadata::Builder;
    /// use models::sample::Identifier;
    /// use models::Namespace;
    /// use models::Sample;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let sample = Sample::new(
    ///     Identifier::new(&namespace, "SampleName001"),
    ///     models::subject::Identifier::new(&namespace, "SubjectName001"),
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// assert_eq!(sample.subject().namespace(), &String::from("organization"));
    /// assert_eq!(sample.subject().name(), &String::from("SubjectName001"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn subject(&self) -> &crate::subject::Identifier {
        &self.subject
    }

    /// Gets the metadata for this [`Sample`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::sample::metadata::Builder;
    /// use models::sample::Identifier;
    /// use models::Namespace;
    /// use models::Sample;
    ///
    /// let metadata = Builder::default().build();
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let sample = Sample::new(
    ///     Identifier::new(&namespace, "SampleName001"),
    ///     models::subject::Identifier::new(&namespace, "SubjectName001"),
    ///     Some(metadata.clone()),
    /// );
    ///
    /// assert_eq!(sample.metadata(), Some(&metadata));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Generates a random [`Sample`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::sample::Identifier;
    /// use models::Namespace;
    /// use models::Sample;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let sample = Sample::random(
    ///     Identifier::new(&namespace, "SampleName001"),
    ///     models::subject::Identifier::new(&namespace, "SubjectName001"),
    /// );
    /// ```
    pub fn random(identifier: Identifier, subject: crate::subject::Identifier) -> Self {
        let mut rng = thread_rng();

        Self {
            id: identifier.clone(),
            subject,
            metadata: match rng.gen_bool(0.7) {
                true => Some(Metadata::random()),
                false => None,
            },
        }
    }
}

impl Entity for Sample {}

impl PartialOrd for Sample {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Samples are sorted purely by identifier: the values contained _within_ a
// [`Sample`] are not relevant to the sort order. They are, however, relevant
// to equality—thus, why [`Eq`] and [`PartialEq`] are derived.
impl Ord for Sample {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::Namespace;

    use super::*;

    #[test]
    fn it_orders_samples_correctly() {
        // SAFETY: this is manually crafted to unwrap every time, as the
        // organization name conforms to the correct pattern.
        let namespace = Namespace::try_new(
            "organization",
            "Example Organization",
            "support@example.com",
            None,
        )
        .unwrap();

        let a = Sample::new(
            Identifier::new(&namespace, "A"),
            crate::subject::Identifier::new(&namespace, "SubjectA"),
            None,
        );
        let b = Sample::new(
            Identifier::new(&namespace, "B"),
            crate::subject::Identifier::new(&namespace, "SubjectB"),
            None,
        );

        assert_eq!(a.cmp(&b), Ordering::Less);

        let c = Sample::new(
            Identifier::new(&namespace, "C"),
            crate::subject::Identifier::new(&namespace, "SubjectC"),
            None,
        );
        let b = Sample::new(
            Identifier::new(&namespace, "B"),
            crate::subject::Identifier::new(&namespace, "SubjectB"),
            None,
        );

        assert_eq!(c.cmp(&b), Ordering::Greater);

        let foo = Sample::new(
            Identifier::new(&namespace, "Name"),
            crate::subject::Identifier::new(&namespace, "Subject"),
            None,
        );
        let bar = Sample::new(
            Identifier::new(&namespace, "Name"),
            crate::subject::Identifier::new(&namespace, "Subject"),
            None,
        );

        assert_eq!(foo.cmp(&bar), Ordering::Equal);
    }

    #[test]
    fn it_tests_equality_correctly() {
        // SAFETY: this is manually crafted to unwrap every time, as the
        // organization name conforms to the correct pattern.
        let namespace = Namespace::try_new(
            "organization",
            "Example Organization",
            "support@example.com",
            None,
        )
        .unwrap();

        let foo = Sample::new(
            Identifier::new(&namespace, "B"),
            crate::subject::Identifier::new(&namespace, "SubjectB"),
            None,
        );
        let bar = Sample::new(
            Identifier::new(&namespace, "B"),
            crate::subject::Identifier::new(&namespace, "SubjectB"),
            None,
        );

        assert!(foo == bar);

        let foo = Sample::new(
            Identifier::new(&namespace, "A"),
            crate::subject::Identifier::new(&namespace, "SubjectA"),
            None,
        );
        let bar = Sample::new(
            Identifier::new(&namespace, "B"),
            crate::subject::Identifier::new(&namespace, "SubjectB"),
            None,
        );

        assert!(foo != bar);

        let foo = Sample::new(
            Identifier::new(&namespace, "Name"),
            crate::subject::Identifier::new(&namespace, "SubjectName"),
            Some(metadata::Builder::default().build()),
        );
        let bar = Sample::new(
            Identifier::new(&namespace, "Name"),
            crate::subject::Identifier::new(&namespace, "SubjectName"),
            None,
        );

        assert!(foo != bar);
    }
}
