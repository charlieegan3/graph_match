use std::collections::HashMap;
use node;
use edge;

// http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
pub struct Graph {
    pub nodes: Vec<node::Node>,
    pub edges: Vec<edge::Edge>,
}

impl Graph {
    pub fn add_node(&mut self, identifier: String, attributes: Option<HashMap<String,String>>)
        -> node::Index {
            let index = self.nodes.len();
            self.nodes.push(node::Node { identifier: identifier, first_outgoing_edge: None, attributes: attributes });
            index
        }

    pub fn add_edge(&mut self, source: node::Index, target: node::Index, identifier: String, attributes: Option<HashMap<String,String>>) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(edge::Edge {
            identifier: identifier,
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
            attributes: attributes,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors(&self, source: node::Index) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }

    pub fn edges_for_node(&self, node_index: node::Index) -> Vec<edge::Index> {
        let mut edge_indexes: Vec<edge::Index> = vec![];
        match self.nodes[node_index].first_outgoing_edge {
            Some(edge_index) => {
                let mut edge = &self.edges[edge_index];
                edge_indexes.push(edge_index);
                loop {
                    match edge.next_outgoing_edge {
                        Some(edge_index) => {
                            edge = &self.edges[edge_index];
                            edge_indexes.push(edge_index);
                        },
                        None => { break; }
                    }
                }
            },
            None => {}
        }
        return edge_indexes;
    }

    pub fn print(self) {
        for n in 0..self.nodes.len() {
            print!("node::Node {} goes to: ", n);
            let mut suc = self.successors(n);
            loop {
                match suc.next() {
                    Some(s) => { print!("{}, ", s) },
                    None => { print!("\n"); break },
                }
            }
        }
    }
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<edge::Index>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = node::Index;

    fn next(&mut self) -> Option<node::Index> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    #[test]
    fn create_small_graph() {
        let mut graph = Graph { nodes: vec![], edges: vec![] };
        let node0 = graph.add_node("node0".to_string(), None);
        let node1 = graph.add_node("node1".to_string(), None);
        graph.add_edge(node0, node1, "edge0".to_string(), None);

        assert_eq!(2, graph.nodes.len());
        assert_eq!(1, graph.edges.len());
        match graph.nodes[0].first_outgoing_edge {
            Some(index) => assert_eq!(1, graph.edges[index].target),
            None => assert!(false),
        }
    }

    #[test]
    fn create_list_graph() {
        let mut graph = Graph { nodes: vec![], edges: vec![] };
        let node0 = graph.add_node("node0".to_string(), None);
        let node1 = graph.add_node("node1".to_string(), None);
        let node2 = graph.add_node("node2".to_string(), None);
        let node3 = graph.add_node("node3".to_string(), None);
        graph.add_edge(node0, node1, "edge0".to_string(), None);
        graph.add_edge(node1, node2, "edge1".to_string(), None);
        graph.add_edge(node2, node3, "edge2".to_string(), None);

        let mut targets = vec![];
        for e in graph.edges {
            targets.push(e.target);
        }
        assert_eq!(vec![1,2,3], targets);
    }

    #[test]
    fn list_node_edges() {
        let mut graph = Graph { nodes: vec![], edges: vec![] };
        let node0 = graph.add_node("node0".to_string(), None);
        graph.add_edge(node0, node0, "edge0".to_string(), None);
        graph.add_edge(node0, node0, "edge1".to_string(), None);
        graph.add_edge(node0, node0, "edge2".to_string(), None);
        let mut targets = vec![];
        let mut successors = graph.successors(0);
        loop {
            match successors.next() {
                Some(edge) => targets.push(edge),
                None => break,
            }
        }
        assert_eq!(vec![0,0,0], targets);
    }

    #[test]
    fn node_attributes() {
        let mut graph = Graph { nodes: vec![], edges: vec![] };

        let mut attributes = HashMap::new();
        attributes.insert("key".to_string(), "value".to_string());

        let node0 = graph.add_node("node0".to_string(), Some(attributes));
        match graph.nodes[node0].attributes {
            Some(ref attrs) => {
                match attrs.get("key") {
                    Some(value) => assert_eq!(&"value".to_string(), value),
                    None => assert!(false),
                }
            },
            None => assert!(false),
        }
    }

    #[test]
    fn edge_attributes() {
        let mut graph = Graph { nodes: vec![], edges: vec![] };

        let mut attributes = HashMap::new();
        attributes.insert("key".to_string(), "value".to_string());

        let node0 = graph.add_node("node0".to_string(), None);
        graph.add_edge(node0, node0, "edge0".to_string(), Some(attributes));
        match graph.edges[0].attributes {
            Some(ref attrs) => {
                match attrs.get("key") {
                    Some(value) => assert_eq!(&"value".to_string(), value),
                    None => assert!(false),
                }
            },
            None => assert!(false),
        }
    }

    #[test]
    fn node_edges() {
        let mut graph = Graph { nodes: vec![], edges: vec![] };
        let node0 = graph.add_node("node0".to_string(), None);
        graph.add_edge(node0, node0, "edge0".to_string(), None);
        graph.add_edge(node0, node0, "edge1".to_string(), None);
        graph.add_edge(node0, node0, "edge1".to_string(), None);

        assert_eq!(vec![2,1,0], graph.edges_for_node(node0));
    }
}
