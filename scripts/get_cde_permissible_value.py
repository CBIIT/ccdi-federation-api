import requests
import copy
import re
from datetime import datetime

# CaDSR Variables
cde_id = "14808227"
cde_deeplink= "https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=14808227%20and%20ver_nr=1"
url = "https://cadsrapi.cancer.gov/rad/NCIAPI/1.0/api/DataElement/" + cde_id

# CCDI Federation variables
enum_name = "LibrarySourceMaterial"
cde_version = "v1"
data_type = "sample"

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
        match rng.gen_range(0..CCDI_TEMPLATE_REPLACE_01) {
            CCDI_TEMPLATE_REPLACE_02
        }
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
index = 0
for permissible_value in permissible_values:
  # print(permissible_value)
  value = copy.deepcopy(enum_value_template)
  value = value.replace("CCDI_TEMPLATE_REPLACE_1", permissible_value["value"])
  value = value.replace("CCDI_TEMPLATE_REPLACE_2", permissible_value["ValueMeaning"]["Concepts"][0]["longName"])
  value = value.replace("CCDI_TEMPLATE_REPLACE_3",permissible_value["ValueMeaning"]["publicId"])
  value = value.replace("CCDI_TEMPLATE_REPLACE_4", permissible_value["ValueMeaning"]["Concepts"][0]["conceptCode"])
  formatted_date = datetime.strptime(permissible_value["ValueMeaning"]["dateModified"], '%Y-%m-%d').strftime('%m/%d/%Y')
  value = value.replace("CCDI_TEMPLATE_REPLACE_5", formatted_date)
  value = value.replace("CCDI_TEMPLATE_REPLACE_6", permissible_value["ValueMeaning"]["Concepts"][0]["definition"])
  formatted_name = ''.join(word.capitalize() for word in permissible_value["value"].split()) 
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

  index = index + 1

combined_values = ''.join(values)
combined_display_values = ''.join(display_values)
combined_distribution_values = ''.join(distribution_values)
# print(combined_distribution_values)




cde_enum = copy.deepcopy(enum_template)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_1", response["DataElement"]["publicId"])
match = re.search(r'v\d+\.\d+$', response["DataElement"]["shortName"])
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_2", match.group())
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_3", response["DataElement"]["definition"])
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_4", cde_deeplink)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_5", cde_version)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_6", data_type)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_7", enum_name)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_8", combined_values)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_9", combined_display_values)
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_01", str(len(permissible_values) - 1))
cde_enum = cde_enum.replace("CCDI_TEMPLATE_REPLACE_02", combined_distribution_values)



print(cde_enum)

