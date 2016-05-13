use std::collections::HashMap;
use edge;

pub type Index = usize;

pub struct Node {
    pub identifier: String,
    pub attributes: Option<HashMap<String, String>>,
    pub first_outgoing_edge: Option<edge::Index>,
}
