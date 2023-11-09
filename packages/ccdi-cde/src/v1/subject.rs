//! Common data elements that have a major version of one and are related to a
//! subject.

pub mod identifier;
mod race;
mod sex;

pub use identifier::Identifier;
pub use race::Race;
pub use sex::Sex;
