use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Url;

/// A standard to which a field is harmonized.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::metadata::field::description::harmonized::Standard)]
pub struct Standard {
    /// The name.
    name: String,

    /// A link that describes the standard.
    #[schema(value_type = models::Url)]
    url: Url,
}

impl Standard {
    /// Creates a new [`Standard`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::Url;
    ///
    /// let standard = Standard::new(
    ///     String::from("caDSR CDE ------- v1.00"),
    ///     Url::try_from("https://cancer.gov").unwrap(),
    /// );
    ///
    /// assert_eq!(standard.name(), "caDSR CDE ------- v1.00");
    /// assert_eq!(standard.url(), "https://cancer.gov/");
    /// ```
    pub fn new(name: String, url: Url) -> Self {
        Self { name, url }
    }

    /// Gets the name of the [`Standard`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::Url;
    ///
    /// let standard = Standard::new(
    ///     String::from("caDSR CDE ------- v1.00"),
    ///     Url::try_from("https://cancer.gov").unwrap(),
    /// );
    ///
    /// assert_eq!(standard.name(), "caDSR CDE ------- v1.00");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Gets the URL that describes the [`Standard`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::description::harmonized::Standard;
    /// use models::Url;
    ///
    /// let standard = Standard::new(
    ///     String::from("caDSR CDE ------- v1.00"),
    ///     Url::try_from("https://cancer.gov").unwrap(),
    /// );
    ///
    /// assert_eq!(standard.url(), "https://cancer.gov/");
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_ref()
    }
}
