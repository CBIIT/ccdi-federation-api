//! Common data elements that have a major version of one and are related to a
//! subject.

mod age_at_vital_status;
pub mod name;
mod race;
mod sex;
mod vital_status;

pub use age_at_vital_status::AgeAtVitalStatus;
pub use name::Name;
pub use race::Race;
pub use sex::Sex;
pub use vital_status::VitalStatus;
