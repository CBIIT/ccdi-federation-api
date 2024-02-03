use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11280338 v1.00`**
///
/// This metadata element is defined by the caDSR as "A free text field that can
/// be used to document the content and other details about an electronic file
/// that may not be captured elsewhere.". No permissible values are defined for
/// this CDE.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11280338%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::file::Description)]
pub struct Description(String);

impl Description {
    /// Creates a new [`Description`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Description;
    ///
    /// let description = Description::new("Hello, world!");
    /// assert_eq!(description.inner(), "Hello, world!");
    /// ```
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Gets the inner value of the [`Description`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Description;
    ///
    /// let description = Description::new("Hello, world!");
    /// assert_eq!(description.inner(), "Hello, world!");
    /// ```
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Consumes `self` and returns the inner value of the [`Description`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Description;
    ///
    /// let description = Description::new("Hello, world!");
    /// assert_eq!(description.into_inner(), String::from("Hello, world!"));
    /// ```
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl CDE for Description {}

impl std::fmt::Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
