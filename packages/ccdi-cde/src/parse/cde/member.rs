//! Parsing the members of an entity common data elements.

mod field;
pub mod variant;

pub use field::Field;
use serde::Deserialize;
use serde::Serialize;
pub use variant::Variant;

/// An error related to parsing a [`Member`].
#[derive(Debug)]
pub enum ParseError {
    /// An error related to parsing a [`Field`].
    FieldError(field::ParseError),

    /// An error related to parsing a [`Variant`].
    VariantError(variant::ParseError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::FieldError(err) => write!(f, "field parsing error: {err}"),
            ParseError::VariantError(err) => write!(f, "variant parsing error: {err}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// A parsed member of an entity that describes a common data element. A member
/// is either a member of a `struct` or a variant of an `enum`. (both can be used
/// to describe common data elements).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Member {
    /// A documentation block parsed for information pertaining to a field.
    Field(field::Field),

    /// A documentation block parsed for information pertaining to a variant.
    Variant(variant::Variant),
}

impl Member {
    /// Returns whether or not this member is a [`Member::Field`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::member::Field;
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Member;
    ///
    /// let field = "A description.".parse::<Field>()?;
    /// let member = Member::Field(field);
    /// assert_eq!(member.is_field(), true);
    ///
    /// let variant = "`A Permissible Value`
    ///
    /// A description."
    ///     .parse::<Variant>()?;
    /// let member = Member::Variant(variant);
    /// assert_eq!(member.is_field(), false);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_field(&self) -> bool {
        matches!(self, Member::Field(_))
    }

    /// Gets a reference to the inner [`Field`] if this [`Member`] is a
    /// [`Member::Field`]. Else, this returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::member::Field;
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Member;
    ///
    /// let field = "A description.".parse::<Field>()?;
    /// let member = Member::Field(field.clone());
    /// assert_eq!(member.get_field(), Some(&field));
    ///
    /// let variant = "`A Permissible Value`
    ///
    /// A description."
    ///     .parse::<Variant>()?;
    /// let member = Member::Variant(variant.clone());
    /// assert_eq!(member.get_field(), None);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_field(&self) -> Option<&field::Field> {
        match self {
            Member::Field(field) => Some(field),
            _ => None,
        }
    }

    /// Returns whether or not this member is a [`Member::Variant`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::member::Field;
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Member;
    ///
    /// let field = "A description.".parse::<Field>()?;
    /// let member = Member::Field(field);
    /// assert_eq!(member.is_field(), true);
    ///
    /// let variant = "`A Permissible Value`
    ///
    /// A description."
    ///     .parse::<Variant>()?;
    /// let member = Member::Variant(variant);
    /// assert_eq!(member.is_field(), false);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_variant(&self) -> bool {
        matches!(self, Member::Variant(_))
    }

    /// Gets a reference to the inner [`Variant`] if this [`Member`] is a
    /// [`Member::Variant`]. Else, this returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::member::Field;
    /// use cde::parse::cde::member::Variant;
    /// use cde::parse::cde::Member;
    ///
    /// let field = "A description.".parse::<Field>()?;
    /// let member = Member::Field(field.clone());
    /// assert_eq!(member.get_variant(), None);
    ///
    /// let variant = "`A Permissible Value`
    ///
    /// A description."
    ///     .parse::<Variant>()?;
    /// let member = Member::Variant(variant.clone());
    /// assert_eq!(member.get_variant(), Some(&variant));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_variant(&self) -> Option<&variant::Variant> {
        match self {
            Member::Variant(variant) => Some(variant),
            _ => None,
        }
    }
}
