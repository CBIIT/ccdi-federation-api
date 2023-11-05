//! Collections of metadata fields.

use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::metadata::field;
use crate::metadata::field::UnharmonizedField;

/// A map of unharmonized metadata fields.
///
/// Unharmonized keys may be any valid JSON string.
#[derive(Clone, Default, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = fields::Unharmonized)]
pub struct Unharmonized {
    /// The inner [`IndexMap`].
    #[serde(flatten)]
    inner: IndexMap<String, field::UnharmonizedField>,
}

impl Unharmonized {
    /// Gets a reference to the inner [`IndexMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned;
    /// use models::metadata::field::UnharmonizedField;
    /// use models::metadata::fields::Unharmonized;
    ///
    /// let mut fields = Unharmonized::default();
    /// assert_eq!(fields.inner().len(), 0);
    /// ```
    pub fn inner(&self) -> &IndexMap<String, UnharmonizedField> {
        &self.inner
    }

    /// Gets a mutable reference to the inner [`IndexMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned;
    /// use models::metadata::field::UnharmonizedField;
    /// use models::metadata::fields::Unharmonized;
    ///
    /// let mut fields = Unharmonized::default();
    /// fields.inner_mut().insert(
    ///     "hello".into(),
    ///     UnharmonizedField::Unowned(unowned::Field::new(
    ///         Value::String("world".into()),
    ///         None,
    ///         None,
    ///     )),
    /// );
    /// ```
    pub fn inner_mut(&mut self) -> &mut IndexMap<String, UnharmonizedField> {
        &mut self.inner
    }

    /// Consumes the [`Unharmonized`] and returns the inner [`IndexMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned;
    /// use models::metadata::field::UnharmonizedField;
    /// use models::metadata::fields::Unharmonized;
    ///
    /// let mut fields = Unharmonized::default();
    /// assert_eq!(fields.into_inner().len(), 0);
    /// ```
    pub fn into_inner(self) -> IndexMap<String, UnharmonizedField> {
        self.inner
    }

    /// Returns the number of key-value pairs in the inner [`IndexMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned;
    /// use models::metadata::field::UnharmonizedField;
    /// use models::metadata::fields::Unharmonized;
    ///
    /// let mut fields = Unharmonized::default();
    /// assert_eq!(fields.len(), 0);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether the map contains any elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned;
    /// use models::metadata::field::UnharmonizedField;
    /// use models::metadata::fields::Unharmonized;
    ///
    /// let mut fields = Unharmonized::default();
    /// assert!(fields.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::metadata::field::owned;
    use crate::metadata::field::unowned;

    use super::*;

    #[test]
    fn it_correctly_inserts_owned_and_unowned_fields() -> Result<(), Box<dyn std::error::Error>> {
        let mut unharmonized = Unharmonized::default();

        unharmonized.inner_mut().insert(
            String::from("hello"),
            UnharmonizedField::Unowned(unowned::Field::new(
                Value::String(String::from("world")),
                None,
                None,
            )),
        );

        unharmonized.inner_mut().insert(
            String::from("foo"),
            UnharmonizedField::Owned(owned::Field::new(
                Value::String(String::from("bar")),
                None,
                None,
                Some(true),
            )),
        );

        assert_eq!(
            serde_json::to_string(&unharmonized)?,
            "{\"hello\":{\"value\":\"world\"},\"foo\":{\"value\":\"bar\",\"owned\":true}}"
        );

        Ok(())
    }
}
