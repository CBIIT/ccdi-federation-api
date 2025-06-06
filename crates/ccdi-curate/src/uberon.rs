pub mod graph;

use std::path::PathBuf;
use std::sync::LazyLock;

use chrono::Utc;
use clap::Parser;
use clap::ValueHint;
use eyre::eyre;
use eyre::Context;
use eyre::Result;
use inflector::Inflector;
use regex::Regex;
use serde_json::Value;
use tracing::info;

/// When the `all-anatomical-sites` feature is not enabled, one entry for every
/// N entries is generated to reduce compile times while developing.
const EVERY_N_ENTRIES: usize = 1000;

/// A **case-sensitive** list of replaced phrases and words. Update this when
/// something isn't compiling correctly in the Rust code directly generated from
/// the script.
const VARIANT_REPLACEMENTS: &[(&str, &str)] = &[
    ("1St", "First"),
    ("2Nd", "Second"),
    ("3Rd", "Third"),
    ("4Th", "Fourth"),
    ("5Th", "Fifth"),
    ("6Th", "Sixth"),
    ("48Cell", "FourEightCell"),
    ("2Cell", "TwoCell"),
    ("4Cell", "FourCell"),
    ("8Cell", "EightCell"),
];

use crate::http;
use crate::uberon::graph::Edge;
use crate::uberon::graph::Node;
use crate::uberon::graph::UberonGraph;

/// The URL for the latest Uberon ontology.
const LATEST_UBERON_URL: &str = "https://purl.obolibrary.org/obo/uberon/uberon-simple.json";

/// The regex to use when hooking the Uberon ontology version.
static UBERON_URL_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https://github.com/obophenotype/uberon/releases/download/(.*)/uberon-simple.json")
        .unwrap()
});

/// Gets the URL for a particular Uberon version.
fn uberon_version_url(version: &str) -> String {
    format!("https://github.com/obophenotype/uberon/releases/download/{version}/uberon-basic.json")
}

////////////////////////////////////////////////////////////////////////////////////////
// Command line tool.
////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// If available locally, the file to load from. This is generally only used
    /// for testing.
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub path: Option<PathBuf>,

    /// If desired, a specific version of the Uberon ontology.
    #[arg(long)]
    pub version: Option<String>,
}

/// Outputs a Rust enum file.
pub fn output_rust_enum(version: String, graph: Vec<Node>) {
    let mut display_clauses = Vec::new();

    println!("//! Anatomical site.");
    println!();
    println!("use introspect::Introspect;");
    println!("use serde::Deserialize;");
    println!("use serde::Serialize;");
    println!("use strum_macros::VariantArray;");
    println!("use utoipa::ToSchema;");
    println!();
    println!("/// Anatomical site for a sample.");
    println!("///");
    println!("/// Anatomical site generally appears in an array of anatomical site values. In");
    println!("/// these cases, you should interpret the assignment of an anatomical site to a");
    println!("/// sample to mean AT LEAST ONE of the anatomical sites applies to the sample.");
    println!("/// Importantly, when the exact anatomical site is not known, but two candidate");
    println!("/// anatomical sites are possible, both sites should be annotated on the sample.");
    println!("///");
    println!("/// The steps that were taken to filter the Uberon ontology to arrive at this");
    println!("/// list are:");
    println!("///");
    println!("/// * Download the Uberon simple ontology (see the file link above).");
    println!("/// * Build a directed graph of `is_a` relationships in the ontology.");
    println!("/// * Include all nodes that fall underneath the `anatomical entity` node.");
    println!("///");
    println!("/// This is an autogenerated entity that represents the all nodes inheriting");
    println!("/// from the 'anatomical entity' node within the simplified Uberon ontology at ");
    println!("/// {version}.");
    println!(
        "/// This enum was generated by the `{} v{}` command line tool on {} (UTC)",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        Utc::now().format("%Y-%m-%d at %H:%M")
    );
    println!("/// from");
    println!("/// [this file]({}).", uberon_version_url(&version));
    #[cfg(not(feature = "all-anatomical-site"))]
    {
        println!("///");
        println!("/// WARNING: this is a debug build of the anatomical site metadata attribute.");
        println!("/// If you are seeing this in production, please file an issue!");
    }
    println!("#[derive(");
    println!("  Clone,");
    println!("  Debug,");
    println!("  Deserialize,");
    println!("  Eq,");
    println!("  Introspect,");
    println!("  Ord,");
    println!("  PartialEq,");
    println!("  PartialOrd,");
    println!("  Serialize,");
    println!("  ToSchema,");
    println!("  VariantArray");
    println!(")]");
    println!("#[schema(as = models::sample::metadata::AnatomicalSite)]");
    println!("pub enum AnatomicalSite {{");
    for (i, entry) in graph.into_iter().enumerate() {
        println!("  /// `{}`", entry.label);
        println!("  ///");
        println!("  /// Link: <{}>.", entry.id);

        let mut variant_name = entry.label.to_pascal_case();

        for (original, replacement) in VARIANT_REPLACEMENTS {
            if variant_name.contains(original) {
                variant_name = variant_name.replace(original, replacement);
            }
        }

        println!(r#"  #[serde(rename = "{}")]"#, entry.label);
        if i % EVERY_N_ENTRIES != 0 {
            println!(r#"  #[cfg(feature = "all-anatomical-site")]"#);
        }

        println!("  {variant_name},");
        println!();

        if i % EVERY_N_ENTRIES != 0 {
            display_clauses.push(String::from(
                r#"    #[cfg(feature = "all-anatomical-site")]"#,
            ));
        }

        display_clauses.push(format!(
            r#"    Self::{} => write!(f, "{}"),"#,
            variant_name, entry.label
        ));
    }

    println!("}}");
    println!();
    println!("impl std::fmt::Display for AnatomicalSite {{");
    println!("  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {{");
    println!("    match self {{");
    for clause in display_clauses {
        println!("      {clause}");
    }
    println!("    }}");
    println!("  }}");
    println!("}}")
}

pub fn main(args: Args) -> Result<()> {
    let client = http::Client::default();

    let (version, json) = if let Some(path) = args.path {
        let version = args.version.unwrap_or_else(|| {
            panic!("the version must be provided if you provide a file path on the command line!")
        });

        let contents = std::fs::read_to_string(&path).expect("file to be read in");
        let json = serde_json::from_str(&contents).expect("file to be valid JSON");

        (version, json)
    } else {
        let url = if let Some(version) = args.version {
            info!("searching for Uberon version {}", version);
            uberon_version_url(&version)
        } else {
            info!("searching for latest Uberon version");
            LATEST_UBERON_URL.to_string()
        };

        let locations = client
            .follow_via_head(url)
            .context("determining the Uberon ontology URL")?;

        ////////////////////////////////////////////////////////////////////////////////////
        // Hooking the Uberon version.
        ////////////////////////////////////////////////////////////////////////////////////

        let mut version: Option<String> = None;

        for location in &locations {
            if let Some(captures) = UBERON_URL_VERSION_REGEX.captures(location) {
                // SAFETY: the regex requires that the first group matches.
                version = Some(captures.get(1).unwrap().as_str().to_string());
                break;
            }
        }

        let version = version.ok_or_else(|| {
            eprintln!("Failed to hook the Uberon version from the following URLs:");
            eprintln!();

            for location in &locations {
                eprintln!("  - {location}");
            }

            eprintln!();
            eprintln!(
            "This probably means that the code needs to be updated to handle a new URL convention."
        );

            eyre!("failed to hook Uberon version")
        })?;

        info!("hooked Uberon version is {}", version);

        ////////////////////////////////////////////////////////////////////////////////////
        // Downloading the Uberon ontology.
        ////////////////////////////////////////////////////////////////////////////////////

        // SAFETY: at this point, there should always be a final URL.
        let url = locations.last().unwrap();
        let text = client
            .get(url)?
            .text()
            .context("decoding the Uberon JSON file as text")?;

        (version, serde_json::from_str::<Value>(&text)?)
    };

    // Grab the `graphs` key from the top-level object.
    let graphs = json
        .get("graphs")
        .ok_or(eyre!(
            "`graphs` is expected to be a top-level key in the returned Uberon JSON object."
        ))?
        .as_array()
        .expect("`graphs` to be an array within the top-level Uberon JSON object.");

    // Ensure there is only one graph—else, the parsing code may need to be updated.
    assert!(
        graphs.len() == 1,
        "the `graphs` array is expected to only have one element"
    );

    let graph = graphs
        .first()
        // SAFETY: we just ensured that there is exactly one element in the graphs array.
        .unwrap();

    let graph = graph
        .as_object()
        .expect("the first element in the `graphs` array to be an object");

    ////////////////////////////////////////////////////////////////////////////////////
    // Build the node graph.
    ////////////////////////////////////////////////////////////////////////////////////

    let nodes = graph
        .get("nodes")
        .ok_or(eyre!("the `nodes` key to be available in the first graph"))?
        .as_array()
        .expect("the `nodes` key to be an array");

    let nodes = nodes
        .iter()
        .filter(|node| {
            let node = node.as_object().expect("a node must be an object");
            let r#type = node
                .get("type")
                .expect("node to have type")
                .as_str()
                .expect("node type to be a string");

            r#type == "CLASS" && node.contains_key("id") && node.contains_key("lbl")
        })
        .map(Node::from_value)
        .collect::<Result<Vec<_>, _>>()?;

    let edges = graph
        .get("edges")
        .ok_or(eyre!("the `edges` key to be available in the first graph"))?
        .as_array()
        .expect("the `edges` key to be an array")
        .iter()
        .filter_map(|v| Edge::from_value(v).unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    let mut graph = UberonGraph::default();
    let mut anatomical_entity_id = None;

    for node in nodes {
        if node.label == "anatomical entity" {
            info!("anatomic entity was hooked as id {}", &node.id);
            anatomical_entity_id = Some(node.id.clone());
        }

        graph.add_node(node);
    }

    let anatomical_entity_id =
        anatomical_entity_id.unwrap_or_else(|| panic!("could not hook anatomic entity id"));

    for edge in edges {
        graph.connect(&edge);
    }

    let nodes = graph.dfs(anatomical_entity_id);
    output_rust_enum(version, nodes);

    Ok(())
}

#[cfg(test)]
mod tests {}
