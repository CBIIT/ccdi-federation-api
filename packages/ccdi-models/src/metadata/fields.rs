//! Collections of metadata fields.

use indexmap::IndexMap;

use crate::metadata::field::Field;

/// A map of unharmonized metadata fields.
///
/// # Examples
///
/// ```
/// use serde_json::Value;
///
/// use ccdi_models as models;
///
/// use models::metadata::field::Field;
/// use models::metadata::field::owned;
/// use models::metadata::field::unowned;
/// use models::metadata::fields::Unharmonized;
///
/// let mut unharmonized = Unharmonized::default();
///
/// unharmonized.insert(
///     String::from("hello"),
///     Field::Unowned(
///         unowned::Field::new(
///             Value::String(String::from("world")),
///             None,
///             None
///         )
///     )
/// );
///
/// unharmonized.insert(
///     String::from("foo"),
///     Field::Owned(
///         owned::Field::new(
///             Value::String(String::from("bar")),
///             None,
///             None,
///             Some(true)
///         )
///     )
/// );
///
/// assert_eq!(
///     serde_json::to_string(&unharmonized)?,
///     "{\"hello\":{\"value\":\"world\"},\"foo\":{\"value\":\"bar\",\"owned\":true}}"
/// );
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub type Unharmonized = IndexMap<String, Field>;
