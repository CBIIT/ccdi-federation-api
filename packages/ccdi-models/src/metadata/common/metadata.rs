//! Common metadata elements.

use nonempty::NonEmpty;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod builder;

pub use builder::Builder;

use crate::metadata::common::deposition::Accession;

/// Metadata that is common to all metadata blocks.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::metadata::common::Metadata)]
pub struct Metadata {
    /// Statements of deposition to public repositories for a given entity.
    ///
    /// **NOTE**: when you declare that a dataset has been deposited to a public
    /// repository such as dbGaP or EGA, you should also include a gateway and
    /// link pointing to where that entity can be found in the public
    /// repository.
    #[schema(value_type = Vec<models::metadata::common::deposition::Accession>)]
    depositions: Option<NonEmpty<Accession>>,
}

impl Metadata {
    /// The deposition declarations for this [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::common::metadata;
    /// ```
    pub fn depositions(&self) -> Option<&NonEmpty<Accession>> {
        self.depositions.as_ref()
    }
}
