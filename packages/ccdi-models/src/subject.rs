//! Representations of subjects.

use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_cde as cde;

use cde::v1::subject::Identifier;

mod kind;
pub mod metadata;

pub use kind::Kind;
pub use metadata::Metadata;

/// A subject.
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::Subject)]
pub struct Subject {
    /// The primary identifier used by the site.
    ///
    /// This identifier should *ALWAYS* be included in the `identifiers` key
    /// under `metadata`, should that key exist.
    #[schema(value_type = cde::v1::subject::Identifier)]
    id: Identifier,

    /// The primary name for a subject used within the source server.
    ///
    /// Note that this field is not namespaced like an `identifier` is, and,
    /// instead, is intended to represent a colloquial or display name for the
    /// sample (without indicating the scope of the name).
    #[schema(example = "SubjectName001")]
    name: String,

    /// The kind of [`Subject`].
    #[schema(value_type = models::subject::Kind)]
    kind: Kind,

    /// Metadata associated with this [`Subject`].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<models::subject::Metadata>)]
    metadata: Option<Metadata>,
}

impl Subject {
    /// Creates a new [`Subject`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::v1::subject::Identifier;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Subject;
    ///
    /// let subject = Subject::new(
    ///     Identifier::parse("organization:Name", ":")?,
    ///     String::from("Name"),
    ///     Kind::Participant,
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(id: Identifier, name: String, kind: Kind, metadata: Option<Metadata>) -> Self {
        Self {
            id,
            name,
            kind,
            metadata,
        }
    }

    /// Gets the primary identifier for this [`Subject`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::v1::subject::Identifier;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Subject;
    ///
    /// let subject = Subject::new(
    ///     Identifier::parse("organization:Name", ":")?,
    ///     String::from("Name"),
    ///     Kind::Participant,
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// assert_eq!(
    ///     subject.id(),
    ///     &Identifier::parse("organization:Name", ":").unwrap()
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn id(&self) -> &Identifier {
        &self.id
    }

    /// Gets the name for this [`Subject`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::v1::subject::Identifier;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Subject;
    ///
    /// let subject = Subject::new(
    ///     Identifier::parse("organization:Name", ":")?,
    ///     String::from("Name"),
    ///     Kind::Participant,
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// assert_eq!(subject.name(), &String::from("Name"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Gets the kind for this [`Subject`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::v1::subject::Identifier;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Subject;
    ///
    /// let subject = Subject::new(
    ///     Identifier::parse("organization:Name", ":")?,
    ///     String::from("Name"),
    ///     Kind::Participant,
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// assert_eq!(subject.kind(), &Kind::Participant);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Gets the metadata for this [`Subject`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use cde::v1::subject::Identifier;
    /// use models::subject::metadata::Builder;
    /// use models::subject::Kind;
    /// use models::Subject;
    ///
    /// let metadata = Builder::default().build();
    ///
    /// let subject = Subject::new(
    ///     Identifier::parse("organization:Name", ":")?,
    ///     String::from("Name"),
    ///     Kind::Participant,
    ///     Some(metadata.clone()),
    /// );
    ///
    /// assert_eq!(subject.metadata(), Some(&metadata));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Generates a random [`Subject`] based on a particular [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::Subject;
    ///
    /// let identifier = cde::v1::subject::Identifier::parse("organization:Name", ":").unwrap();
    /// let subject = Subject::random(identifier);
    /// ```
    pub fn random(identifier: Identifier) -> Self {
        let mut rng = thread_rng();

        Self {
            id: identifier.clone(),
            name: identifier.name().clone(),
            kind: rand::random(),
            metadata: match rng.gen_bool(0.7) {
                true => Some(Metadata::random(identifier)),
                false => None,
            },
        }
    }
}
