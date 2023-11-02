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

/// A sample.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::Sample)]
pub struct Sample {
    /// The identifier for this [`Sample`].
    #[schema(value_type = models::sample::Identifier)]
    id: Identifier,

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
    /// use models::Sample;
    ///
    /// let sample = Sample::new(
    ///     Identifier::new("organization", "SampleName001"),
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(id: Identifier, metadata: Option<Metadata>) -> Self {
        Self { id, metadata }
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
    /// use models::Sample;
    ///
    /// let sample = Sample::new(
    ///     Identifier::new("organization", "SampleName001"),
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
    /// use models::Sample;
    ///
    /// let metadata = Builder::default().build();
    ///
    /// let sample = Sample::new(
    ///     Identifier::new("organization", "SampleName001"),
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
    /// use ccdi_models as models;
    /// use models::sample::Identifier;
    ///
    /// use models::Sample;
    ///
    /// let sample = Sample::random(Identifier::new("organization", "SampleName001"));
    /// ```
    pub fn random(identifier: Identifier) -> Self {
        let mut rng = thread_rng();

        Self {
            id: identifier.clone(),
            metadata: match rng.gen_bool(0.7) {
                true => Some(Metadata::random()),
                false => None,
            },
        }
    }
}

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

    use super::*;

    #[test]
    fn it_orders_samples_correctly() {
        let a = Sample::new(Identifier::parse("organization:A", ":").unwrap(), None);
        let b = Sample::new(Identifier::parse("organization:B", ":").unwrap(), None);

        assert_eq!(a.cmp(&b), Ordering::Less);

        let c = Sample::new(Identifier::parse("organization:C", ":").unwrap(), None);
        let b = Sample::new(Identifier::parse("organization:B", ":").unwrap(), None);

        assert_eq!(c.cmp(&b), Ordering::Greater);

        let foo = Sample::new(Identifier::parse("organization:Name", ":").unwrap(), None);
        let bar = Sample::new(Identifier::parse("organization:Name", ":").unwrap(), None);

        assert_eq!(foo.cmp(&bar), Ordering::Equal);
    }

    #[test]
    fn it_tests_equality_correctly() {
        let foo = Sample::new(Identifier::parse("organization:B", ":").unwrap(), None);
        let bar = Sample::new(Identifier::parse("organization:B", ":").unwrap(), None);

        assert!(foo == bar);

        let foo = Sample::new(Identifier::parse("organization:A", ":").unwrap(), None);
        let bar = Sample::new(Identifier::parse("organization:B", ":").unwrap(), None);

        assert!(foo != bar);

        let foo = Sample::new(
            Identifier::parse("organization:Name", ":").unwrap(),
            Some(metadata::Builder::default().build()),
        );
        let bar = Sample::new(Identifier::parse("organization:Name", ":").unwrap(), None);

        assert!(foo != bar);
    }
}
