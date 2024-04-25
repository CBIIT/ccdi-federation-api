//! Harmonized metadata field descriptions.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_cde as cde;

use cde::parse::cde::Member;

use crate::Url;

pub mod file;
pub mod namespace;
pub mod organization;
pub mod sample;
mod standard;
pub mod subject;

pub use standard::Standard;

/// A kind of harmonized value.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum Kind {
    /// An enum.
    Enum,

    /// A struct.
    Struct,
}

/// A harmonized metadata field description.
///
/// Harmonized keys _must_ fit the regex pattern `^[a-z_]+$`.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::metadata::field::description::Harmonized)]
pub struct Harmonized {
    /// Whether or not this field is harmonized across the ecosystem.
    ///
    /// This will always be set to `true`.
    #[schema(default = true)]
    harmonized: bool,

    /// The kind of harmonized metadata field.
    #[serde(skip_serializing)]
    kind: Kind,

    /// A comma (`.`) delimited path to the field's location on the `metadata`
    /// objects returned by the various subject endpoints.
    path: String,

    /// A description of the harmonized metadata field.
    #[serde(skip_serializing)]
    description: String,

    /// A URL to the CCDI wiki documentation where the definition of this
    /// harmonized field resides.
    #[schema(value_type = models::Url)]
    wiki_url: Url,

    /// If available, the standard to which this field is harmonized (this field
    /// is defined by the documentation for the CCDI metadata fields when the
    /// field is backed by a CDE).
    #[schema(value_type = Option<models::metadata::field::description::harmonized::Standard>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    standard: Option<Standard>,

    /// If present, the parsed [`Member`]s and their respective identifiers of
    /// the entity. For a `struct`, this equates to each of the members within
    /// the `struct`. For an `enum`, this is all of the available variants for
    /// the `enum`.
    #[serde(skip_serializing)]
    members: Option<Vec<(Option<String>, Member)>>,
}

impl Harmonized {
    /// Creates a new [harmonized metadata field description](Harmonized).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use models::metadata::field::description::harmonized::Kind;
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::metadata::field::description::Harmonized;
    /// use models::Url;
    ///
    /// let description = Harmonized::new(
    ///     Kind::Enum,
    ///     String::from("entity"),
    ///     String::from("A description for the entity."),
    ///     "https://github.com/CBIIT/ccdi-federation-api/wiki"
    ///         .parse::<Url>()
    ///         .unwrap(),
    ///     Some(Standard::new(
    ///         String::from("caDSR ------ v1.00"),
    ///         "https://cancer.gov".parse::<Url>().unwrap(),
    ///     )),
    ///     Some(vec![(
    ///         Some(String::from("Unknown")),
    ///         Member::Variant(
    ///             "`Unknown`
    ///              
    ///             A description for the variant."
    ///                 .parse::<Variant>()
    ///                 .unwrap(),
    ///         ),
    ///     )]),
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        kind: Kind,
        path: String,
        description: String,
        wiki_url: Url,
        standard: Option<Standard>,
        members: Option<Vec<(Option<String>, Member)>>,
    ) -> Self {
        Harmonized {
            harmonized: true,
            kind,
            path,
            description,
            wiki_url,
            standard,
            members,
        }
    }

    /// Gets the [`Kind`] of the [`Harmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use models::metadata::field::description::harmonized::Kind;
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::metadata::field::description::Harmonized;
    /// use models::Url;
    ///
    /// let description = Harmonized::new(
    ///     Kind::Enum,
    ///     String::from("entity"),
    ///     String::from("A description for the entity."),
    ///     "https://github.com/CBIIT/ccdi-federation-api/wiki"
    ///         .parse::<Url>()
    ///         .unwrap(),
    ///     Some(Standard::new(
    ///         String::from("caDSR ------ v1.00"),
    ///         "https://cancer.gov".parse::<Url>().unwrap(),
    ///     )),
    ///     Some(vec![(
    ///         Some(String::from("Unknown")),
    ///         Member::Variant(
    ///             "`Unknown`
    ///              
    ///             A description for the variant."
    ///                 .parse::<Variant>()
    ///                 .unwrap(),
    ///         ),
    ///     )]),
    /// );
    ///
    /// assert_eq!(description.kind(), &Kind::Enum);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Gets the path of the [`Harmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use models::metadata::field::description::harmonized::Kind;
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::metadata::field::description::Harmonized;
    /// use models::Url;
    ///
    /// let description = Harmonized::new(
    ///     Kind::Enum,
    ///     String::from("entity"),
    ///     String::from("A description for the entity."),
    ///     "https://github.com/CBIIT/ccdi-federation-api/wiki"
    ///         .parse::<Url>()
    ///         .unwrap(),
    ///     Some(Standard::new(
    ///         String::from("caDSR ------ v1.00"),
    ///         "https://cancer.gov".parse::<Url>().unwrap(),
    ///     )),
    ///     Some(vec![(
    ///         Some(String::from("Unknown")),
    ///         Member::Variant(
    ///             "`Unknown`
    ///              
    ///             A description for the variant."
    ///                 .parse::<Variant>()
    ///                 .unwrap(),
    ///         ),
    ///     )]),
    /// );
    ///
    /// assert_eq!(description.path(), "entity");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    /// Gets the description for the [`Harmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use models::metadata::field::description::harmonized::Kind;
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::metadata::field::description::Harmonized;
    /// use models::Url;
    ///
    /// let description = Harmonized::new(
    ///     Kind::Enum,
    ///     String::from("entity"),
    ///     String::from("A description for the entity."),
    ///     "https://github.com/CBIIT/ccdi-federation-api/wiki"
    ///         .parse::<Url>()
    ///         .unwrap(),
    ///     Some(Standard::new(
    ///         String::from("caDSR ------ v1.00"),
    ///         "https://cancer.gov".parse::<Url>().unwrap(),
    ///     )),
    ///     Some(vec![(
    ///         Some(String::from("Unknown")),
    ///         Member::Variant(
    ///             "`Unknown`
    ///              
    ///             A description for the variant."
    ///                 .parse::<Variant>()
    ///                 .unwrap(),
    ///         ),
    ///     )]),
    /// );
    ///
    /// assert_eq!(description.description(), "A description for the entity.");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Gets the wiki URL for the [`Harmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use models::metadata::field::description::harmonized::Kind;
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::metadata::field::description::Harmonized;
    /// use models::Url;
    ///
    /// let description = Harmonized::new(
    ///     Kind::Enum,
    ///     String::from("entity"),
    ///     String::from("A description for the entity."),
    ///     "https://github.com/CBIIT/ccdi-federation-api/wiki"
    ///         .parse::<Url>()
    ///         .unwrap(),
    ///     Some(Standard::new(
    ///         String::from("caDSR ------ v1.00"),
    ///         "https://cancer.gov".parse::<Url>().unwrap(),
    ///     )),
    ///     Some(vec![(
    ///         Some(String::from("Unknown")),
    ///         Member::Variant(
    ///             "`Unknown`
    ///              
    ///             A description for the variant."
    ///                 .parse::<Variant>()
    ///                 .unwrap(),
    ///         ),
    ///     )]),
    /// );
    ///
    /// assert_eq!(
    ///     description.wiki_url(),
    ///     "https://github.com/CBIIT/ccdi-federation-api/wiki"
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn wiki_url(&self) -> &str {
        self.wiki_url.as_ref()
    }

    /// Gets the harmonization standard of the [`Harmonized`] by reference (if
    /// it exists).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use models::metadata::field::description::harmonized::Kind;
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::metadata::field::description::Harmonized;
    /// use models::Url;
    ///
    /// let description = Harmonized::new(
    ///     Kind::Enum,
    ///     String::from("entity"),
    ///     String::from("A description for the entity."),
    ///     "https://github.com/CBIIT/ccdi-federation-api/wiki"
    ///         .parse::<Url>()
    ///         .unwrap(),
    ///     Some(Standard::new(
    ///         String::from("caDSR ------ v1.00"),
    ///         "https://cancer.gov".parse::<Url>().unwrap(),
    ///     )),
    ///     Some(vec![(
    ///         Some(String::from("Unknown")),
    ///         Member::Variant(
    ///             "`Unknown`
    ///              
    ///             A description for the variant."
    ///                 .parse::<Variant>()
    ///                 .unwrap(),
    ///         ),
    ///     )]),
    /// );
    ///
    /// assert_eq!(description.standard().unwrap().name(), "caDSR ------ v1.00");
    /// assert_eq!(description.standard().unwrap().url(), "https://cancer.gov/");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn standard(&self) -> Option<&Standard> {
        self.standard.as_ref()
    }

    /// Gets the members for the [`Harmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use models::metadata::field::description::harmonized::Kind;
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::metadata::field::description::Harmonized;
    /// use models::Url;
    ///
    /// let description = Harmonized::new(
    ///     Kind::Enum,
    ///     String::from("entity"),
    ///     String::from("A description for the entity."),
    ///     "https://github.com/CBIIT/ccdi-federation-api/wiki"
    ///         .parse::<Url>()
    ///         .unwrap(),
    ///     Some(Standard::new(
    ///         String::from("caDSR ------ v1.00"),
    ///         "https://cancer.gov".parse::<Url>().unwrap(),
    ///     )),
    ///     Some(vec![(
    ///         Some(String::from("Unknown")),
    ///         Member::Variant(
    ///             "`Unknown`
    ///              
    ///             A description for the variant."
    ///                 .parse::<Variant>()
    ///                 .unwrap(),
    ///         ),
    ///     )]),
    /// );
    ///
    /// assert_eq!(description.members().unwrap().len(), 1);
    ///
    /// let (identifier, variant) = description
    ///     .members()
    ///     .unwrap()
    ///     .clone()
    ///     .into_iter()
    ///     .next()
    ///     .unwrap();
    ///
    /// assert_eq!(identifier.unwrap().as_str(), "Unknown");
    /// assert_eq!(
    ///     variant.get_variant().unwrap().permissible_value(),
    ///     "Unknown"
    /// );
    /// assert_eq!(
    ///     variant.get_variant().unwrap().description(),
    ///     "A description for the variant."
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn members(&self) -> Option<&Vec<(Option<String>, Member)>> {
        self.members.as_ref()
    }
}
