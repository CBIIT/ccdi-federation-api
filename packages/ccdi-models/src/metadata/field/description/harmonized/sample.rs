//! Harmonized metadata field descriptions for samples.

use ccdi_cde as cde;

use cde::CDE;
use introspect::Entity;
use introspect::IntrospectedEntity as _;

use crate::metadata::field::description;
use crate::metadata::field::description::harmonized::Kind;
use crate::metadata::field::description::harmonized::Standard;
use crate::metadata::field::description::r#trait::Description as _;
use crate::metadata::field::description::Harmonized;
use crate::Url;

/// Gets the harmonized fields for samples.
pub fn get_field_descriptions() -> Vec<description::Description> {
    vec![
        crate::sample::metadata::AgeAtDiagnosis::description(),
        cde::v1::sample::DiseasePhase::description(),
        cde::v1::sample::LibraryStrategy::description(),
        cde::v2::sample::TissueType::description(),
        cde::v1::sample::TumorClassification::description(),
        cde::v1::sample::TumorTissueMorphology::description(),
        crate::sample::metadata::AgeAtCollection::description(),
    ]
}

impl description::r#trait::Description for crate::sample::metadata::AgeAtDiagnosis {
    fn description() -> description::Description {
        let description = match Self::introspected_entity() {
            Entity::Enum(entity) => entity.documentation().unwrap().to_string(),
            Entity::Struct(entity) => entity.documentation().unwrap().to_string(),
        };

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("age_at_diagnosis"),
            description,
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#age_at_diagnosis".parse::<Url>().unwrap(),
            None,
            None,
        ))
    }
}

impl description::r#trait::Description for cde::v1::sample::DiseasePhase {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Enum,
            String::from("disease_phase"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#disease_phase".parse::<Url>().unwrap(),
            Some(Standard::new(entity.standard_name().to_string(), crate::Url::from(entity.standard_url().clone()))),
            members,
        ))
    }
}

impl description::r#trait::Description for cde::v1::sample::LibraryStrategy {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Enum,
            String::from("library_strategy"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#library_strategy"
                .parse::<Url>().unwrap(),
            Some(Standard::new(
                entity.standard_name().to_string(),
                crate::Url::from(entity.standard_url().clone()),
            )),
            members,
        ))
    }
}

impl description::r#trait::Description for cde::v2::sample::TissueType {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Enum,
            String::from("tissue_type"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#tissue_type"
                .parse::<Url>()
                .unwrap(),
            Some(Standard::new(
                entity.standard_name().to_string(),
                crate::Url::from(entity.standard_url().clone()),
            )),
            members,
        ))
    }
}

impl description::r#trait::Description for cde::v1::sample::TumorClassification {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Enum,
            String::from("tumor_classification"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#tumor_classification".parse::<Url>().unwrap(),
            Some(Standard::new(entity.standard_name().to_string(), crate::Url::from(entity.standard_url().clone()))),
            members,
        ))
    }
}

impl description::r#trait::Description for cde::v1::sample::TumorTissueMorphology {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("tumor_tissue_morphology"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#tumor_tissue_morphology".parse::<Url>().unwrap(),
            Some(Standard::new(entity.standard_name().to_string(), crate::Url::from(entity.standard_url().clone()))),
            members,
        ))
    }
}

impl description::r#trait::Description for crate::sample::metadata::AgeAtCollection {
    fn description() -> description::Description {
        let description = match Self::introspected_entity() {
            Entity::Enum(entity) => entity.documentation().unwrap().to_string(),
            Entity::Struct(entity) => entity.documentation().unwrap().to_string(),
        };

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("age_at_collection"),
            description,
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Sample-Metadata-Fields#age_at_collection".parse::<Url>().unwrap(),
            None,
            None,
        ))
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::metadata::field::description::Description;
    use crate::HARMONIZED_KEY_REGEX;

    use super::*;

    #[test]
    fn all_of_the_descriptions_unwrap() {
        // Because each of the field descriptions are instantiated when this
        // array is constructed, we can simply use this function to test that
        // all of them unwrap successfully.
        get_field_descriptions();
    }

    #[test]
    fn all_of_the_harmonized_keys_conform_to_the_harmonized_key_regex() {
        let regex = Regex::new(HARMONIZED_KEY_REGEX).unwrap();

        for field in get_field_descriptions() {
            let path = match field {
                Description::Harmonized(description) => description.path,
                // This test is only concerned with harmonized fields, so we can
                // skip non-harmonized ones.
                Description::Unharmonized(_) => continue,
            };

            assert!(regex.is_match(path.as_str()))
        }
    }
}
