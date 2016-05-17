mod graph;
mod node;
mod edge;

mod matching;

pub fn match_graph(query: graph::Graph, query_root_index: node::Index, graph: graph::Graph) -> bool {
    let query_root_node = &query.nodes[query_root_index];

    let mut graph_root_index: Option<usize> = None;
    for i in 0..graph.nodes.len() {
        if query_root_node.matches(&graph.nodes[i]) {
            graph_root_index = Some(i);
            break;
        }
    }
    if graph_root_index.is_none() { return false; }

    return matching::recusive_node_match(query_root_index, graph_root_index.unwrap(), &query, &graph);
}

pub fn expand_subgraph(graph: &graph::Graph, root_index: node::Index) -> Vec<node::Index> {
    let mut node_list = vec![root_index];
    let connected_nodes: Vec<node::Index> = graph.edges_for_node(root_index).iter().map(|&e| graph.edges[e].target).collect();
    for node in connected_nodes {
        for inner_node in expand_subgraph(graph, node) {
            node_list.push(inner_node);
        }
    }

    return node_list;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use graph;

    #[test]
    fn complete_traverse() {
        let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = simple_graph.add_node("node0".to_string(), None);
        let node1 = simple_graph.add_node("node1".to_string(), None);
        let node2 = simple_graph.add_node("node2".to_string(), None);
        simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
        simple_graph.add_edge(node1, node2, "edge1".to_string(), None);

        assert_eq!(expand_subgraph(&simple_graph, 0), vec![0,1,2]);
    }

    #[test]
    fn incomplete_traverse() {
        let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = simple_graph.add_node("node0".to_string(), None);
        let node1 = simple_graph.add_node("node1".to_string(), None);
        let node2 = simple_graph.add_node("node2".to_string(), None);
        simple_graph.add_edge(node0, node1, "edge0".to_string(), None);
        simple_graph.add_edge(node2, node1, "edge1".to_string(), None);

        assert_eq!(expand_subgraph(&simple_graph, 0), vec![0,1]);
    }

    #[test]
    fn match_complete_graph() {
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
    fn match_subgraph() {
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
    fn fail_to_match() {
        let mut attributes = HashMap::new();
        attributes.insert("key".to_string(), "value".to_string());
        let mut simple_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = simple_graph.add_node("node0".to_string(), Some(attributes.clone()));
        let node1 = simple_graph.add_node("node1".to_string(), Some(attributes.clone()));
        simple_graph.add_edge(node0, node1, "edge0".to_string(), Some(attributes.clone()));

        attributes.insert("key2".to_string(), "value2".to_string());
        let mut query_graph = graph::Graph { nodes: vec![], edges: vec![] };
        let node0 = query_graph.add_node("node0".to_string(), Some(attributes.clone()));
        let node1 = query_graph.add_node("node1".to_string(), Some(attributes.clone()));
        query_graph.add_edge(node0, node1, "edge0".to_string(), Some(attributes.clone()));

        assert_eq!(match_graph(query_graph, 0, simple_graph), false);
    }
}
