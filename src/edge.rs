use std::collections::HashMap;
use node;

pub type Index = usize;

pub struct Edge {
    pub identifier: String,
    pub target: node::Index,
    pub attributes: Option<HashMap<String, String>>,
    pub next_outgoing_edge: Option<Index>,
}

impl Edge {
    pub fn equal(&self, edge: Edge) -> bool {
        self.identifier == edge.identifier &&
            self.attributes == edge.attributes
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    #[test]
    fn edge_equality() {
        let mut attributes = HashMap::new();
        attributes.insert("key".to_string(), "value".to_string());
        let mut attributes2 = HashMap::new();
        attributes.insert("core".to_string(), "worth".to_string());

        let edge0 = Edge {
            identifier: "edgeid".to_string(),
            attributes: Some(attributes.clone()),
            next_outgoing_edge: None,
            target: 0,
        };
        let edge1 = Edge {
            identifier: "edgeid".to_string(),
            attributes: Some(attributes.clone()),
            next_outgoing_edge: None,
            target: 0,
        };
        let edge2 = Edge {
            identifier: "edgeid".to_string(),
            attributes: Some(attributes2.clone()),
            next_outgoing_edge: None,
            target: 0,
        };
        assert!(edge0.equal(edge1));
        assert_eq!(edge0.equal(edge2), false);
    }
}
