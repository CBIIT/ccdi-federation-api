//! A crate containing information on the common data elements (CDEs) defined
//! within the Cancer Data Standards Registry and Repository (caDSR) that are
//! used in the Childhood Cancer Data Initiative's federated API.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod v1;
pub mod v2;

/// A trait to define the harmonization standard used for a common data element.
pub trait Standard {
    /// Gets the harmonization standard name for a common data element.
    fn standard() -> &'static str;
}

/// A trait to define the URL where users can learn more about a common data
/// element.
pub trait Url {
    /// Gets the URL to learn more about a common data element.
    fn url() -> &'static str;
}

/// A marker trait for common data elements (CDEs).
pub trait CDE: std::fmt::Display + Eq + PartialEq + Standard + Url {}
