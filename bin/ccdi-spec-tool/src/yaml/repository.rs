//! A repository for storing and manipulating the OpenAPI specification as YAML.

use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::yaml;

const PATHS_KEY: &str = "paths";
const TAGS_KEY: &str = "tags";

/// An error related to a [`Repository`].
#[derive(Debug)]
pub enum Error {
    /// A reader error.
    ReaderError(yaml::reader::Error),

    /// A conflicting key would be inserted into the repository.
    KeyAlreadyPresent(String),

    /// A key that was expected was not found in the repository.
    MissingExpectedKey(String),

    /// A `serde_yaml` error.
    SerdeYamlError(serde_yaml::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReaderError(err) => write!(f, "reader error: {err}"),
            Error::KeyAlreadyPresent(key) => write!(f, "key already present: {key}"),
            Error::MissingExpectedKey(key) => write!(f, "missing expected key: {key}"),
            Error::SerdeYamlError(err) => write!(f, "serde yaml error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

/// A [Result](std::result::Result) with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// A repository for storing and manipulating the OpenAPI specification as YAML.
#[derive(Default, Debug)]
pub struct Repository {
    inner: Mapping,
}

impl Repository {
    /// Gets the inner [`Mapping`] from the [`Repository`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_yaml::Mapping;
    ///
    /// use ccdi_spec_tool as ccdi;
    /// use ccdi::yaml::Repository;
    ///
    /// let mut repository = Repository::default();
    /// assert_eq!(repository.inner(), &Mapping::new());
    /// ```
    pub fn inner(&self) -> &Mapping {
        &self.inner
    }

    /// Consumes the [`Repository`] to return the inner [`Mapping`].
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_yaml::Mapping;
    ///
    /// use ccdi_spec_tool as ccdi;
    /// use ccdi::yaml::Repository;
    ///
    /// let mut repository = Repository::default();
    /// assert_eq!(repository.into_inner(), Mapping::new());
    /// ```
    pub fn into_inner(self) -> Mapping {
        self.inner
    }

    /// Inserts a [`Value`] into the mapping at the specified `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_yaml::Value;
    ///
    /// use ccdi_spec_tool as ccdi;
    /// use ccdi::yaml::Repository;
    ///
    /// let mut repository = Repository::default();
    /// repository.insert("hello", Value::String(String::from("world")));
    /// ```
    pub fn insert(&mut self, key: &str, value: Value) -> Result<()> {
        if self.inner.contains_key(key) {
            return Err(Error::KeyAlreadyPresent(key.to_owned()));
        }

        // We don't care about whether the key is new or if an old value used to
        // exist in the map, so we don't do anything with this return value.
        self.inner.insert(Value::String(key.to_owned()), value);

        Ok(())
    }

    /// Attempts to retreive the [`Value`] for the given `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_yaml::Value;
    ///
    /// use ccdi_spec_tool as ccdi;
    /// use ccdi::yaml::Repository;
    ///
    /// let mut repository = Repository::default();
    /// repository.insert("hello", Value::String(String::from("world")));
    ///
    /// assert_eq!(
    ///     repository.get("hello"),
    ///     Some(&Value::String(String::from("world")))
    /// );
    /// ```
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    /// Pulls a particular key from the `reader` into this [`Repository`].
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_yaml::Number;
    /// use serde_yaml::Value;
    ///
    /// use ccdi_spec_tool as ccdi;
    /// use ccdi::yaml::Reader;
    /// use ccdi::yaml::Repository;
    ///
    /// let data = b"x: 10\ny: 20\n";
    /// let reader = Reader::try_new(&data[..])?;
    ///
    /// let mut repository = Repository::default();
    /// repository.pull("x", &reader);
    ///
    /// assert_eq!(
    ///     repository.get("x"),
    ///     Some(&Value::Number(Number::from(10)))
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    ///
    /// ```
    pub fn pull(&mut self, key: &str, reader: &yaml::Reader) -> Result<()> {
        self.insert(
            key,
            reader.ensure_key(key).map_err(Error::ReaderError)?.clone(),
        )
    }

    /// Assigns the `tags` key to all [Operation
    /// Objects](https://swagger.io/specification/#operation-object) where the
    /// path is prefixed by `prefix`.
    ///
    /// In other words, this applies the grouping of API calls in the final
    /// OpenAPI specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_spec_tool as ccdi;
    /// use ccdi::yaml::Repository;
    ///
    /// let mut repository = Repository::default();
    ///
    /// let value = serde_yaml::from_str("
    /// /subject:
    ///   get:
    ///     foo: bar
    /// /subject/:id:
    ///   get:
    ///     foo: bar
    /// /sample:
    ///   get:
    ///     foo: bar")?;
    ///
    /// repository.insert("paths", value);
    /// repository.assign_tag_for_path_prefix("/subject", "Subject");
    ///
    /// let result = serde_yaml::to_string(repository.inner())?;
    /// assert_eq!(
    ///     result,
    ///     "paths:
    ///   /subject:
    ///     get:
    ///       foo: bar
    ///       tags:
    ///       - Subject
    ///   /subject/:id:
    ///     get:
    ///       foo: bar
    ///       tags:
    ///       - Subject
    ///   /sample:
    ///     get:
    ///       foo: bar
    /// ");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn assign_tag_for_path_prefix(&mut self, prefix: &str, tag: &str) -> Result<()> {
        let paths = self
            .inner
            .get_mut(PATHS_KEY)
            .map(Ok)
            .unwrap_or(Err(Error::MissingExpectedKey(PATHS_KEY.to_owned())))?;

        for (path, entry) in paths.as_mapping_mut().unwrap() {
            let path = match path {
                Value::String(path) => path,
                _ => continue,
            };

            if !path.starts_with(prefix) {
                continue;
            }

            let methods = match entry {
                Value::Mapping(mapping) => mapping,
                _ => continue,
            };

            for (_, entry) in methods.iter_mut() {
                let entry = match entry {
                    Value::Mapping(entry) => entry,
                    _ => continue,
                };

                entry.insert(
                    Value::String(TAGS_KEY.to_owned()),
                    Value::Sequence(vec![Value::String(tag.to_owned())]),
                );
            }
        }

        Ok(())
    }

    /// Writes the [`Repository`] to the `writer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_yaml::Value;
    ///
    /// use ccdi_spec_tool as ccdi;
    /// use ccdi::yaml::Repository;
    ///
    /// let mut buffer = Vec::new();
    /// let mut repository = Repository::default();
    ///
    /// repository.insert("hello", Value::String(String::from("world")));
    /// repository.insert("abcd", Value::String(String::from("efgh")));
    /// repository.write(&mut buffer);
    ///
    /// let result = std::str::from_utf8(&buffer).unwrap();
    /// assert_eq!(&result, &"hello: world\nabcd: efgh\n");
    /// ```
    pub fn write<W: std::io::Write>(&self, writer: W) -> Result<()> {
        serde_yaml::to_writer(writer, &self.inner).map_err(Error::SerdeYamlError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fails_when_attempting_to_insert_a_key_twice(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut repository = Repository::default();
        repository.insert("hello", Value::String(String::from("world")))?;

        let err = repository
            .insert("hello", Value::String(String::from("world")))
            .unwrap_err();
        assert_eq!(err.to_string(), "key already present: hello");

        Ok(())
    }
}
