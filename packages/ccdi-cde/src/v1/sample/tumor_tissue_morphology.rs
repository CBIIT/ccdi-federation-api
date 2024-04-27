use std::collections::HashMap;

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE 11326261 v1.00`**
///
/// This metadata element is defined by the caDSR as "The microscopic anatomy of
/// normal and abnormal cells and tissues of the specimen as captured in the
/// morphology codes of the International Classification of Diseases for
/// Oncology, 3rd Edition (ICD-O-3)."
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11326261%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::v1::sample::TumorTissueMorphology)]
#[skip_serializing_none]
pub struct TumorTissueMorphology {
    /// The ICD-O-3 code.
    icd_o_3: Option<String>,
    // NOTE: if more morphologies are added here, they also need to be added to
    // the `as_map()` function below.
}

impl TumorTissueMorphology {
    /// Creates a new [`TumorTissueMorphology`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// let morphologies = cde::v1::sample::TumorTissueMorphology::new(Some(String::from("8000/0")));
    /// ```
    pub fn new(icd_o_3: Option<String>) -> Self {
        Self { icd_o_3 }
    }

    /// Gets the md5 checksum from the [`TumorTissueMorphology`] by reference (if it exists).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// let morphologies = cde::v1::sample::TumorTissueMorphology::new(Some(String::from("8000/0")));
    /// assert_eq!(morphologies.icd_o_3().unwrap(), "8000/0");
    /// ```
    pub fn icd_o_3(&self) -> Option<&str> {
        self.icd_o_3.as_deref()
    }

    /// Gets the morphologies as a [`HashMap`] where the key is the algorithm name
    /// and the values are the (optional) checksum values.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// let morphologies = cde::v1::sample::TumorTissueMorphology::new(Some(String::from("8000/0")));
    ///
    /// let map = morphologies.as_map();
    /// assert_eq!(map.get("icd_o_3").unwrap().as_str(), "8000/0");
    /// ```
    pub fn as_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if let Some(value) = &self.icd_o_3 {
            map.insert(String::from("icd_o_3"), value.to_string());
        }

        map
    }
}

impl CDE for TumorTissueMorphology {}

impl std::fmt::Display for TumorTissueMorphology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
