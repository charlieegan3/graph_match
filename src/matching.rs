use graph;
use edge;
use node;

#[derive(Debug,PartialEq)]
pub struct Component {
    pub from_edge: Option<edge::Index>,
    pub node: node::Index,
}

#[derive(Debug,PartialEq)]
pub struct MatchedComponents {
    pub list: Vec<Component>,
}

pub fn recusive_node_match(query_root_index: usize,
                           graph_root_index: usize,
                           query: &graph::Graph,
                           graph: &graph::Graph,
                           source_edge: Option<edge::Index>)
                           -> MatchedComponents {
    if !query.nodes[query_root_index].matches(&graph.nodes[graph_root_index]) {
        return MatchedComponents { list: vec![] };
    }

    let mut matched_components = MatchedComponents {
        list: vec![Component {
                       from_edge: source_edge,
                       node: graph_root_index,
                   }],
    };

    if query.edges_for_node(query_root_index).len() == 0 {
        return matched_components;
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
                match query.nodes[query.edges[query_edge_index].target]
                    .matches(&graph.nodes[graph.edges[edge].target]) {
                    true => {
                        let descendents =
                            recusive_node_match(query.edges[query_edge_index].target,
                                                graph.edges[matching_edge_in_graph.unwrap()]
                                                    .target,
                                                &query,
                                                &graph,
                                                Some(edge));
                        for descendent in descendents.list {
                            matched_components.list.push(descendent);
                        }
                    }
                    false => return MatchedComponents { list: vec![] },
                }
            }
            None => return MatchedComponents { list: vec![] },
        }
    }
    return matched_components;
}
