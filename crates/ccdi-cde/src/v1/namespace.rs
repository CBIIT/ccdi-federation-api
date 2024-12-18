//! Common data elements that have a major version of one and are related to a
//! namespace.

mod study_funding_id;
mod study_id;
mod study_name;
mod study_short_title;

pub use study_funding_id::StudyFundingId;
pub use study_id::StudyId;
pub use study_name::StudyName;
pub use study_short_title::StudyShortTitle;
