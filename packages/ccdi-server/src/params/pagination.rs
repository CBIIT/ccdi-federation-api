//! Parameters related to pagination.

use serde::Deserialize;
use serde::Serialize;
use utoipa::IntoParams;

/// The default page number if no `page` parameter is provided. By default,
/// pagination is not enabled in queries. If the user enables pagination—say, by
/// providing the `per_page` parameter—but they don't specify a page, this value
/// will be used as the default.
///
/// To be explicit, the true default value for `page` is actually `None`.
pub const DEFAULT_PAGE: usize = 1;

/// The default number of entities per page if no `per_page` parameter is
/// provided. By default, pagination is not enabled in queries. If the user
/// enables pagination—say, by providing the `page` parameter—but they don't
/// specify the number of entities per page, this value will be used as the
/// default.
///
/// To be explicit, the true default value for `per_page` is actually `None`.
pub const DEFAULT_PER_PAGE: usize = 100;

/// Optional parameters for a paginated request to the server.
///
/// Pagination is enabled if any pagination parameters are provided as query
/// parameters to an endpoint that supports pagination.
#[derive(Debug, Default, Deserialize, IntoParams, Serialize)]
#[into_params(parameter_in = Query)]
pub struct Pagination {
    /// The page to retrieve.
    ///
    /// This is a 1-based index of a page within a page set. The value of `page`
    /// **must** default to `1` when pagination is enabled but this parameter is
    /// not provided. Pagination is enabled if this parameter is provided to the
    /// endpoint.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[param(nullable = false)]
    page: Option<Option<usize>>,

    /// The number of results per page.
    ///
    /// Each server can select its own default value for `per_page` if
    /// pagination is enabled but this parameter is not provided. A default
    /// value of `100` is recommended if all values are equally reasonable.
    /// Pagination is enabled if this parameter is provided to the endpoint.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[param(nullable = false)]
    per_page: Option<Option<usize>>,
}

impl Pagination {
    /// Gets the page number from the [`Pagination`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// let params = server::params::Pagination::default();
    /// assert_eq!(params.page(), None);
    /// ```
    pub fn page(&self) -> Option<Option<usize>> {
        self.page
    }

    /// Gets the page number from the [`Pagination`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// let params = server::params::Pagination::default();
    /// assert_eq!(params.per_page(), None);
    /// ```
    pub fn per_page(&self) -> Option<Option<usize>> {
        self.per_page
    }
}
