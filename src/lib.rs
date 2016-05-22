pub mod graph;
mod node;
mod edge;

pub mod matching;

pub fn match_graph(query: &graph::Graph, query_root_index: node::Index, graph: &graph::Graph, equality: &matching::EqualityRequirement) -> Vec<matching::MatchedComponents> {
    let query_root_node = &query.nodes[query_root_index];

    let mut graph_roots: Vec<usize> = Vec::new();

    for i in 0..graph.nodes.len() {
        if query_root_node.matches(&graph.nodes[i], &equality) {
            graph_roots.push(i);
        }
    }

    return graph_roots.iter()
        .map(|&root_index| matching::recusive_node_match(query_root_index, root_index, &query, &graph, None, &equality))
        .filter(|&ref matched_components| matched_components.list.len() == query.nodes.len())
        .collect::<Vec<_>>();
}

pub fn expand_subgraph(
    graph: &graph::Graph, root_index: node::Index, banned_identifiers: &Vec<String>)
    -> Vec<node::Index> {
        let mut node_list = vec![root_index];
        let root_node_edges: Vec<node::Index> = graph.edges_for_node(root_index);
        for edge in root_node_edges {
            if banned_identifiers.contains(&graph.edges[edge].identifier) {
                continue;
            }
            let target = graph.edges[edge].target;
            for inner_node in expand_subgraph(graph, target, &banned_identifiers) {
                node_list.push(inner_node);
            }
        }

        return node_list;
    }
