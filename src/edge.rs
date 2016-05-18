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
    pub fn matches(&self, edge: &Edge) -> bool {
        match self.attributes {
            Some(ref attrs) => {
                match edge.attributes {
                    Some(ref edge_attrs) => {
                        for pair in attrs {
                            match edge_attrs.get(pair.0) {
                                Some(value) => {
                                    if value != pair.1 {
                                        return false
                                    }
                                },
                                None => return false
                            }
                        }
                    },
                    None => return false
                }
            },
            None => return true
        }
        return true;
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
        attributes2.insert("core".to_string(), "worth".to_string());

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
        assert!(edge0.matches(&edge1));
        assert_eq!(false, edge0.matches(&edge2));
    }
}
