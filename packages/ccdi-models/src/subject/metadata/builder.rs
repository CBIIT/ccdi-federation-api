//! A builder for [`Metadata`].

use crate::metadata::field;
use crate::metadata::field::EthnicityOrNull;
use crate::metadata::field::IdentifiersOrNull;
use crate::metadata::field::RacesOrNull;
use crate::metadata::field::SexOrNull;
use crate::subject::Metadata;

/// A builder for [`Metadata`].
#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// The sex of the subject.
    sex: Option<field::unowned::Sex>,

    /// The race of the subject.
    race: Option<Vec<field::unowned::Race>>,

    /// The ethnicity of the subject.
    ethnicity: Option<field::unowned::Ethnicity>,

    /// The identifiers for the subject.
    identifiers: Option<Vec<field::owned::Identifier>>,
}

impl Builder {
    /// Sets the `sex` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::Sex;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Sex::new(cde::v1::Sex::Unknown, None, None);
    /// let builder = Builder::default().sex(field);
    /// ```
    pub fn sex(mut self, sex: field::unowned::Sex) -> Self {
        self.sex = Some(sex);
        self
    }

    /// Append a value to the `race` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::Race;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Race::new(cde::v1::Race::Unknown, None, None);
    /// let builder = Builder::default().append_race(field);
    /// ```
    pub fn append_race(mut self, race: field::unowned::Race) -> Self {
        let mut inner = self.race.unwrap_or_default();
        inner.push(race);

        self.race = Some(inner);

        self
    }

    /// Sets the `ethnicity` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::Ethnicity;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Ethnicity::new(cde::v2::Ethnicity::Unknown, None, None);
    /// let builder = Builder::default().ethnicity(field);
    /// ```
    pub fn ethnicity(mut self, ethnicity: field::unowned::Ethnicity) -> Self {
        self.ethnicity = Some(ethnicity);
        self
    }

    /// Append a value to the `identifier` field of the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::owned::Identifier;
    /// use models::subject::metadata::Builder;
    ///
    /// let field = Identifier::new(
    ///     cde::v1::Identifier::parse("organization:Name", ":")?,
    ///     None,
    ///     None,
    ///     None
    /// );
    ///
    /// let builder = Builder::default().append_identifier(field);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn append_identifier(mut self, identifier: field::owned::Identifier) -> Self {
        let mut inner = self.identifiers.unwrap_or_default();
        inner.push(identifier);

        self.identifiers = Some(inner);

        self
    }

    /// Consumes `self` to build a [`Metadata`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::subject::metadata::Builder;
    ///
    /// let builder = Builder::default();
    /// ```
    pub fn build(self) -> Metadata {
        let sex = self.sex.map(SexOrNull::Unowned).unwrap_or(SexOrNull::Null);

        let race = self
            .race
            .map(RacesOrNull::MultipleUnowned)
            .unwrap_or(RacesOrNull::Null);

        let ethnicity = self
            .ethnicity
            .map(EthnicityOrNull::Unowned)
            .unwrap_or(EthnicityOrNull::Null);

        let identifiers = self
            .identifiers
            .map(IdentifiersOrNull::MultipleOwned)
            .unwrap_or(IdentifiersOrNull::Null);

        Metadata {
            sex,
            race,
            ethnicity,
            identifiers,
        }
    }
}