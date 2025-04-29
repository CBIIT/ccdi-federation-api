//! Common data elements that have a major version of one and are related to a
//! sample.

mod disease_phase;
mod library_source_material;
mod library_strategy;
mod specimen_molecular_analyte_type;
mod tissue_type;
mod tumor_classification;
mod tumor_tissue_morphology;

pub use disease_phase::DiseasePhase;
pub use library_source_material::LibrarySourceMaterial;
pub use library_strategy::LibraryStrategy;
pub use specimen_molecular_analyte_type::SpecimenMolecularAnalyteType;
pub use tissue_type::TissueType;
pub use tumor_classification::TumorClassification;
pub use tumor_tissue_morphology::TumorTissueMorphology;
