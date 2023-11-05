//! A crate for encapsulating the an example Childhood Cancer Data Initiative
//! federation API server along with the definitions for the OpenAPI
//! specification.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod filter;
pub mod paginate;
pub mod params;
pub mod responses;
pub mod routes;
