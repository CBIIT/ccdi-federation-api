use ccdi_cde as cde;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use serde::Deserialize;
use serde::Serialize;
use utoipa::schema;
use utoipa::ToSchema;

/// A list of checksums for a file.
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema,
)]
#[schema(as = models::file::metadata::Checksums)]
pub struct Checksums {
    /// The namespace of the identifier.
    #[schema(example = "example-organization")]
    md5: Option<cde::v1::file::checksum::MD5>,
}

impl Checksums {
    /// Creates a new [`Checksums`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// let checksums = models::file::metadata::Checksums::new(Some(
    ///     cde::v1::file::checksum::MD5::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap(),
    /// ));
    /// ```
    pub fn new(md5: Option<cde::v1::file::checksum::MD5>) -> Self {
        Self { md5 }
    }

    /// Gets the md5 checksum from the [`Checksums`] by reference (if it exists).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// let checksums = models::file::metadata::Checksums::new(Some(
    ///     cde::v1::file::checksum::MD5::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap(),
    /// ));
    ///
    /// assert_eq!(
    ///     checksums.md5().unwrap().inner(),
    ///     "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
    /// );
    /// ```
    pub fn md5(&self) -> Option<&cde::v1::file::checksum::MD5> {
        self.md5.as_ref()
    }
}

impl std::fmt::Display for Checksums {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ md5: {} }}",
            self.md5.as_ref().map(|md5| md5.inner()).unwrap_or("None")
        )
    }
}

impl Distribution<Checksums> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Checksums {
        Checksums {
            md5: Some(rng.gen()),
        }
    }
}
