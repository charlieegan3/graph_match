use std::collections::HashMap;
use edge;

pub type Index = usize;

pub struct Node {
    pub identifier: String,
    pub attributes: Option<HashMap<String, String>>,
    pub first_outgoing_edge: Option<edge::Index>,
}

impl Node {
    pub fn equal(&self, node: &Node) -> bool {
        self.identifier == node.identifier &&
            self.attributes == node.attributes
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    #[test]
    fn node_equality() {
        let mut attributes = HashMap::new();
        attributes.insert("key".to_string(), "value".to_string());
        let mut attributes2 = HashMap::new();
        attributes2.insert("core".to_string(), "worth".to_string());

        let node0 = Node {
            identifier: "nodeid".to_string(),
            attributes: Some(attributes.clone()),
            first_outgoing_edge: None,
        };
        let node1 = Node {
            identifier: "nodeid".to_string(),
            attributes: Some(attributes.clone()),
            first_outgoing_edge: None,
        };
        let node2 = Node {
            identifier: "nodeid".to_string(),
            attributes: Some(attributes2.clone()),
            first_outgoing_edge: None,
        };
        assert!(node0.equal(&node1));
        assert_eq!(node0.equal(&node2), false);
    }
}
