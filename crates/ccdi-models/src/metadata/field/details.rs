//! Details pertaining to a specific harmonized value.

mod harmonizer;
mod method;

pub use harmonizer::Harmonizer;
pub use method::Method;

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Url;

/// Details regarding the harmonization process.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::metadata::field::Details)]
pub struct Details {
    /// The method by which the data was harmonized.
    #[schema(value_type = Option<models::metadata::field::details::Method>)]
    method: Option<Method>,

    /// The type of individual (or individuals) that harmonized the data.
    #[schema(value_type = Option<models::metadata::field::details::Harmonizer>)]
    harmonizer: Option<Harmonizer>,

    /// An optional URL at which you can learn more specific details about the
    /// considerations for this harmonized value.
    #[schema(value_type = Option<models::Url>)]
    url: Option<Url>,
}

impl Details {
    /// Creates a new [`Details`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::details::Harmonizer;
    /// use models::metadata::field::details::Method;
    /// use models::metadata::field::Details;
    /// use models::Url;
    ///
    /// let details = Details::new(
    ///     Some(Method::Mapped),
    ///     Some(Harmonizer::DomainExpert),
    ///     Some(Url::from(
    ///         url::Url::try_from("https://hello.world/").unwrap(),
    ///     )),
    /// );
    ///
    /// assert_eq!(details.method(), Some(&Method::Mapped));
    /// assert_eq!(details.harmonizer(), Some(&Harmonizer::DomainExpert));
    /// assert_eq!(details.url().unwrap().as_str(), "https://hello.world/");
    /// ```
    pub fn new(method: Option<Method>, harmonizer: Option<Harmonizer>, url: Option<Url>) -> Self {
        Self {
            method,
            harmonizer,
            url,
        }
    }

    /// Gets the [`Method`] from the [`Details`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::details::Harmonizer;
    /// use models::metadata::field::details::Method;
    /// use models::metadata::field::Details;
    /// use models::Url;
    ///
    /// let details = Details::new(
    ///     Some(Method::Mapped),
    ///     Some(Harmonizer::DomainExpert),
    ///     Some(Url::from(
    ///         url::Url::try_from("https://hello.world/").unwrap(),
    ///     )),
    /// );
    ///
    /// assert_eq!(details.method(), Some(&Method::Mapped));
    /// ```
    pub fn method(&self) -> Option<&Method> {
        self.method.as_ref()
    }

    /// Gets the [`Harmonizer`] from the [`Details`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::details::Harmonizer;
    /// use models::metadata::field::details::Method;
    /// use models::metadata::field::Details;
    /// use models::Url;
    ///
    /// let details = Details::new(
    ///     Some(Method::Mapped),
    ///     Some(Harmonizer::DomainExpert),
    ///     Some(Url::from(
    ///         url::Url::try_from("https://hello.world/").unwrap(),
    ///     )),
    /// );
    ///
    /// assert_eq!(details.harmonizer(), Some(&Harmonizer::DomainExpert));
    /// ```
    pub fn harmonizer(&self) -> Option<&Harmonizer> {
        self.harmonizer.as_ref()
    }

    /// Gets the [`Harmonizer`] from the [`Details`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::details::Harmonizer;
    /// use models::metadata::field::details::Method;
    /// use models::metadata::field::Details;
    /// use models::Url;
    ///
    /// let details = Details::new(
    ///     Some(Method::Mapped),
    ///     Some(Harmonizer::DomainExpert),
    ///     Some(Url::from(
    ///         url::Url::try_from("https://hello.world/").unwrap(),
    ///     )),
    /// );
    ///
    /// assert_eq!(details.url().unwrap().as_str(), "https://hello.world/");
    /// ```
    pub fn url(&self) -> Option<&Url> {
        self.url.as_ref()
    }
}
