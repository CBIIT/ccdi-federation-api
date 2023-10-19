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

pub mod count;
pub mod metadata;
pub mod sample;
pub mod subject;

pub use sample::Sample;
pub use subject::Subject;
