//! Harmonized metadata field descriptions.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// A harmonized metadata field description.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::metadata::field::description::Harmonized)]
pub struct Harmonized {
    /// Whether or not this field is harmonized across the ecosystem.
    ///
    /// This will always be set to `true`.
    #[schema(default = true)]
    harmonized: bool,

    /// A comma (`.`) delimited path to the field's location on the `metadata`
    /// objects returned by the various subject endpoints.
    path: String,

    /// The proper name of the standard to which this field is harmonized (defined
    /// by the documentation for the CCDI metadata fields).
    standard: String,

    /// A URL to the CCDI documentation where the definition of this harmonized
    /// field resides.
    url: String,
}

impl Harmonized {
    /// Creates a new [harmonized metadata field description](Harmonized).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use models::metadata::field::description::Harmonized;
    ///
    /// let description = Harmonized::new(
    ///     "test",
    ///     "caDSR ------ v1.00",
    ///     "https://cancer.gov"
    /// );
    /// ```
    pub fn new<S: Into<String>>(path: S, standard: S, url: S) -> Self {
        Harmonized {
            harmonized: true,
            path: path.into(),
            standard: standard.into(),
            url: url.into(),
        }
    }
}
