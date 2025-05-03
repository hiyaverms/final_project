use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::NodeIndexable;
use petgraph::algo::{dijkstra, connected_components};
use petgraph::unionfind::UnionFind;
use std::collections::HashMap;

// nodes = actor ids, edges = shared titles
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

// degree centrality(counts direct neighbors)
pub fn degree_centrality(graph: &UnGraph<String, ()>) -> HashMap<String, usize> {
    graph.node_indices()
        .map(|n| (graph[n].clone(), graph.neighbors(n).count()))
        .collect()
}

// closeness centrality(average distance to other nodes)
pub fn closeness_centrality(graph: &UnGraph<String, ()>) -> HashMap<String, f64> {
    let mut closeness = HashMap::new();
    for node in graph.node_indices() {
        let paths = dijkstra(graph, node, None, |_| 1);
        let sum_dist: f64 = paths.values().map(|&d| d as f64).sum();
        if sum_dist > 0.0 {
            closeness.insert(graph[node].clone(), (paths.len() as f64 - 1.0) / sum_dist);
        } else {
            closeness.insert(graph[node].clone(), 0.0);
        }
    }
    closeness
}

// number of connected components
pub fn num_connected_components(graph: &UnGraph<String, ()>) -> usize {
    connected_components(graph)
}

// maps each connected component to its actor IDs
pub fn connected_components_map(graph: &UnGraph<String, ()>) -> HashMap<usize, Vec<String>> {
    let mut uf = UnionFind::new(graph.node_bound());
    for edge in graph.edge_indices() {
        let (a, b) = graph.edge_endpoints(edge).unwrap();
        uf.union(a.index(), b.index());
    }

    let mut components: HashMap<usize, Vec<String>> = HashMap::new();
    for node in graph.node_indices() {
        let comp_id = uf.find(node.index());
        components.entry(comp_id).or_default().push(graph[node].clone());
    }

    components
}

// shortest path length between two actor IDs.
pub fn shortest_path_length(graph: &UnGraph<String, ()>, from: &str, to: &str) -> Option<usize> {
    let node_indices: HashMap<_, _> = graph.node_indices()
        .map(|i| (graph[i].clone(), i))
        .collect();

    let from_idx = *node_indices.get(from)?;
    let to_idx = *node_indices.get(to)?;

    let paths = dijkstra(graph, from_idx, Some(to_idx), |_| 1);
    paths.get(&to_idx).copied()
}
