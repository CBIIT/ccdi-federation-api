//! A metadata field.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod description;
pub mod owned;
pub mod unowned;

pub use description::Description;

pub use owned::Identifier;
pub use unowned::Ethnicity;
pub use unowned::Race;
pub use unowned::Sex;

/// A metadata field.
#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, ToSchema)]
#[serde(untagged)]
pub enum Field {
    /// An owned field.
    Owned(owned::Field),

    /// An unowned field.
    Unowned(unowned::Field),
}

macro_rules! unowned_field_or_null {
    ($name: ident, $as: ty, $inner: ty) => {
        /// An unowned metadata field or null.
        #[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, ToSchema)]
        #[schema(as = $as)]
        #[serde(untagged)]
        pub enum $name {
            /// An unowned value representing the field.
            #[schema(value_type = field::$inner)]
            Unowned($inner),

            /// A null field.
            Null,
        }
    };
}

unowned_field_or_null!(SexOrNull, field::SexOrNull, Sex);
unowned_field_or_null!(EthnicityOrNull, field::EthnicityOrNull, Ethnicity);

macro_rules! multiple_unowned_fields_or_null {
    ($name: ident, $as: ty, $inner: ty) => {
        /// Multiple unowned metadata fields or null.
        #[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, ToSchema)]
        #[schema(as = $as)]
        #[serde(untagged)]
        pub enum $name {
            /// Multiple unowned values representing the field(s).
            #[schema(value_type = Vec<field::$inner>)]
            MultipleUnowned(Vec<$inner>),

            /// A null field.
            Null,
        }
    };
}

multiple_unowned_fields_or_null!(RacesOrNull, field::RacesOrNull, Race);

#[allow(unused_macros)]
macro_rules! owned_field_or_null {
    ($name: ident, $as: ty, $inner: ty) => {
        /// An owned metadata field.
        #[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, ToSchema)]
        #[schema(as = $as)]
        #[serde(untagged)]
        pub enum $name {
            /// An owned value representing the field.
            #[schema(value_type = field::$inner)]
            Owned($inner),

            /// A null field.
            Null,
        }
    };
}

macro_rules! multiple_owned_fields_or_null {
    ($name: ident, $as: ty, $inner: ty) => {
        /// Multiple owned metadata fields or null.
        #[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, ToSchema)]
        #[schema(as = $as)]
        #[serde(untagged)]
        pub enum $name {
            /// Multiple owned values representing the field(s).
            #[schema(value_type = Vec<field::$inner>)]
            MultipleOwned(Vec<$inner>),

            /// A null field.
            Null,
        }
    };
}

multiple_owned_fields_or_null!(IdentifiersOrNull, field::IdentifiersOrNull, Identifier);

#[cfg(test)]
mod tests {
    use serde_test::assert_tokens;
    use serde_test::Token;

    use super::*;

    #[test]
    fn it_serializes_null_correctly() {
        let field = SexOrNull::Null;

        assert_tokens(&field, &[Token::Unit]);
        assert_eq!(serde_json::to_string(&field).unwrap(), "null");
    }
}
