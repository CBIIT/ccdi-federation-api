//! Common data elements that have a major version of two and are related to a
//! sample.

mod library_selection_method;
mod preservation_method;
mod tissue_type;

pub use library_selection_method::LibrarySelectionMethod;
pub use preservation_method::PreservationMethod;
pub use tissue_type::TissueType;
