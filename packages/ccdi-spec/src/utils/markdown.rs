use std::slice::Iter;

use ccdi_cde as cde;
use ccdi_models as models;

use cde::parse::cde::Member;
use models::metadata::field::description;
use models::metadata::field::description::harmonized::Kind;
use models::metadata::field::description::Description;

const METADATA_TABLE_FIELDS: &[&str] =
    &["VM Long Name", "VM Public ID", "Concept Code", "Begin Date"];

pub struct Section(Description);

impl From<Description> for Section {
    fn from(value: Description) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Description::Harmonized(description) => display_harmonized(f, description),
            // SAFETY: this command is concerned with printing out the wiki
            // entries for the _harmonized_ data elements. As such, we should
            // never be printing out information for _unharmonized_ data
            // elements (the documentation for those are handled by each
            // individual site).
            Description::Unharmonized(_) => unreachable!(),
        }
    }
}

fn display_harmonized(
    f: &mut std::fmt::Formatter<'_>,
    harmonized: &description::Harmonized,
) -> std::fmt::Result {
    // Write the path to the metadata element in the response.
    writeln!(f, "### **`{}`**\n", harmonized.path())?;

    // Write the header line for the metadata element.
    if let Some(standard) = harmonized.standard() {
        writeln!(
            f,
            "**Formal Name: `{}`** ([Link]({}))\n",
            standard.name(),
            standard.url()
        )?;
    }

    // Write the documentation for the metadata element.
    writeln!(f, "{}\n", harmonized.description())?;

    if let Some(members) = harmonized.members() {
        match harmonized.kind() {
            Kind::Enum => write_variant_members(f, members.iter())?,
            Kind::Struct => write_field_members(f, members.iter())?,
        }
    }

    Ok(())
}

fn write_field_members(
    f: &mut std::fmt::Formatter<'_>,
    members: Iter<'_, (String, Member)>,
) -> std::fmt::Result {
    for (identifier, member) in members {
        let field = match member {
            Member::Field(field) => field,
            // SAFETY: this function is only called when we check that the first
            // member in the `members` list is a [`Member::Field`]. If the first
            // element is a [`Member::Field`], then all of them should be.
            _ => unreachable!(),
        };

        writeln!(f, "* **{}.** {}", identifier, field.description())?;
    }

    Ok(())
}

fn write_variant_members(
    f: &mut std::fmt::Formatter<'_>,
    members: Iter<'_, (String, Member)>,
) -> std::fmt::Result {
    // Write table header.
    write!(f, "| Permissible Value | Description |")?;

    for field in METADATA_TABLE_FIELDS {
        write!(f, " {} |", field)?;
    }

    writeln!(f)?;

    // Write the alignment line.
    write!(f, "|:-- | -- |")?;

    for _ in METADATA_TABLE_FIELDS {
        write!(f, " -- |")?;
    }

    writeln!(f)?;

    for (_, member) in members {
        let variant = match member {
            Member::Variant(variant) => variant,
            // SAFETY: this function is only called when we check that the first
            // member in the `members` list is a [`Member::Variant`]. If the
            // first element is a [`Member::Variant`], then all of them should
            // be.
            v => unreachable!("{:?}", v),
        };

        // Write the row.
        write!(
            f,
            "| `{}` | {} |",
            variant.permissible_value(),
            variant.description()
        )?;

        for field in METADATA_TABLE_FIELDS {
            let key = field.to_string();
            let value = variant
                .metadata()
                .map(|metadata| {
                    metadata
                        .get(&key)
                        .map(|value| value.to_string())
                        .unwrap_or(String::new())
                })
                .unwrap_or(String::new());

            write!(f, " {} |", value)?;
        }

        writeln!(f)?;
    }

    Ok(())
}
