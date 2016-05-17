use graph;
use edge;

pub fn recusive_node_match(
    query_root_index: usize, graph_root_index: usize, query: &graph::Graph, graph: &graph::Graph)
    -> bool {
        if !query.nodes[query_root_index].matches(&graph.nodes[graph_root_index]) {
            return false;
        }
        if query.edges_for_node(query_root_index).len() == 0 {
            return true;
        }

        for query_edge_index in query.edges_for_node(query_root_index) {
            let mut matching_edge_in_graph: Option<edge::Index> = None;
            for graph_edge_index in graph.edges_for_node(graph_root_index) {
                if query.edges[query_edge_index].matches(&graph.edges[graph_edge_index]) {
                    matching_edge_in_graph = Some(graph_edge_index);
                    break;
                }
            }
            match matching_edge_in_graph {
                Some(edge) => {
                    match query.nodes[query.edges[query_edge_index].target].matches(&graph.nodes[graph.edges[edge].target]) {
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
