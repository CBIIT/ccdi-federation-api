//! Routing.

pub mod file;
pub mod info;
pub mod metadata;
pub mod namespace;
pub mod organization;
pub mod sample;
pub mod sample_diagnosis;
pub mod subject;
pub mod subject_diagnosis;

/// A result for a group by operation.
#[derive(Debug)]
pub enum GroupByResults<T> {
    /// The key specified to group by is supported.
    Supported(T),

    /// The key specified to group by is _not_ supported.
    Unsupported,
}
