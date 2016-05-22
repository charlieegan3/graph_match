use std::collections::HashMap;
use edge;
use matching;
use matching::EqualityRequirement;

pub type Index = usize;

pub struct Node {
    pub identifier: String,
    pub attributes: Option<HashMap<String, String>>,
    pub first_outgoing_edge: Option<edge::Index>,
}

impl Node {
    pub fn matches(&self, node: &Node, equality: &EqualityRequirement) -> bool {
        // test the attributes
        match self.attributes {
            Some(ref attrs) => {
                // test the node's attributes
                match node.attributes {
                    Some(ref node_attrs) => {
                        // for each of the query attributes, test the node
                        for pair in attrs {
                            match node_attrs.get(pair.0) {
                                Some(value) => {
                                    if !matching::values_match(value, pair.1, &equality) {
                                        return false;
                                    }
                                }
                                None => return false,
                            }
                        }
                    }
                    // when node is empty and query is not then the node is not matched
                    None => return false,
                }
            }
            // no attributes means a blank query
            None => return true,
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use matching::EqualityRequirement;
    use super::*;
    #[test]
    fn node_complete_equality() {
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
        assert!(node0.matches(&node1, &EqualityRequirement::Complete));
        assert_eq!(false, node0.matches(&node2, &EqualityRequirement::Complete));
    }

    #[test]
    fn node_contains_equality() {
        let mut attributes = HashMap::new();
        attributes.insert("key".to_string(), "value".to_string());
        let mut attributes2 = HashMap::new();
        attributes2.insert("key".to_string(), "the values".to_string());

        let node0 = Node {
            identifier: "nodeid".to_string(),
            attributes: Some(attributes.clone()),
            first_outgoing_edge: None,
        };
        let node1 = Node {
            identifier: "nodeid".to_string(),
            attributes: Some(attributes2.clone()),
            first_outgoing_edge: None,
        };
        assert!(node0.matches(&node1, &EqualityRequirement::Contains));
    }
}
