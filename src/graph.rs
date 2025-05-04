//define functions to build the graph, compute centrality, and find the shortest path between nodes
//(nodes are actors, edges are titles. a title is either a movie or series)

use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::{Bfs, IntoNodeReferences};
use petgraph::algo::{dijkstra, connected_components};
use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::unionfind::UnionFind;
use rand::seq::IteratorRandom;
use rand::Rng;

// constructs an undirected graph where each actor is a node, and an edge exists between actors who co-starred in the same movie
// it takes `movie_to_actors` which is a map of movie IDs to lists of actor IDs
// and returns `UnGraph<String, ()>`, a graph of actor connections
pub fn build_graph(movie_to_actors: HashMap<String, Vec<String>>) -> UnGraph<String, ()> {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut actor_map: HashMap<String, NodeIndex> = HashMap::new();

    for actors in movie_to_actors.values() {
        for actor in actors {
            actor_map.entry(actor.clone()).or_insert_with(|| graph.add_node(actor.clone()));
        }

        //connect pairs
        for i in 0..actors.len() {
            for j in i + 1..actors.len() {
                let a_idx = actor_map[&actors[i]];
                let b_idx = actor_map[&actors[j]];
                graph.update_edge(a_idx, b_idx, ());
            }
        }
    }
    graph
}

//computes the number of neighbors for each node in the graph
//the graph itself is passed as input
//the output maps each node to its number of neighbors
pub fn degree_centrality(graph: &UnGraph<String, ()>) -> HashMap<NodeIndex, usize> {
    graph.node_indices().map(|n| (n, graph.neighbors(n).count())).collect()
}

//computes the average distance from a node to all other nodes in the graph
//the graph itself is passed as input
//the output maps each node to its closeness centrality score
pub fn closeness_centrality(graph: &UnGraph<String, ()>) -> HashMap<NodeIndex, f64> {
    let mut closeness = HashMap::new();
    for node in graph.node_indices() {
        let res = dijkstra(graph, node, None, |_| 1);
        let total_dist: usize = res.values().sum();
        let n = res.len();
        if total_dist > 0 {
            closeness.insert(node, (n - 1) as f64 / total_dist as f64);
        } else {
            closeness.insert(node, 0.0);
        }
    }
    closeness
}

//takes the graph as input and returns the number of connected components
pub fn num_connected_components(graph: &UnGraph<String, ()>) -> usize {
    connected_components(graph)
}

//maps each component/cluster to the nodes it contains
//input: the graph
//output: a map from component ID to a list of NodeIndexes belonging to that component
pub fn connected_components_map(graph: &UnGraph<String, ()>) -> HashMap<usize, Vec<NodeIndex>> {
    let mut uf = UnionFind::new(graph.node_count());
    //union nodes connected by an edge
    for edge in graph.edge_indices() {
        let (a, b) = graph.edge_endpoints(edge).unwrap();
        uf.union(a.index(), b.index());
    }

    //group nodes by component
    let mut map: HashMap<usize, Vec<NodeIndex>> = HashMap::new();
    for node in graph.node_indices() {
        let comp = uf.find(node.index());
        map.entry(comp).or_default().push(node);
    }

    map
}

//finds the shortest path between two nodes
//input: the graph, the id of the first actor, the id of the second actor
//output: the length of the path between them, if it exists 
pub fn shortest_path_length(
    graph: &UnGraph<String, ()>,
    from: &str,
    to: &str,
) -> Option<usize> {
    //map actor name to node indicies 
    let node_map: HashMap<String, NodeIndex> = graph
        .node_references()
        .map(|(idx, name)| (name.clone(), idx))
        .collect();
    let start = node_map.get(from)?;
    let end = node_map.get(to)?;

    //using dijkstra to find shortest path
    let res = dijkstra(graph, *start, Some(*end), |_| 1);
    res.get(end).copied()
}

//uses bfs to create a subgraph surrounding a particular actor
//input: the full graph, the actor id mappings, the actor name, and the depth(how many steps from the actor)
//output: an undirected subgraph containing the actor and neighbors within the given depth
pub fn extract_subgraph_around_actor(
    graph: &UnGraph<String, ()>,
    actor_id_map: &HashMap<String, NodeIndex>,
    actor_name: &str,
    max_depth: usize,
) -> UnGraph<String, ()> {
    let mut subgraph = UnGraph::<String, ()>::new_undirected();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut node_map = HashMap::new(); // maps original node indices to subgraph indices

    let start = match actor_id_map.get(actor_name) {
        Some(idx) => *idx,
        None => return subgraph, // actor not found
    };

    queue.push_back((start, 0));
    visited.insert(start);

    // add starting node
    let sub_start = subgraph.add_node(graph[start].clone());
    node_map.insert(start, sub_start);

    while let Some((node, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        for neighbor in graph.neighbors(node) { //visit each neighbor only once
            if visited.insert(neighbor) {
                queue.push_back((neighbor, depth + 1));
            }

            // add neighbor if its within max_depth
            if depth + 1 <= max_depth {
                let sub_node = *node_map.entry(node)
                    .or_insert_with(|| subgraph.add_node(graph[node].clone()));
                let sub_neighbor = *node_map.entry(neighbor)
                    .or_insert_with(|| subgraph.add_node(graph[neighbor].clone()));

                if !subgraph.contains_edge(sub_node, sub_neighbor) {
                    subgraph.add_edge(sub_node, sub_neighbor, ());
                }
            }
        }
    }

    subgraph
}

//randomly sample a set of nodes and builds a subgraph containing only them
//input: reference to the graph, sample size, and a mutable random number generator
//output: subgraph of the sampled nodes
pub fn random_actor_subgraph(
    graph: &UnGraph<String, ()>,
    sample_size: usize,
    rng: &mut impl Rng,
) -> UnGraph<String, ()> {
    let all_nodes: Vec<_> = graph.node_indices().collect();
    //randomly sample sample_size nodes
    let sampled_nodes: HashSet<_> = all_nodes
        .iter()
        .copied()
        .choose_multiple(rng, sample_size.min(all_nodes.len()))
        .into_iter()
        .collect();

    let mut subgraph = UnGraph::<String, ()>::new_undirected();
    let mut node_map = HashMap::new();

    //add sampled nodes to new subgraph
    for &node in &sampled_nodes {
        let idx = subgraph.add_node(graph[node].clone());
        node_map.insert(node, idx);
    }

    //add edges that exist in the original graph
    for &node in &sampled_nodes {
        for neighbor in graph.neighbors(node) {
            if sampled_nodes.contains(&neighbor) {
                let a = node_map[&node];
                let b = node_map[&neighbor];
                if !subgraph.contains_edge(a, b) {
                    subgraph.add_edge(a, b, ());
                }
            }
        }
    }

    subgraph
}

//computes betweeness centrality for all nodes of the graph
//input: reference to a graph
//output: hashmap of the nodeidex to the centrality score
pub fn betweenness_centrality(
    graph: &UnGraph<String, ()>,
) -> HashMap<NodeIndex, f64> {
    let mut bc = HashMap::new();
    for node in graph.node_indices() {
        bc.insert(node, 0.0);
    }

    for s in graph.node_indices() {
        let mut stack = Vec::new();
        let mut pred: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();
        let mut sigma: HashMap<NodeIndex, usize> = HashMap::new();
        let mut dist: HashMap<NodeIndex, isize> = HashMap::new();

        for v in graph.node_indices() {
            pred.insert(v, Vec::new());
            sigma.insert(v, 0);
            dist.insert(v, -1);
        }
        sigma.insert(s, 1);
        dist.insert(s, 0);

        //BFS traversal to calculate shortest paths
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            let d = dist[&v];
            for w in graph.neighbors(v) {
                if dist[&w] < 0 {
                    queue.push_back(w);
                    dist.insert(w, d + 1);
                }
                if dist[&w] == d + 1 {
                    sigma.insert(w, sigma[&w] + sigma[&v]);
                    pred.get_mut(&w).unwrap().push(v);
                }
            }
        }

        //back-propagation of dependencies
        //for centrality scores
        let mut delta: HashMap<NodeIndex, f64> = HashMap::new();
        for v in graph.node_indices() {
            delta.insert(v, 0.0);
        }

        while let Some(w) = stack.pop() {
            for v in &pred[&w] {
                let c = (sigma[v] as f64 / sigma[&w] as f64) * (1.0 + delta[&w]);
                delta.insert(*v, delta[&v] + c);
            }
            if w != s {
                bc.insert(w, bc[&w] + delta[&w]);
            }
        }
    }

    //normalize values so the scale is consistent
    let norm = if graph.node_count() <= 2 {
        1.0
    } else {
        ((graph.node_count() - 1) * (graph.node_count() - 2)) as f64 / 2.0
    };

    for val in bc.values_mut() {
        *val /= norm;
    }

    bc
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use petgraph::graph::UnGraph;

    //helper function to create sample data
    fn sample_movie_to_actors() -> HashMap<String, Vec<String>> {
        let mut data = HashMap::new();
        data.insert("m1".to_string(), vec!["a1".to_string(), "a2".to_string()]);
        data.insert("m2".to_string(), vec!["a2".to_string(), "a3".to_string()]);
        data.insert("m3".to_string(), vec!["a3".to_string(), "a4".to_string()]);
        data.insert("m4".to_string(), vec!["a4".to_string(), "a1".to_string()]);
        data
    }

    //test the building of the graph
    #[test]
    fn test_build_graph() {
        let data = sample_movie_to_actors();
        let graph = build_graph(data);
        assert_eq!(graph.node_count(), 4); //expect 4 actors
        assert_eq!(graph.edge_count(), 4); //expect 4 edges
    }

    //test the computation of degree centrality
    #[test]
    fn test_degree_centrality() {
        let mut movie_to_actors = HashMap::new();
        movie_to_actors.insert("m1".to_string(), vec!["a1".to_string(), "a2".to_string()]);
        movie_to_actors.insert("m2".to_string(), vec!["a2".to_string(), "a3".to_string()]);
        movie_to_actors.insert("m3".to_string(), vec!["a3".to_string(), "a1".to_string()]);

        let graph = build_graph(movie_to_actors);
        let degrees = degree_centrality(&graph);

        //all nodes should have degree 2 in this case
        for node in graph.node_indices() {
            assert_eq!(degrees[&node], 2);
        }
    }

    //test the computation of closeness centrality
    #[test]
    fn test_closeness_centrality() {
        let data = sample_movie_to_actors();
        let graph = build_graph(data);
        let closeness = closeness_centrality(&graph);
        // all nodes must have nonzero closeness
        for c in closeness.values() {
            assert!(*c > 0.0);
        }
    }

    //test number of connected components
    #[test]
    fn test_connected_components() {
        let data = sample_movie_to_actors();
        let graph = build_graph(data);
        let num = num_connected_components(&graph);
        assert_eq!(num, 1); //expect only one connected component due to shape of the graph
    }

    //test connected component grouping
    #[test]
    fn test_connected_components_map() {
        let data = sample_movie_to_actors();
        let graph = build_graph(data);
        let comp_map = connected_components_map(&graph);
        assert_eq!(comp_map.len(), 1); //one component expected
        let (_, group) = comp_map.iter().next().unwrap();
        assert_eq!(group.len(), 4);//component should include all four nodes
    }

    //test computation of shortest path
    #[test]
    fn test_shortest_path_length() {
        let data = sample_movie_to_actors();
        let graph = build_graph(data);
        let len = shortest_path_length(&graph, "a1", "a3");
        assert_eq!(len, Some(2)); //path is a1->a2->a3
    }

    //test extracting a subgraph within one "step" of node C
    #[test]
    fn test_extract_subgraph_around_actor() {
        let mut graph = UnGraph::<String, ()>::new_undirected();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());
        let d = graph.add_node("D".to_string());
        let e = graph.add_node("E".to_string());

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());
        graph.add_edge(c, d, ());
        graph.add_edge(d, e, ());

        let mut actor_id_map = HashMap::new();
        actor_id_map.insert("A".to_string(), a);
        actor_id_map.insert("B".to_string(), b);
        actor_id_map.insert("C".to_string(), c);
        actor_id_map.insert("D".to_string(), d);
        actor_id_map.insert("E".to_string(), e);

        //extract graph centered on C with depth 1
        let subgraph = extract_subgraph_around_actor(&graph, &actor_id_map, "C", 1);
        let names: HashSet<_> = subgraph.node_references().map(|(_, name)| name.clone()).collect();
        let expected: HashSet<_> = ["B", "C", "D"].iter().map(|s| s.to_string()).collect(); //shouldnt be able to reach A
        assert_eq!(names, expected);
    }

    //test generation of a random subgraph
    #[test]
    fn test_random_actor_subgraph() {
        use rand::SeedableRng; //so I know what to expect
        use rand::rngs::StdRng;

        let data = sample_movie_to_actors(); 
        let graph = build_graph(data);
        let mut rng = StdRng::seed_from_u64(12345); // seed

        let subgraph = random_actor_subgraph(&graph, 3, &mut rng);

        //correct number of nodes
        assert_eq!(subgraph.node_count(), 3);

        // all nodes in subgraph should have names from original graph
        let valid_names: HashSet<_> = graph.node_references().map(|(_, name)| name.clone()).collect();
        for (_, name) in subgraph.node_references() {
            assert!(valid_names.contains(name));
        }

        // edges must only connect valid sampled nodes
        for edge in subgraph.edge_indices() {
            let (a, b) = subgraph.edge_endpoints(edge).unwrap();
            assert_ne!(a, b);
        }
    }


}
