use std::collections::HashMap;
use node;

pub type Index = usize;

pub struct Edge {
    pub identifier: String,
    pub target: node::Index,
    pub attributes: Option<HashMap<String, String>>,
    pub next_outgoing_edge: Option<Index>,
}
