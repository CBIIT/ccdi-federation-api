//! A size for a file.

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11479876 v1.00`**
///
/// This metadata element is defined by the caDSR as "The measure (in bytes) of
/// how much space a data file takes up on a storage medium.". No permissible
/// values are defined for this CDE.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11479876%20and%20ver_nr=1>
#[derive(
    Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema, Introspect,
)]
#[schema(as = cde::v1::file::Size)]
pub struct Size(usize);

impl Size {
    /// Creates a new [`Size`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Size;
    ///
    /// let size = Size::new(42);
    /// ```
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    /// Gets the inner value of the [`Size`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Size;
    ///
    /// let size = Size::new(42);
    /// assert_eq!(size.inner(), 42);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn inner(&self) -> usize {
        self.0
    }
}

impl CDE for Size {}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::file::Size;

    #[test]
    fn it_displays_correctly() {
        let size = Size::new(42);
        assert_eq!(size.to_string(), "42");
    }
}
