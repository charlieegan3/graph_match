mod graph;
mod node;
mod edge;

pub fn match_graph(query: graph::Graph, root_index: node::Index, graph: graph::Graph) -> bool {
    let query_root_node = &query.nodes[root_index];

    let mut graph_root_node: Option<&node::Node> = None;
    for i in 0..graph.nodes.len() {
        if query_root_node.equal(&graph.nodes[i]) {
            graph_root_node = Some(&graph.nodes[i]);
            break;
        }
    }
    if graph_root_node.is_none() { return false; }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph;

    #[test]
    fn match_root_node() {
        let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = simple_graph.add_node("node0".to_string(), None);
        let node1 = simple_graph.add_node("node1".to_string(), None);
        simple_graph.add_edge(node0, node1, "edge0".to_string(), None);

        let mut query_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = query_graph.add_node("node0".to_string(), None);
        let node1 = query_graph.add_node("node1".to_string(), None);
        query_graph.add_edge(node0, node1, "edge0".to_string(), None);

        assert_eq!(match_graph(query_graph, 0, simple_graph), true);
    }
}
