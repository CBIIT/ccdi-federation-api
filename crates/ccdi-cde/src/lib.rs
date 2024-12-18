//! A crate containing information on the common data elements (CDEs) defined
//! within the Cancer Data Standards Registry and Repository (caDSR) that are
//! used in the Childhood Cancer Data Initiative's federated API.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]

use introspect::Entity;
use introspect::Introspected;
use introspect::Member;

use crate::parse::cde::member;

pub mod parse;
pub mod v1;
pub mod v2;

/// An error related to a [`CDE`].
#[derive(Debug)]
pub enum Error {
    /// The common data element is missing documentation.
    MissingDocumentation,

    /// An error occurred while parsing an entity.
    EntityError(parse::cde::entity::ParseError),

    /// An error occurred while parsing a member of an entity.
    MemberError(parse::cde::member::ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingDocumentation => write!(f, "missing documentation"),
            Error::EntityError(err) => write!(f, "parse entity error: {err}"),
            Error::MemberError(err) => write!(f, "parse member error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// A marker trait for common data elements (CDEs).
pub trait CDE: std::fmt::Display + Eq + PartialEq + Introspected {
    /// Gets the parsed entity information from the corresponding entity's
    /// documentation.
    fn entity() -> Result<parse::cde::Entity> {
        let entity = Self::introspected_entity();

        let documentation = match &entity {
            Entity::Enum(entity) => entity.documentation().to_owned(),
            Entity::Struct(entity) => entity.documentation().to_owned(),
        }
        .map(Ok)
        .unwrap_or(Err(Error::MissingDocumentation))?;

        documentation
            .parse::<parse::cde::Entity>()
            .map_err(Error::EntityError)
    }

    /// Gets the parsed members of an entity from the corresponding member's
    /// documentation.
    #[allow(clippy::type_complexity)]
    fn members() -> Option<Result<Vec<(Option<String>, parse::cde::Member)>>> {
        Self::introspected_members()
            .into_iter()
            .map(|member| match member {
                Member::Field(member) => {
                    member
                        .documentation()
                        .map(|doc| match doc.parse::<member::Field>() {
                            Ok(field) => Ok((
                                member.identifier().map(|identifier| identifier.to_string()),
                                crate::parse::cde::Member::Field(field),
                            )),
                            Err(err) => {
                                Err(Error::MemberError(member::ParseError::FieldError(err)))
                            }
                        })
                }
                Member::Variant(member) => member.documentation().map(|doc| match doc
                    .parse::<member::Variant>()
                {
                    Ok(variant) => Ok((
                        Some(member.identifier().to_string()),
                        crate::parse::cde::Member::Variant(variant),
                    )),
                    Err(err) => Err(Error::MemberError(member::ParseError::VariantError(err))),
                }),
            })
            // .map(|member| member.unwrap_or(Err(Error::MissingDocumentation)))
            .collect::<Option<Result<Vec<_>>>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::subject::Sex;

    use super::*;

    #[test]
    fn entity_parsing_works_correctly() {
        let entity = Sex::entity().unwrap();

        assert_eq!(entity.standard_name(), "caDSR CDE 6343385 v1.00");
    }

    #[test]
    fn member_parsing_works_correctly() {
        let mut entity = Sex::members().unwrap().unwrap().into_iter();

        let (identifer, variant) = entity.next().unwrap();
        assert_eq!(identifer.as_deref(), Some("Unknown"));
        assert_eq!(variant.get_variant().unwrap().permissible_value(), "U");

        let (identifer, variant) = entity.next().unwrap();
        assert_eq!(identifer.as_deref(), Some("Female"));
        assert_eq!(variant.get_variant().unwrap().permissible_value(), "F");

        let (identifer, variant) = entity.next().unwrap();
        assert_eq!(identifer.as_deref(), Some("Male"));
        assert_eq!(variant.get_variant().unwrap().permissible_value(), "M");

        let (identifer, variant) = entity.next().unwrap();
        assert_eq!(identifer.as_deref(), Some("Undifferentiated"));
        assert_eq!(
            variant.get_variant().unwrap().permissible_value(),
            "UNDIFFERENTIATED"
        );
    }
}
