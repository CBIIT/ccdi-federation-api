//! Unharmonized metadata field descriptions.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Url;

/// An unharmonized metadata field description.
///
/// Unharmonized keys may be any valid JSON string.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::metadata::field::description::Unharmonized)]
pub struct Unharmonized {
    /// Whether or not this field is harmonized across the ecosystem.
    ///
    /// This will always be set to `false`.
    #[schema(default = false)]
    harmonized: bool,

    /// A display name for this metadata field as _suggested_ by the server (this
    /// is not considered authoritative and can be ignored by the client if it so
    /// chooses). This is mainly to avoid naming collisions of common fields across
    /// servers.
    name: Option<String>,

    /// A plain-text description of what the field represents.
    description: Option<String>,

    /// A comma (`.`) delimited path to the field's location on the `metadata`
    /// objects returned by the various subject endpoints.
    path: String,

    /// If the field is considered harmonized across the federation ecosystem, the
    /// name of the standard to which the field is harmonized.
    ///
    /// If the field is _not_ harmonized across the federation ecosystem, then this
    /// should be [`None`].
    standard: Option<String>,

    /// A URL that describes more about the metadata field, if available.
    #[schema(value_type = Option<models::Url>)]
    url: Option<Url>,
}

impl Unharmonized {
    /// Creates a new [unharmonized metadata field description](Unharmonized).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::description::Unharmonized;
    ///
    /// let field = Unharmonized::new(
    ///     Some(String::from("test")),
    ///     Some(String::from("A description.")),
    ///     String::from("test"),
    ///     None,
    ///     None,
    /// );
    /// ```
    pub fn new(
        name: Option<String>,
        description: Option<String>,
        path: String,
        standard: Option<String>,
        url: Option<Url>,
    ) -> Self {
        Unharmonized {
            harmonized: false,
            name,
            description,
            path,
            standard,
            url,
        }
    }

    /// Gets the name of the [`Unharmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::description::Unharmonized;
    ///
    /// let field = Unharmonized::new(
    ///     Some(String::from("test")),
    ///     Some(String::from("A description.")),
    ///     String::from("test"),
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(field.name(), Some(&String::from("test")))
    /// ```
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Gets the description of the [`Unharmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::description::Unharmonized;
    ///
    /// let field = Unharmonized::new(
    ///     Some(String::from("test")),
    ///     Some(String::from("A description.")),
    ///     String::from("test"),
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(field.description(), Some(&String::from("A description.")))
    /// ```
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Gets the path of the [`Unharmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::description::Unharmonized;
    ///
    /// let field = Unharmonized::new(
    ///     Some(String::from("test")),
    ///     Some(String::from("A description.")),
    ///     String::from("test"),
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(field.path(), &String::from("test"))
    /// ```
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Gets the harmonization standard name of the [`Unharmonized`] by
    /// reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::description::Unharmonized;
    ///
    /// let field = Unharmonized::new(
    ///     Some(String::from("test")),
    ///     Some(String::from("A description.")),
    ///     String::from("test"),
    ///     Some(String::from("US Census Bureau")),
    ///     None,
    /// );
    ///
    /// assert_eq!(field.standard().unwrap(), &String::from("US Census Bureau"))
    /// ```
    pub fn standard(&self) -> Option<&String> {
        self.standard.as_ref()
    }

    /// Gets the URL for which one can learn more about the [`Unharmonized`] by
    /// reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::description::Unharmonized;
    /// use models::Url;
    ///
    /// let field = Unharmonized::new(
    ///     Some(String::from("test")),
    ///     Some(String::from("A description.")),
    ///     String::from("test"),
    ///     None,
    ///     Some("https://cancer.gov".parse::<Url>().unwrap()),
    /// );
    ///
    /// assert_eq!(field.url().unwrap().as_str(), "https://cancer.gov/")
    /// ```
    pub fn url(&self) -> Option<&Url> {
        self.url.as_ref()
    }
}
