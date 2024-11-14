import requests
import copy
import sys
from datetime import datetime

# Get cde_id, version, name, data_type from command line
if len(sys.argv) == 5:
    cde_id = sys.argv[1]
    cde_version = sys.argv[2]
    cde_version_numeric = cde_version.replace("v", "")
    enum_name = sys.argv[3]
    data_type = sys.argv[4]
else:
    print("Usage: get_cde_permissible_value.py 14808227 v1 LibrarySourceMaterial sample")
    exit()

# CaDSR Variables
cde_deeplink= f"https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID={cde_id}%20and%20ver_nr={cde_version_numeric}"
url = "https://cadsrapi.cancer.gov/rad/NCIAPI/1.0/api/DataElement/" + cde_id

# Templates
enum_value_template = """
    /// `CCDI_TEMPLATE_REPLACE_1`
    ///
    /// * **VM Long Name**: CCDI_TEMPLATE_REPLACE_2
    /// * **VM Public ID**: CCDI_TEMPLATE_REPLACE_3
    /// * **Concept Code**: CCDI_TEMPLATE_REPLACE_4
    /// * **Begin Date**:   CCDI_TEMPLATE_REPLACE_5
    ///
    /// CCDI_TEMPLATE_REPLACE_6
    #[serde(rename = "CCDI_TEMPLATE_REPLACE_1")]
    CCDI_TEMPLATE_REPLACE_7,

"""

enum_display_value_template = """
            CCDI_TEMPLATE_REPLACE_3::CCDI_TEMPLATE_REPLACE_2 => write!(f, "CCDI_TEMPLATE_REPLACE_1"),"""

enum_distrubution_value_template = """
            CCDI_TEMPLATE_REPLACE_1 => CCDI_TEMPLATE_REPLACE_2::CCDI_TEMPLATE_REPLACE_3,"""

enum_template = """
use introspect::Introspect;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// **`caDSR CDE CCDI_TEMPLATE_REPLACE_1 CCDI_TEMPLATE_REPLACE_2`**
///
/// This metadata element is defined by the caDSR as "CCDI_TEMPLATE_REPLACE_3".
///
/// Link:
/// <CCDI_TEMPLATE_REPLACE_4>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema, Introspect)]
#[schema(as = cde::CCDI_TEMPLATE_REPLACE_5::CCDI_TEMPLATE_REPLACE_6::CCDI_TEMPLATE_REPLACE_7)]
pub enum CCDI_TEMPLATE_REPLACE_7 {
CCDI_TEMPLATE_REPLACE_8
}

impl CDE for CCDI_TEMPLATE_REPLACE_7 {}

impl std::fmt::Display for CCDI_TEMPLATE_REPLACE_7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CCDI_TEMPLATE_REPLACE_9
        }
    }
}

impl Distribution<CCDI_TEMPLATE_REPLACE_7> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CCDI_TEMPLATE_REPLACE_7 {
        match rng.gen_range(0..=CCDI_TEMPLATE_REPLACE_01) {
            CCDI_TEMPLATE_REPLACE_02
        }
    }
}
"""


test_convert_to_string_template = """
        assert_eq!(CCDI_TEMPLATE_REPLACE_1::CCDI_TEMPLATE_REPLACE_2.to_string(), "CCDI_TEMPLATE_REPLACE_3");"""

test_convert_to_json_template = """
        assert_eq!(
            serde_json::to_string(&CCDI_TEMPLATE_REPLACE_1::CCDI_TEMPLATE_REPLACE_2).unwrap(),
            "\\\"CCDI_TEMPLATE_REPLACE_3\\\""
        );"""

test_template = """
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_string_correctly() {
        CCDI_TEMPLATE_REPLACE_1
    }

    #[test]
    fn it_serializes_to_json_correctly() {
        CCDI_TEMPLATE_REPLACE_2
    }
}
"""



# Retrieve the data from CaDSR
payload = {}
headers = {
  'accept': 'application/json',
}
response = requests.request("GET", url, headers=headers, data=payload)
# print(response.text)
response = response.json()
# print(response["DataElement"]["ValueDomain"]["PermissibleValues"])
permissible_values = response["DataElement"]["ValueDomain"]["PermissibleValues"]

values = []
display_values = []
distribution_values = []

tests_string = []
tests_json = []

index = 0
for permissible_value in permissible_values:
  # print(permissible_value)
  value = copy.deepcopy(enum_value_template)
  value = value.replace("CCDI_TEMPLATE_REPLACE_1", permissible_value["value"])
  value = value.replace("CCDI_TEMPLATE_REPLACE_2", permissible_value["ValueMeaning"]["longName"])
  value = value.replace("CCDI_TEMPLATE_REPLACE_3",permissible_value["ValueMeaning"]["publicId"])
  # Include conceptCode if present; some permissible values do not have any associated Concepts.
  if len(permissible_value["ValueMeaning"]["Concepts"]):
    value = value.replace("CCDI_TEMPLATE_REPLACE_4", permissible_value["ValueMeaning"]["Concepts"][0]["conceptCode"])
  else:
    value = value.replace("CCDI_TEMPLATE_REPLACE_4","")
  formatted_date = datetime.strptime(permissible_value["ValueMeaning"]["dateModified"], '%Y-%m-%d').strftime('%m/%d/%Y')
  value = value.replace("CCDI_TEMPLATE_REPLACE_5", formatted_date)
  value = value.replace("CCDI_TEMPLATE_REPLACE_6", permissible_value["ValueMeaning"]["definition"])
  # Format the name.
  # Cannot contain -_ or start with a digit
  name = permissible_value["value"]
  for i in ["-", "_"]: name = name.replace(i," ")
  if name[0].isdigit():
    num2words = {'1':'One','2':'Two','3':'Three','4':'Four','5':'Five','6':'Six','7':'Seven','8':'Eight','9':'Nine','0':'Zero'}
    name = num2words[name[0]] + name[1:]
  formatted_name = ''.join(word.capitalize() for word in name.split())
  value = value.replace("CCDI_TEMPLATE_REPLACE_7", formatted_name)
  values.append(value)


  display_value = copy.deepcopy(enum_display_value_template)
  display_value = display_value.replace("CCDI_TEMPLATE_REPLACE_1", permissible_value["value"])
  display_value = display_value.replace("CCDI_TEMPLATE_REPLACE_2", formatted_name)
  display_value = display_value.replace("CCDI_TEMPLATE_REPLACE_3", enum_name)
  display_values.append(display_value)

  distribution_value = copy.deepcopy(enum_distrubution_value_template)
  distribution_value = distribution_value.replace("CCDI_TEMPLATE_REPLACE_1", str(index) if index < len(permissible_values) - 1 else "_")
  distribution_value = distribution_value.replace("CCDI_TEMPLATE_REPLACE_2", enum_name)
  distribution_value = distribution_value.replace("CCDI_TEMPLATE_REPLACE_3", formatted_name)
  distribution_values.append(distribution_value)


  test_string = copy.deepcopy(test_convert_to_string_template)
  test_string = test_string.replace("CCDI_TEMPLATE_REPLACE_1", enum_name)
  test_string = test_string.replace("CCDI_TEMPLATE_REPLACE_2", formatted_name)
  test_string = test_string.replace("CCDI_TEMPLATE_REPLACE_3", permissible_value["value"])
  tests_string.append(test_string)


  test_json = copy.deepcopy(test_convert_to_json_template)
  test_json = test_json.replace("CCDI_TEMPLATE_REPLACE_1", enum_name)
  test_json = test_json.replace("CCDI_TEMPLATE_REPLACE_2", formatted_name)
  test_json = test_json.replace("CCDI_TEMPLATE_REPLACE_3", permissible_value["value"])
  tests_json.append(test_json)

  index = index + 1

combined_values = ''.join(values)
combined_display_values = ''.join(display_values)
combined_distribution_values = ''.join(distribution_values)
# print(combined_distribution_values)

combined_test_json = ''.join(tests_json)
combined_test_string = ''.join(tests_string)

cde_enum = copy.deepcopy(enum_template)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_1", response["DataElement"]["publicId"])
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_2", response["DataElement"]["version"] + ".00")
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_3", response["DataElement"]["definition"])
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_4", cde_deeplink)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_5", cde_version)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_6", data_type)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_7", enum_name)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_8", combined_values)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_9", combined_display_values)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_01", str(len(permissible_values) - 1))
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_02", combined_distribution_values)



test_str = copy.deepcopy(test_template)
test_str = test_str.replace("CCDI_TEMPLATE_REPLACE_1", combined_test_string)
test_str = test_str.replace("CCDI_TEMPLATE_REPLACE_2", combined_test_json)


ccdi_cde_file = cde_enum + test_str
# print(cde_enum)
print(ccdi_cde_file)

