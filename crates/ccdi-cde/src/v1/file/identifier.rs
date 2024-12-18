//! An name for a file.

use std::ops::Deref;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11284037 v1.00`**
///
/// This metadata element is defined by the caDSR as "The literal label for an
/// electronic data file.". No permissible values are defined for this CDE.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11284037%20and%20ver_nr=1>
#[derive(
    Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema, Introspect,
)]
#[schema(as = cde::v1::file::Name, example = "File001.txt")]
pub struct Name(String);

impl Name {
    /// Creates a new [`Name`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Name;
    ///
    /// let name = Name::new("File001.txt");
    /// assert_eq!(name.as_str(), "File001.txt");
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

#[cfg(test)]
mod tests {
    use crate::v1::file::Name;

    #[test]
    fn it_displays_correctly() {
        let name = Name::new("File001.txt");
        assert_eq!(name.as_str(), "File001.txt");
    }
}
