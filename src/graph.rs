// http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

impl Graph {
    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData { first_outgoing_edge: None });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }

    pub fn print(self) {
        for n in 0..self.nodes.len() {
            print!("Node {} goes to: ", n);
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

pub type NodeIndex = usize;
pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

pub type EdgeIndex = usize;
pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
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
    use super::*;
    #[test]
    fn create_small_graph() {
        let mut graph = Graph { nodes: vec![], edges: vec![] };
        let node0 = graph.add_node();
        let node1 = graph.add_node();
        graph.add_edge(node0, node1);

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
        match graph.nodes[0].first_outgoing_edge {
            Some(index) => assert_eq!(graph.edges[index].target, 1),
            None => assert!(false),
        }
    }

    #[test]
    fn create_list_graph() {
        let mut graph = Graph { nodes: vec![], edges: vec![] };
        let node0 = graph.add_node();
        let node1 = graph.add_node();
        let node2 = graph.add_node();
        let node3 = graph.add_node();
        graph.add_edge(node0, node1);
        graph.add_edge(node1, node2);
        graph.add_edge(node2, node3);

        let mut targets = vec![];
        for e in graph.edges {
            targets.push(e.target);
        }
        assert_eq!(targets, [1,2,3]);
    }

    #[test]
    fn list_node_edges() {
        let mut graph = Graph { nodes: vec![], edges: vec![] };
        let node0 = graph.add_node();
        graph.add_edge(node0, node0);
        graph.add_edge(node0, node0);
        graph.add_edge(node0, node0);
        let mut targets = vec![];
        let mut successors = graph.successors(0);
        loop {
            match successors.next() {
                Some(edge) => targets.push(edge),
                None => break,
            }
        }
        assert_eq!(targets, [0,0,0]);
    }
}
