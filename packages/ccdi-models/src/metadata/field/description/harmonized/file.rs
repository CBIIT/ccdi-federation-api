//! Harmonized metadata field descriptions for files.

use ccdi_cde as cde;

use cde::CDE;

use crate::metadata::field::description;
use crate::metadata::field::description::harmonized::Kind;
use crate::metadata::field::description::harmonized::Standard;
use crate::metadata::field::description::r#trait::Description as _;
use crate::metadata::field::description::Harmonized;
use crate::Url;

/// Gets the harmonized fields for files.
pub fn get_field_descriptions() -> Vec<description::Description> {
    vec![
        cde::v1::file::Type::description(),
        cde::v1::file::Size::description(),
        cde::v1::file::checksum::MD5::description(),
        cde::v1::file::Description::description(),
    ]
}

impl description::r#trait::Description for cde::v1::file::Type {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|x| x.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Enum,
            String::from("type"),
            entity.description().to_string(),
            Url::try_from(
                "https://github.com/CBIIT/ccdi-federation-api/wiki/File-Metadata-Fields#type",
            )
            .unwrap(),
            Some(Standard::new(
                entity.standard_name().to_string(),
                crate::Url::from(entity.standard_url().clone()),
            )),
            members,
        ))
    }
}

impl description::r#trait::Description for cde::v1::file::Size {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|x| x.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("size"),
            entity.description().to_string(),
            Url::try_from(
                "https://github.com/CBIIT/ccdi-federation-api/wiki/File-Metadata-Fields#size",
            )
            .unwrap(),
            Some(Standard::new(
                entity.standard_name().to_string(),
                crate::Url::from(entity.standard_url().clone()),
            )),
            members,
        ))
    }
}

impl description::r#trait::Description for cde::v1::file::checksum::MD5 {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|x| x.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("checksums.md5"),
            entity.description().to_string(),
            Url::try_from(
                "https://github.com/CBIIT/ccdi-federation-api/wiki/File-Metadata-Fields#checksumsmd5",
            )
            .unwrap(),
            Some(Standard::new(
                entity.standard_name().to_string(),
                crate::Url::from(entity.standard_url().clone()),
            )),
            members,
        ))
    }
}

impl description::r#trait::Description for cde::v1::file::Description {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().map(|x| x.unwrap());

        description::Description::Harmonized(Harmonized::new(
            Kind::Struct,
            String::from("description"),
            entity.description().to_string(),
            Url::try_from("https://github.com/CBIIT/ccdi-federation-api/wiki/File-Metadata-Fields#description").unwrap(),
            Some(Standard::new(entity.standard_name().to_string(), crate::Url::from(entity.standard_url().clone()))),
            members,
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
