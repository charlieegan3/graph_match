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

pub type NodeIndex = usize;
pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

pub type EdgeIndex = usize;
pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut g = Graph { nodes: vec![], edges: vec![] };
        let node0 = g.add_node();
        let node1 = g.add_node();
        let node2 = g.add_node();
        let node3 = g.add_node();
        g.add_edge(node0, node1);
        g.add_edge(node0, node2);
        g.add_edge(node2, node1);
        g.add_edge(node2, node3);
        g.print();
    }
}
