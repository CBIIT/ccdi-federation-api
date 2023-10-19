//! Representations of samples.

use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod metadata;

pub use metadata::Metadata;

/// A sample.
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::Sample)]
pub struct Sample {
    /// The primary name for a sample used within the source server.
    ///
    /// Note that this field is not namespaced like an `identifier` is, and,
    /// instead, is intended to represent a colloquial or display name for the
    /// sample (without indicating the scope of the name).
    #[schema(example = "SampleName001")]
    name: String,

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
    /// use models::Sample;
    /// use models::sample::metadata::Builder;
    ///
    /// let sample = Sample::new(
    ///     String::from("Name"),
    ///     Some(Builder::default().build())
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(name: String, metadata: Option<Metadata>) -> Self {
        Self { name, metadata }
    }

    /// Gets the name for this [`Sample`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::Sample;
    /// use models::sample::metadata::Builder;
    ///
    /// let sample = Sample::new(
    ///     String::from("Name"),
    ///     Some(Builder::default().build())
    /// );
    ///
    /// assert_eq!(sample.name(), &String::from("Name"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Gets the metadata for this [`Sample`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::Sample;
    /// use models::sample::metadata::Builder;
    ///
    /// let metadata = Builder::default().build();
    ///
    /// let sample = Sample::new(
    ///     String::from("Name"),
    ///     Some(metadata.clone())
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
    ///
    /// use models::Sample;
    ///
    /// let sample = Sample::random(1usize);
    /// ```
    pub fn random(sample_number: usize) -> Self {
        let mut rng = thread_rng();

        Self {
            name: format!("SampleName{:0>6}", sample_number),
            metadata: match rng.gen_bool(0.7) {
                true => Some(Metadata::random()),
                false => None,
            },
        }
    }
}
