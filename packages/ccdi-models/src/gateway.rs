//! Representations of gateways.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod closed;
mod link;
pub mod named;

pub use closed::Closed;
pub use link::Link;
pub use named::Named;

/// Gateways, which notify of resources that are external to the API.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(tag = "kind")]
#[schema(as = models::Gateway)]
pub enum Gateway {
    /// An open access gateway.
    ///
    /// This gateway notifies of resources that are accessible without any
    /// authentication or authorization.
    Open {
        /// The link.
        #[schema(value_type = models::gateway::Link)]
        link: Link,
    },

    /// A registered access gateway.
    ///
    /// This gateway notifies of resources that are accessible after successful
    /// authentication but _without_ any required authorization.
    ///
    /// The term "registered" is used here instead of "authenticated" because
    /// (a) identification first requires registration of some information that
    /// is used to identify an individual (such as a username or IP address),
    /// and (b) the term "registered" is used more commonly in a colloquial
    /// context.
    Registered {
        /// The link.
        #[schema(value_type = models::gateway::Link)]
        link: Link,
    },

    /// A controlled access gateway.
    ///
    /// This gateway notifies of resources that are accessible after successful
    /// authentication _and_ and explicit authorization to view the resources.
    ///
    /// The term "controlled" is used here instead of "authorized" because (a)
    /// authorization must be explicitly granted by some entity which controls
    /// the resource, and (b) the term "controlled" is used more commonly in a
    /// colloquial context.
    ///
    /// Note that, if authorization is not explicitly granted by some entity
    /// controlling the resource, OR if authorization is given to _all_
    /// authenticated individuals, then a [`Gateway::Registered`] should be used
    /// instead.
    Controlled {
        /// The link.
        #[schema(value_type = models::gateway::Link)]
        link: Link,
    },

    /// A closed access gateway.
    ///
    /// This gateway notifies of resources that are not accessible to consumers
    /// of the API at the time the gateway is constructed.
    ///
    /// Resources that are closed access are not typically returned with the
    /// context of this API (whose purpose is to index _accessible_ data). That
    /// said, some use cases exist where one might want to define a closed
    /// access gateway.
    ///
    /// For example, consider data that is _currently_ closed access but for
    /// which you wish to indicate that the data will become open access in the
    /// future. This may be common in the case of a publication for which you
    /// are sharing access links that will become available once the paper is
    /// published. In these cases, you may couple [`Gateway::Closed`] with a URL
    /// where users can watch for access to become available.
    #[schema(value_type = models::gateway::Closed)]
    Closed(Closed),
}

/// An anonymous [`Gateway`] or a reference to a named [`Gateway`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(tag = "kind")]
#[schema(as = models::gateway::AnonymousOrReference)]
pub enum AnonymousOrReference {
    /// An anonymous gateway.
    Anonymous {
        /// The underlying [`Gateway`].
        #[schema(value_type = models::Gateway)]
        gateway: Gateway,
    },

    /// A reference to a named gateway.
    Reference {
        /// The reference to a [`Named`] gateway.
        gateway: String,
    },
}
