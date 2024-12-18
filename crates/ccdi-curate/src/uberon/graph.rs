//! A graph structure for traversing the Uberon ontology.
//!
//! This is useful for picking out the nodes that we want to be classifiable to.

use std::collections::HashMap;

use eyre::eyre;
use eyre::Result;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

/// A node in the Uberon ontology.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
    /// The identifier.
    pub id: String,

    /// The label.
    pub label: String,

    /// Any synonyms.
    pub synonyms: Vec<String>,
}

impl Node {
    /// Attempts to create a [`Node`] directly from a parsed JSON value from an
    /// Uberon file.
    pub fn from_value(value: &Value) -> Result<Self> {
        let value = value
            .as_object()
            .ok_or(eyre!("a node entry was not a JSON object"))?;

        let id = value
            .get("id")
            .ok_or(eyre!("the node entry object did not contain an `id` key"))?
            .as_str()
            .ok_or(eyre!("the node entry object `id` key was not a string"))?
            .to_string();

        let label = value
            .get("lbl")
            .ok_or(eyre!("the node entry object did not contain an `lbl` key"))?
            .as_str()
            .ok_or(eyre!("the node entry object `lbl` key was not a string"))?
            .to_string();

        let synonyms = value
            .get("meta")
            .and_then(|meta| {
                meta.get("synonyms").map(|value| {
                    value
                        .as_array()
                        .expect("`synonyms` was not an array")
                        .iter()
                        .map(|synonym| {
                            synonym
                                .as_object()
                                .expect("synonym to be object")
                                .get("val")
                                .expect("synonym to have `val` key")
                                .as_str()
                                .expect("`val` of a synonym to be a string")
                                .to_string()
                        })
                        .collect::<Vec<String>>()
                })
            })
            .unwrap_or_default();

        Ok(Self {
            id,
            label,
            synonyms,
        })
    }
}

/// An edge in the Uberon ontology.
pub struct Edge {
    /// The parent identifier.
    parent: String,

    /// The child identifier.
    child: String,
}

impl Edge {
    /// Attempts to create an [`Edge`] directly from a parsed JSON value from an
    /// Uberon file.
    pub fn from_value(value: &Value) -> Result<Option<Self>> {
        let value = value
            .as_object()
            .ok_or(eyre!("an edge entry was not a JSON object"))?;

        let predicate = value
            .get("pred")
            .ok_or(eyre!("the edge entry object did not contain a `pred` key"))?
            .as_str()
            .ok_or(eyre!("the edge entry object `pred` key was not a string",))?;

        match predicate {
            "is_a" => {}
            "subPropertyOf" | "inverseOf" => return Ok(None),
            v => {
                // If the predicate starts with one of the special purl URLs,
                // then we don't typically care about those relationships for
                // this kind of graph.
                if predicate.starts_with("http://purl.obolibrary.org/obo/") {
                    return Ok(None);
                }

                todo!("unhandled predicate: {v}")
            }
        }

        let parent = value
            .get("obj")
            .ok_or(eyre!("the edge entry object did not contain a `obj` key"))?
            .as_str()
            .ok_or(eyre!("the edge entry object `obj` key was not a string",))?
            .to_string();

        let child = value
            .get("sub")
            .ok_or(eyre!("the edge entry object did not contain a `sub` key"))?
            .as_str()
            .ok_or(eyre!("the edge entry object `sub` key was not a string",))?
            .to_string();

        Ok(Some(Edge { parent, child }))
    }
}

/// The Uberon graph.
#[derive(Debug, Default)]
pub struct UberonGraph {
    /// The inner graph.
    graph: DiGraph<Node, ()>,

    /// A map between Uberon identifiers and their node index.
    node_map: HashMap<String, NodeIndex>,
}

impl UberonGraph {
    /// Adds a node to the Uberon graph.
    pub fn add_node(&mut self, node: Node) {
        let id = node.id.clone();

        if self.node_map.contains_key(&id) {
            panic!("this node has already been inserted into the node map: {id}");
        }

        self.node_map.insert(id, self.graph.add_node(node));
    }

    /// Connects a parent and child node.
    pub fn connect(&mut self, edge: &Edge) {
        let parent = *self
            .node_map
            .get(&edge.parent)
            .expect("parent node does not exist in node map");

        let child = *self
            .node_map
            .get(&edge.child)
            .expect("child node does not exist in node map");

        self.graph.add_edge(parent, child, ());
    }

    /// Does a DFS on the tree, returning every node therein, by starting at the
    /// node with the provided node name.
    pub fn dfs(&self, node_name: String) -> Vec<Node> {
        let mut results = Vec::new();

        let node_index = *self
            .node_map
            .get(&node_name)
            .expect("root node for DFS does not exist in node map");

        let mut dfs = Dfs::new(&self.graph, node_index);

        while let Some(node) = dfs.next(&self.graph) {
            results.push(self.graph.node_weight(node).unwrap().clone());
        }

        results
    }
}
