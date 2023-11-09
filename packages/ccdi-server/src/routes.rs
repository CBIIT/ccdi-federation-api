//! Routing.

pub mod info;
pub mod metadata;
pub mod namespace;
pub mod sample;
pub mod subject;

const MISSING_GROUP_BY_KEY: &str = "missing";
const NULL_GROUP_BY_KEY: &str = "null";
