//! An md5 checksum for a file.

use introspect::Introspect;
use lazy_static::lazy_static;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::v1::file::checksum::md5::distribution::Hexadecimal;
use crate::CDE;

mod distribution;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
}

#[derive(Debug)]
pub enum Error {
    /// Attempted to create an invalid md5 checksum.
    Invalid(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Invalid(value) => write!(f, "invalid md5 checksum: {value}"),
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

/// **`caDSR CDE 11556150 v1.00`**
///
/// This metadata element is defined by the caDSR as "A 32-character hexadecimal
/// number that is computed on a file.". No permissible values are defined for
/// this CDE.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11556150%20and%20ver_nr=1>
#[derive(
    Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema, Introspect,
)]
#[schema(as = cde::v1::file::checksum::MD5)]
pub struct MD5(String);

impl MD5 {
    /// Attempts to create a new [`MD5`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::checksum::MD5;
    ///
    /// let md5 = MD5::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    /// ```
    pub fn try_new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if !PATTERN.is_match(&value) {
            return Err(Error::Invalid(value));
        }

        Ok(Self(value))
    }

    /// Gets the inner value of the [`MD5`] (by reference).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::checksum::MD5;
    ///
    /// let md5 = MD5::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    /// assert_eq!(md5.inner(), "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Consumes `self` and returns the inner value of the [`MD5`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::checksum::MD5;
    ///
    /// let md5 = MD5::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    /// assert_eq!(
    ///     md5.into_inner(),
    ///     String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl CDE for MD5 {}

impl Distribution<MD5> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> MD5 {
        MD5::try_new(
            (0..32)
                .map(|_| rng.sample(Hexadecimal) as char)
                .collect::<String>(),
        )
        .unwrap()
    }
}

impl std::fmt::Display for MD5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_standard_pattern_compiles_and_captures() {
        assert!(PATTERN.is_match("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"));
    }

    #[test]
    fn it_fails_to_create_a_non_hex_value() {
        let err = MD5::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA.").unwrap_err();
        assert!(matches!(err, Error::Invalid(_)));
    }

    #[test]
    fn it_fails_to_create_a_hex_value_of_the_wrong_length() {
        let err = MD5::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap_err();
        assert!(matches!(err, Error::Invalid(_)));
    }

    #[test]
    fn it_functions_as_expected() {
        let md5 = MD5::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
        assert_eq!(md5.inner(), "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        assert_eq!(
            md5.clone().into_inner(),
            String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
        );
    }

    #[test]
    fn a_random_string_is_generated_correctly() {
        for _ in 0..1000 {
            let _: MD5 = rand::random();
        }
    }
}
