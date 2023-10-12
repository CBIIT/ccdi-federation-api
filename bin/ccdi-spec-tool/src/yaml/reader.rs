//! A YAML reader.

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use serde_yaml::Value;

/// An error related to a [`Reader`].
#[derive(Debug)]
pub enum Error {
    /// A file does not exist at the path specified by the [`String`].
    FileDoesNotExist(PathBuf),

    /// An expected key was missing in the inner [`Value`].
    MissingKey(String),

    /// An input/output error.
    IoError(io::Error),

    /// A `serde_yaml` error.
    SerdeYamlError(serde_yaml::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileDoesNotExist(path) => write!(f, "file does not exist: {}", path.display()),
            Error::MissingKey(key) => write!(f, "missing expected key: {key}"),
            Error::IoError(err) => write!(f, "i/o error: {err}"),
            Error::SerdeYamlError(err) => write!(f, "serde yaml error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

/// A [Result](std::result::Result) with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// A YAML reader.
#[derive(Debug)]
pub struct Reader {
    value: Value,
}

impl Reader {
    /// Attempts to create a new [`Reader`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_spec_tool as ccdi;
    /// use ccdi::yaml::Reader;
    ///
    /// // A valid reader.
    ///
    /// let data = b"x: 10\ny: 20\n";
    /// let reader = Reader::try_new(&data[..])?;
    ///
    /// // An invalid reader.
    ///
    /// let data = b"x: 0\ny\n";
    /// let err = Reader::try_new(&data[..]).unwrap_err();
    /// assert_eq!(err.to_string(), "serde yaml error: could not find expected ':' at line 3 column 1, while scanning a simple key at line 2 column 1");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_new<R: Read>(reader: R) -> Result<Self> {
        let value = serde_yaml::from_reader(reader).map_err(Error::SerdeYamlError)?;
        Ok(Self { value })
    }

    /// Gets a key throwing an [`Error::MissingKey`] if it is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_spec_tool as ccdi;
    /// use ccdi::yaml::Reader;
    ///
    /// let data = b"x: 10\ny: 20\n";
    /// let reader = Reader::try_new(&data[..])?;
    ///
    /// let result = reader.ensure_key("x")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn ensure_key(&self, k: &str) -> Result<&Value> {
        self.value
            .get(k)
            .map(Ok)
            .unwrap_or(Err(Error::MissingKey(k.to_owned())))
    }
}

impl TryFrom<&Path> for Reader {
    type Error = Error;

    fn try_from(p: &Path) -> Result<Self> {
        if !p.exists() {
            return Err(Error::FileDoesNotExist(p.to_owned()));
        }

        let file = File::open(p).map_err(Error::IoError)?;

        Self::try_new(file)
    }
}
