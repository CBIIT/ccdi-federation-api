//! Harmonized metadata field descriptions for subjects.

use ccdi_cde as cde;

use cde::CDE;
use introspect::Entity;
use introspect::IntrospectedEntity as _;

use crate::metadata::field::description;
use crate::Url;

use crate::metadata::field::description::harmonized::Kind;
use crate::metadata::field::description::harmonized::Standard;
use crate::metadata::field::description::r#trait::Description;
use crate::metadata::field::description::Harmonized;

/// Gets the harmonized fields for subjects.
pub fn get_field_descriptions() -> Vec<description::Description> {
    vec![
        cde::v1::subject::Sex::description(),
        cde::v1::subject::Race::description(),
        cde::v2::subject::Ethnicity::description(),
        cde::v1::subject::Name::description(),
        cde::v1::subject::VitalStatus::description(),
        crate::subject::metadata::AgeAtVitalStatus::description(),
    ]
}

impl Description for cde::v1::subject::Sex {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Enum,
            String::from("sex"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#sex"
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

impl Description for cde::v1::subject::Race {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Enum,
            String::from("race"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#race"
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

impl Description for cde::v2::subject::Ethnicity {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Enum,
            String::from("ethnicity"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#ethnicity"
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

impl Description for cde::v1::subject::Name {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("identifiers"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#identifiers"
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

impl Description for cde::v1::subject::VitalStatus {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|member| member.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Enum,
            String::from("vital_status"),
            entity.description().to_string(),
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#vital_status".parse::<Url>().unwrap(),
            Some(Standard::new(entity.standard_name().to_string(), crate::Url::from(entity.standard_url().clone()))),
            members,
        ))
    }
}

impl description::r#trait::Description for crate::subject::metadata::AgeAtVitalStatus {
    fn description() -> description::Description {
        let description = match Self::introspected_entity() {
            Entity::Enum(entity) => entity.documentation().unwrap().to_string(),
            Entity::Struct(entity) => entity.documentation().unwrap().to_string(),
        };

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("age_at_vital_status"),
            description,
            "https://github.com/CBIIT/ccdi-federation-api/wiki/Subject-Metadata-Fields#age_at_vital_status".parse::<Url>().unwrap(),
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
