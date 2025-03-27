use std::ops::Deref;
use std::ops::DerefMut;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// The associated_diagnoses for a [`Subject`](crate::Subject).
///
/// This value can be any permissible diagnosis in v1.7.2 of the CCDI Submission
/// Template. These values are from the value set **diagnosis_classification**
/// found in the 'Terms and Value Sets' tab from the [CCDI Submission Template
/// v1.7.2].
///
/// To facilitate quick access to these values, we have provided a slimmed down
/// spreadsheet containing the valid diagnoses:
///
/// 1. Download the spreadsheet linked below titled
///    'CCDI_Submission_Template_v1.7.2.diagnosis_values.xlsx'.
/// 2. The permissible values are found in column A of the 'diagnosis' tab,
///    titled **diagnosis_category_term**
///
/// [CCDI Submission Template v1.7.2]: https://github.com/CBIIT/ccdi-model/blob/682a99d93b66540bb880ce5899ba8096968a96cf/metadata-manifest/CCDI_Submission_Template_v1.7.2.xlsx
/// [CCDI_Submission_Template_v1.7.2.diagnosis_values.xlsx]: https://cbiit.github.io/ccdi-federation-api/assets/CCDI_Submission_Template_v1.7.2.diagnosis_values.xlsx
#[derive(
    Clone, Debug, Deserialize, Eq, Introspect, Ord, PartialEq, PartialOrd, Serialize, ToSchema,
)]
#[schema(as = models::subject::metadata::AssociatedDiagnoses)]
pub struct AssociatedDiagnoses(String);

impl From<String> for AssociatedDiagnoses {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for AssociatedDiagnoses {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AssociatedDiagnoses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for AssociatedDiagnoses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
