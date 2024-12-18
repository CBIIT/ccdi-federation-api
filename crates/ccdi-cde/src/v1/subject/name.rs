//! A name for a subject.

use std::ops::Deref;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 6380049 v1.00`**
///
/// This metadata element is defined by the caDSR as "A unique subject
/// identifier within a site and a study.". No permissible values are defined
/// for this CDE.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6380049%20and%20ver_nr=1>
#[derive(
    Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema, Introspect,
)]
#[schema(as = cde::v1::subject::Name, example = "SubjectName001")]
pub struct Name(String);

impl Name {
    /// Creates a new [`Name`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::subject::Name;
    ///
    /// let name = Name::new("Name");
    /// assert_eq!(name.as_str(), "Name");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CDE for Name {}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::subject::Name;

    #[test]
    fn it_displays_correctly() {
        let name = Name::new("Name");
        assert_eq!(name.as_str(), "Name");
    }
}
