//! A crate for encapsulating the [OpenAPI
//! specification](https://swagger.io/specification/).

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]

mod api;

pub use api::Api;
