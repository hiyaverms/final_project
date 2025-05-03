use petgraph::graph::{UnGraph, NodeIndex};
use std::collections::HashMap;

pub fn build_graph(movie_to_actors: HashMap<String, Vec<String>>) -> UnGraph<String, ()> {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut actor_nodes: HashMap<String, NodeIndex> = HashMap::new();

    for actors in movie_to_actors.values() {
        for i in 0..actors.len() {
            let a_id = &actors[i];
            let a_node = *actor_nodes.entry(a_id.clone())
                .or_insert_with(|| graph.add_node(a_id.clone()));
            
            for j in i + 1..actors.len() {
                let b_id = &actors[j];
                let b_node = *actor_nodes.entry(b_id.clone())
                    .or_insert_with(|| graph.add_node(b_id.clone()));
                
                if graph.find_edge(a_node, b_node).is_none() {
                    graph.add_edge(a_node, b_node, ());
                }
            }
        }
    }

    graph
}
