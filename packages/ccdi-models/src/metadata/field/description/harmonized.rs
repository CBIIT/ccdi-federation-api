//! Harmonized metadata field descriptions.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_cde as cde;

use cde::parse::cde::Entity;
use cde::parse::cde::Member;

pub mod sample;
pub mod subject;

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

    /// The parsed [`Entity`].
    #[serde(skip_serializing)]
    entity: Entity,

    /// The parsed [`Member`]s and their respective identifiers of the entity.
    #[serde(skip_serializing)]
    members: Vec<(String, Member)>,
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
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use cde::parse::cde::member::Variant;
    /// use models::metadata::field::description::Harmonized;
    ///
    /// let description = Harmonized::new(
    ///     "test",
    ///     "caDSR ------ v1.00",
    ///     "https://cancer.gov",
    ///     "**`A Standard`**
    ///     
    ///     A description for the entity.
    ///
    ///     Link: <https://example.com>".parse::<Entity>()?,
    ///     vec![
    ///         (
    ///             String::from("Unknown"),
    ///             Member::Variant("`Unknown`
    ///              
    ///             A description for the variant.".parse::<Variant>().unwrap())
    ///         )
    ///     ]
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        path: impl Into<String>,
        standard: impl Into<String>,
        url: impl Into<String>,
        entity: Entity,
        members: Vec<(String, Member)>,
    ) -> Self {
        Harmonized {
            harmonized: true,
            path: path.into(),
            standard: standard.into(),
            url: url.into(),
            entity,
            members,
        }
    }

    /// Gets the path of the [`Harmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use cde::parse::cde::member::Variant;
    /// use models::metadata::field::description::Harmonized;
    ///
    /// let description = Harmonized::new(
    ///     "test",
    ///     "caDSR ------ v1.00",
    ///     "https://cancer.gov",
    ///     "**`A Standard`**
    ///     
    ///     A description for the entity.
    ///
    ///     Link: <https://example.com>".parse::<Entity>()?,
    ///     vec![
    ///         (
    ///             String::from("Unknown"),
    ///             Member::Variant("`Unknown`
    ///              
    ///             A description for the variant.".parse::<Variant>().unwrap())
    ///         )
    ///     ]
    /// );
    ///
    /// assert_eq!(description.path(), &String::from("test"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Gets the harmonization standard name of the [`Harmonized`] by
    /// reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use cde::parse::cde::member::Variant;
    /// use models::metadata::field::description::Harmonized;
    ///
    /// let description = Harmonized::new(
    ///     "test",
    ///     "caDSR ------ v1.00",
    ///     "https://cancer.gov",
    ///     "**`A Standard`**
    ///     
    ///     A description for the entity.
    ///
    ///     Link: <https://example.com>".parse::<Entity>()?,
    ///     vec![
    ///         (
    ///             String::from("Unknown"),
    ///             Member::Variant("`Unknown`
    ///              
    ///             A description for the variant.".parse::<Variant>().unwrap())
    ///         )
    ///     ]
    /// );
    ///
    /// assert_eq!(description.standard(), &String::from("caDSR ------ v1.00"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn standard(&self) -> &String {
        &self.standard
    }

    /// Gets the URL for which one can learn more about the [`Harmonized`] by
    /// reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use cde::parse::cde::member::Variant;
    /// use models::metadata::field::description::Harmonized;
    ///
    /// let description = Harmonized::new(
    ///     "test",
    ///     "caDSR ------ v1.00",
    ///     "https://cancer.gov",
    ///     "**`A Standard`**
    ///     
    ///     A description for the entity.
    ///
    ///     Link: <https://example.com>".parse::<Entity>()?,
    ///     vec![
    ///         (
    ///             String::from("Unknown"),
    ///             Member::Variant("`Unknown`
    ///              
    ///             A description for the variant.".parse::<Variant>().unwrap())
    ///         )
    ///     ]
    /// );
    ///
    /// assert_eq!(description.url(), &String::from("https://cancer.gov"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Gets the entity for the [`Harmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use cde::parse::cde::member::Variant;
    /// use models::metadata::field::description::Harmonized;
    ///
    /// let description = Harmonized::new(
    ///     "test",
    ///     "caDSR ------ v1.00",
    ///     "https://cancer.gov",
    ///     "**`A Standard`**
    ///     
    ///     A description for the entity.
    ///
    ///     Link: <https://example.com>".parse::<Entity>()?,
    ///     vec![
    ///         (
    ///             String::from("Unknown"),
    ///             Member::Variant("`Unknown`
    ///              
    ///             A description for the variant.".parse::<Variant>().unwrap())
    ///         )
    ///     ]
    /// );
    ///
    /// assert_eq!(description.entity().standard(), "A Standard");
    /// assert_eq!(description.entity().description(), "A description for the entity.");
    /// assert_eq!(description.entity().url(), "https://example.com");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    /// Gets the members for the [`Harmonized`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::parse::cde::Entity;
    /// use cde::parse::cde::Member;
    /// use cde::parse::cde::member::Variant;
    /// use models::metadata::field::description::Harmonized;
    ///
    /// let description = Harmonized::new(
    ///     "test",
    ///     "caDSR ------ v1.00",
    ///     "https://cancer.gov",
    ///     "**`A Standard`**
    ///     
    ///     A description for the entity.
    ///
    ///     Link: <https://example.com>".parse::<Entity>()?,
    ///     vec![
    ///         (
    ///             String::from("Unknown"),
    ///             Member::Variant("`Unknown`
    ///              
    ///             A description for the variant.".parse::<Variant>().unwrap())
    ///         )
    ///     ]
    /// );
    ///
    /// assert_eq!(description.members().len(), 1);
    ///
    /// let (identifier, variant) = description.members()
    ///                 .first()
    ///                 .unwrap();
    ///
    /// assert_eq!(identifier.as_str(), "Unknown");
    /// assert_eq!(variant.get_variant().unwrap().permissible_value(), "Unknown");
    /// assert_eq!(variant.get_variant().unwrap().description(), "A description for the variant.");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn members(&self) -> &Vec<(String, Member)> {
        &self.members
    }
}
