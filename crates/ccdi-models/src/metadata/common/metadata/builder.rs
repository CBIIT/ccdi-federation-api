use nonempty::NonEmpty;

use crate::metadata::common;
use crate::metadata::common::deposition::Accession;

/// A builder for a [`Metadata`](common::Metadata).
#[derive(Debug, Default)]
pub struct Builder {
    /// Statements of deposition to public repositories for a given entity.
    ///
    /// **NOTE**: when you declare that a dataset has been deposited to a public
    /// repository such as dbGaP or EGA, you should also include a gateway and
    /// link pointing to where that entity can be found in the public
    /// repository.
    depositions: Option<NonEmpty<Accession>>,
}

impl Builder {
    /// Adds an [`Accession`] to the deposition declarations for this
    /// [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::v1::deposition::DbgapPhsAccession;
    /// use models::metadata::common::deposition::Accession;
    /// use models::metadata::common::metadata::Builder;
    ///
    /// let accession = Accession::dbGaP(DbgapPhsAccession::from(String::from("phs000000.v1.p1")));
    /// let mut metadata = Builder::default()
    ///     .push_deposition(accession.clone())
    ///     .build();
    ///
    /// assert_eq!(
    ///     metadata.depositions().unwrap().into_iter().next().unwrap(),
    ///     &accession
    /// );
    /// ```
    pub fn push_deposition(mut self, accession: Accession) -> Self {
        let depositions = match self.depositions {
            Some(mut depositions) => {
                depositions.push(accession);
                Some(depositions)
            }
            None => Some(NonEmpty::new(accession)),
        };

        self.depositions = depositions;
        self
    }

    /// Consumes `self` to produce a [`Metadata`](common::Metadata).
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::common::metadata::Builder;
    ///
    /// let builder = Builder::default().build();
    /// ```
    pub fn build(self) -> common::Metadata {
        common::Metadata {
            depositions: self.depositions,
        }
    }
}
