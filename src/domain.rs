use std::collections::BinaryHeap;

use ordered_float::NotNan;
use serde::{Deserialize, Serialize};

/// The root of the JSON tree.
#[derive(Serialize, Deserialize)]
struct Root<'a> {
    #[serde(borrow)]
    root: Tree<'a>,
}

/// A tree node.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Tree<'a> {
    Node {
        #[serde(default)]
        length: f64,
        children: Vec<Tree<'a>>,
    },
    Leaf {
        #[serde(default)]
        length: f64,
        name: &'a str,
    },
}

impl<'a> Tree<'a> {
    /// Returns the number of leaves in the tree.
    fn size(&self) -> usize {
        match self {
            Tree::Node { children, .. } => children.iter().map(|x| x.size()).sum(),
            _ => 1,
        }
    }

    /// Walks the tree and returns an iterator over all the results.
    fn walk(&self, curr: f64) -> 
        Box<dyn Iterator<Item = Result<(NotNan<f64>, &'a str), String>> + '_> {
        match self {
            Tree::Leaf { length, name } => Box::new(std::iter::once(
                NotNan::new(curr + length)
                    .map_err(|e| e.to_string())
                    .map(|not_nan| (not_nan, *name))
            )),
            Tree::Node { length, children } => Box::new(children
                .iter()
                .flat_map(move |x| x.walk(curr + length / self.size() as f64))
            ),
        }
    }
}

/// Solves the problem.
pub fn solve(json: &str) -> Result<BinaryHeap<(NotNan<f64>, String)>, String> {
    Ok(
        serde_json::from_str::<Root>(json)
            .map_err(|e| e.to_string())?
            .root
            .walk(0.0)
            .map(|x| x.map(|(x, y)| (x, y.to_string())))
            .collect::<Result<BinaryHeap<_>, _>>()?
    )
}