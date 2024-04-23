use std::ops::Deref;
use std::ops::DerefMut;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11524544 v1.00`**
///
/// This metadata element is defined by the caDSR as "A stable unique
/// alphanumeric identifier assigned to a study and any objects by the database
/// of Genotypes and Phenotypes (dbGaP).".
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11524544%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::deposition::DbgapPhsAccession)]
pub struct DbgapPhsAccession(String);

impl From<String> for DbgapPhsAccession {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for DbgapPhsAccession {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbgapPhsAccession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CDE for DbgapPhsAccession {}

impl std::fmt::Display for DbgapPhsAccession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
