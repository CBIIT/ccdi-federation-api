//! Common data elements that have a major version of one and are related to a
//! file.

pub mod checksum;
mod description;
mod identifier;
mod size;
mod r#type;

pub use description::Description;
pub use identifier::Name;
pub use r#type::Type;
pub use size::Size;
