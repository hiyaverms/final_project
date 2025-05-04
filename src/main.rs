use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use petgraph::visit::IntoNodeReferences;
use rand::thread_rng;

mod graph;
mod parser;

use graph::{
    build_graph, degree_centrality, closeness_centrality, betweenness_centrality,
    extract_subgraph_around_actor, shortest_path_length, random_actor_subgraph
};
use parser::read_dataset;

fn main() {
    // read dataset
    let (movie_to_actors, actor_id_to_name) = read_dataset("actor_name_data.tsv");

    // build the graph
    let actor_graph = build_graph(movie_to_actors);

    // create actor id to NodeIndex map
    let actor_id_map: HashMap<String, NodeIndex> = actor_graph
        .node_references()
        .map(|(idx, name)| (name.clone(), idx))
        .collect();

    // target actor id
    let priyanka_id = "nm1231899";

    // extract subgraph with depth 3
    let subgraph = extract_subgraph_around_actor(&actor_graph, &actor_id_map, priyanka_id, 1);

    println!("Subgraph contains {} nodes and {} edges.", subgraph.node_count(), subgraph.edge_count());

    // run centrality metrics on the subgraph
    println!("\nDegree Centrality:");
    for (node, deg) in degree_centrality(&subgraph) {
        println!("{:<35}: {:<5}", actor_id_to_name[&subgraph[node]], deg);
    }

    println!("\nCloseness Centrality:");
    for (node, c) in closeness_centrality(&subgraph) {
        println!("{:<35}: {:<7.3}", actor_id_to_name[&subgraph[node]], c);
    }

    println!("\nBetweenness Centrality:");
    for (node, b) in betweenness_centrality(&subgraph) {
        println!("{:<35}: {:<7.3}", actor_id_to_name[&subgraph[node]], b);
    }


    //random sample graph
    let mut rng = thread_rng(); 
    let sampled_graph = random_actor_subgraph(&actor_graph, 500, &mut rng); 

    println!("Sampled subgraph has {} nodes and {} edges.", sampled_graph.node_count(), sampled_graph.edge_count());

    println!("\nDegree Centrality on Sample:");
    for (node, deg) in degree_centrality(&sampled_graph) {
        println!("{:<35} {}", actor_id_to_name[&sampled_graph[node]], deg);
    }

    println!("\nCloseness Centrality on Sample:");
    for (node, c) in closeness_centrality(&sampled_graph) {
        println!("{:<35} {:.3}", actor_id_to_name[&sampled_graph[node]], c);
    }

    println!("\nBetweenness Centrality on Sample:");
    for (node, b) in betweenness_centrality(&sampled_graph) {
        println!("{:<35} {:.3}", actor_id_to_name[&sampled_graph[node]], b);
    }


    // find shortest path between major Bollywood actor and major Hollywood actor
    let amitabh_id = "nm0000821"; // amitabh bachchan id
    let timothee_id = "nm3154303"; // timothee chalamet id

    println!("\nShortest Path from Amitabh Bachchan to Timothée Chalamet:");

    let path_len = shortest_path_length(&actor_graph, amitabh_id, timothee_id);

    match path_len {
        Some(length) => {
            println!(
                "Shortest path length from {} to {}: {}",
                actor_id_to_name[amitabh_id],
                actor_id_to_name[timothee_id],
                length
            );
        }
        None => {
            println!("No path found between Amitabh Bachchan and Timothée Chalamet.");
        }
    }
}
