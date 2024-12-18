//! Metadata fields describing where data has been deposited.

use ccdi_cde::v1::deposition::DbgapPhsAccession;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// An accession of a public repository where the data has been deposited.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(tag = "kind", content = "value")]
#[schema(as = models::metadata::common::deposition::Accession)]
pub enum Accession {
    /// The database of genotypes and phenotypes
    /// <https://www.ncbi.nlm.nih.gov/gap>.
    #[schema(value_type = cde::v1::deposition::DbgapPhsAccession)]
    dbGaP(DbgapPhsAccession),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            serde_json::to_string(&Accession::dbGaP(DbgapPhsAccession::from(String::from(
                "phs000000.v1.p1"
            ))))
            .unwrap(),
            "{\"kind\":\"dbGaP\",\"value\":\"phs000000.v1.p1\"}"
        );
        Ok(())
    }
}
