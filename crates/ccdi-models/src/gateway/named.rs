//! Named gateways.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Gateway;

/// A named gateway.
///
/// A named gateway is simply a [`Gateway`] with a name. Named gateways exist so
/// that multiple files in a response can refer to the same gateway (by name)
/// without duplicating the information for that gateway multiple times.
///
/// **Note:** a _named_ gateway can only be included in a `gateways` response
/// object—they cannot be embedded directly within a [`File`](crate::File) in
/// the response.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::gateway::Named)]
pub struct Named {
    /// The name.
    name: String,

    /// The gateway.
    #[serde(flatten)]
    #[schema(value_type = models::Gateway)]
    gateway: Gateway,
}

impl Named {
    /// Creates a new [`Named`] gateway.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::gateway::Link;
    /// use models::gateway::Named;
    /// use models::Gateway;
    /// use models::Url;
    ///
    /// let gateway = Gateway::Open {
    ///     link: Link::Direct {
    ///         url: "https://example.com".parse::<Url>().unwrap(),
    ///     },
    /// };
    ///
    /// let named = Named::new(String::from("name"), gateway);
    /// assert_eq!(named.name(), "name");
    /// assert!(matches!(named.gateway(), Gateway::Open { .. }));
    /// ```
    pub fn new(name: String, gateway: Gateway) -> Self {
        Self { name, gateway }
    }

    /// Gets the name for the [`Named`] gateway by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::gateway::Link;
    /// use models::gateway::Named;
    /// use models::Gateway;
    /// use models::Url;
    ///
    /// let gateway = Gateway::Open {
    ///     link: Link::Direct {
    ///         url: "https://example.com".parse::<Url>().unwrap(),
    ///     },
    /// };
    ///
    /// let named = Named::new(String::from("name"), gateway);
    /// assert_eq!(named.name(), "name");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Gets the [`Gateway`] for the [`Named`] gateway by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::gateway::Link;
    /// use models::gateway::Named;
    /// use models::Gateway;
    /// use models::Url;
    ///
    /// let gateway = Gateway::Open {
    ///     link: Link::Direct {
    ///         url: "https://example.com".parse::<Url>().unwrap(),
    ///     },
    /// };
    ///
    /// let named = Named::new(String::from("name"), gateway);
    /// assert!(matches!(named.gateway(), Gateway::Open { .. }));
    /// ```
    pub fn gateway(&self) -> &Gateway {
        &self.gateway
    }
}
