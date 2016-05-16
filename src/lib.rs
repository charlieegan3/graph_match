mod graph;
mod node;
mod edge;

fn recusive_node_match(
    query_root_index: usize, graph_root_index: usize, query: &graph::Graph, graph: &graph::Graph)
    -> bool {
        if !query.nodes[query_root_index].equal(&graph.nodes[graph_root_index]) {
            return false;
        }
        if query.edges_for_node(query_root_index).len() == 0 {
            return true;
        }

        for query_edge_index in query.edges_for_node(query_root_index) {
            let mut matching_edge_in_graph: Option<edge::Index> = None;
            for graph_edge_index in graph.edges_for_node(graph_root_index) {
                if query.edges[query_edge_index].equal(&graph.edges[graph_edge_index]) {
                    matching_edge_in_graph = Some(graph_edge_index);
                    break;
                }
            }
            match matching_edge_in_graph {
                Some(edge) => {
                    match query.nodes[query.edges[query_edge_index].target].equal(&graph.nodes[graph.edges[edge].target]) {
                        true => { return recusive_node_match(
                                            query.edges[query_edge_index].target,
                                            graph.edges[matching_edge_in_graph.unwrap()].target,
                                            &query,
                                            &graph)
                        },
                        false => { return false }
                    }
                },
                None => { return false; }
            }
        }
        return true;
}

pub fn match_graph(query: graph::Graph, query_root_index: node::Index, graph: graph::Graph) -> bool {
    let query_root_node = &query.nodes[query_root_index];

    let mut graph_root_index: Option<usize> = None;
    for i in 0..graph.nodes.len() {
        if query_root_node.equal(&graph.nodes[i]) {
            graph_root_index = Some(i);
            break;
        }
    }
    if graph_root_index.is_none() { return false; }

    return recusive_node_match(query_root_index, graph_root_index.unwrap(), &query, &graph);
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph;

    #[test]
    fn match_two_node_graph() {
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

    #[test]
    fn match_three_node_graph() {
        let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = simple_graph.add_node("node0".to_string(), None);
        let node1 = simple_graph.add_node("node1".to_string(), None);
        let node2 = simple_graph.add_node("node2".to_string(), None);
        simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
        simple_graph.add_edge(node1, node2, "edge1".to_string(), None);

        let mut query_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = query_graph.add_node("node0".to_string(), None);
        let node1 = query_graph.add_node("node1".to_string(), None);
        let node2 = query_graph.add_node("node2".to_string(), None);
        query_graph.add_edge(node0, node1, "edge0".to_string(), None);
        query_graph.add_edge(node1, node2, "edge1".to_string(), None);

        assert_eq!(match_graph(query_graph, 0, simple_graph), true);
    }

    #[test]
    fn match_three_node_subgraph() {
        let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = simple_graph.add_node("node0".to_string(), None);
        let node1 = simple_graph.add_node("node1".to_string(), None);
        let node2 = simple_graph.add_node("node2".to_string(), None);
        let node3 = simple_graph.add_node("node3".to_string(), None);
        simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
        simple_graph.add_edge(node1, node2, "edge1".to_string(), None);
        simple_graph.add_edge(node2, node3, "edge1".to_string(), None);

        let mut query_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = query_graph.add_node("node0".to_string(), None);
        let node1 = query_graph.add_node("node1".to_string(), None);
        let node2 = query_graph.add_node("node2".to_string(), None);
        query_graph.add_edge(node0, node1, "edge0".to_string(), None);
        query_graph.add_edge(node1, node2, "edge1".to_string(), None);

        assert_eq!(match_graph(query_graph, 0, simple_graph), true);
    }

    #[test]
    fn match_failure() {
        let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = simple_graph.add_node("node0".to_string(), None);
        let node1 = simple_graph.add_node("node1".to_string(), None);
        simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
        simple_graph.add_edge(node1, node0, "edge1".to_string(), None);

        let mut query_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = query_graph.add_node("node0".to_string(), None);
        let node1 = query_graph.add_node("node1".to_string(), None);
        let node2 = query_graph.add_node("node2".to_string(), None);
        query_graph.add_edge(node0, node1, "edge0".to_string(), None);
        query_graph.add_edge(node1, node2, "edge1".to_string(), None);

        assert_eq!(match_graph(query_graph, 0, simple_graph), false);
    }
}
