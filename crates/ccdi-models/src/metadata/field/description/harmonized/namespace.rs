//! Harmonized metadata field descriptions for namespaces.

use ccdi_cde as cde;
use introspect::Entity;
use introspect::IntrospectedEntity as _;

use crate::metadata::field::description;
use crate::metadata::field::description::harmonized::Kind;
use crate::metadata::field::description::r#trait::Description as _;
use crate::metadata::field::description::Harmonized;
use crate::Url;

/// Gets the harmonized fields for samples.
pub fn get_field_descriptions() -> Vec<description::Description> {
    vec![
        cde::v1::namespace::StudyShortTitle::description(),
        cde::v1::namespace::StudyId::description(),
        cde::v1::namespace::StudyName::description(),
        cde::v1::namespace::StudyFundingId::description(),
    ]
}

impl description::r#trait::Description for cde::v1::namespace::StudyShortTitle {
    fn description() -> description::Description {
        let description = match Self::introspected_entity() {
            Entity::Enum(entity) => entity.documentation().unwrap().to_string(),
            Entity::Struct(entity) => entity.documentation().unwrap().to_string(),
        };

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("study_short_title"),
            description,
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Namespace-Metadata-Fields#study_short_title".parse::<Url>().unwrap(),
            None,
            None,
        ))
    }
}

impl description::r#trait::Description for cde::v1::namespace::StudyId {
    fn description() -> description::Description {
        let description = match Self::introspected_entity() {
            Entity::Enum(entity) => entity.documentation().unwrap().to_string(),
            Entity::Struct(entity) => entity.documentation().unwrap().to_string(),
        };

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("study_id"),
            description,
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Namespace-Metadata-Fields#study_id"
                .parse::<Url>()
                .unwrap(),
            None,
            None,
        ))
    }
}

impl description::r#trait::Description for cde::v1::namespace::StudyName {
    fn description() -> description::Description {
        let description = match Self::introspected_entity() {
            Entity::Enum(entity) => entity.documentation().unwrap().to_string(),
            Entity::Struct(entity) => entity.documentation().unwrap().to_string(),
        };

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("study_name"),
            description,
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Namespace-Metadata-Fields#study_name"
                .parse::<Url>()
                .unwrap(),
            None,
            None,
        ))
    }
}

impl description::r#trait::Description for cde::v1::namespace::StudyFundingId {
    fn description() -> description::Description {
        let description = match Self::introspected_entity() {
            Entity::Enum(entity) => entity.documentation().unwrap().to_string(),
            Entity::Struct(entity) => entity.documentation().unwrap().to_string(),
        };

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("study_funding_id"),
            description,
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Namespace-Metadata-Fields#study_funding_id"
                .parse::<Url>()
                .unwrap(),
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
