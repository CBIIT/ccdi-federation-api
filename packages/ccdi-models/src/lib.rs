//! A crate containing data models for common objects used within the Childhood
//! Cancer Data Initiative federated API.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(const_trait_impl)]
#![feature(decl_macro)]
#![feature(trivial_bounds)]

/// A marker trait for queriable entities within this API.
pub trait Entity {}

pub mod metadata;
pub mod sample;
pub mod subject;

pub use sample::Sample;
pub use subject::Subject;

/// The regex that all harmonized keys must conform to.
///
/// We accept non-empty string comprised of any lowercase character (`[a-z]`)
/// and an underscore.
pub const HARMONIZED_KEY_REGEX: &str = r"^[a-z_]+$";

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn the_harmonized_key_regex_matches_valid_key_names() {
        let regex = Regex::new(HARMONIZED_KEY_REGEX).unwrap();
        assert!(regex.is_match("hello_world"));
        assert!(regex.is_match("a"));
    }

    #[test]
    fn the_harmonized_key_regex_does_not_match_an_empty_key() {
        let regex = Regex::new(HARMONIZED_KEY_REGEX).unwrap();
        assert!(!regex.is_match(""));
    }

    #[test]
    fn the_harmonized_key_regex_does_not_match_invalid_keys() {
        let regex = Regex::new(HARMONIZED_KEY_REGEX).unwrap();
        assert!(!regex.is_match("HeLlOwOrLd"));
        assert!(!regex.is_match("key "));
    }
}
