mod parser;
mod graph;

use parser::read_dataset;
use graph::build_graph;

fn main() {
    let path = "./actors_name_data.tsv";
    let (movie_to_actors, actor_id_to_name) = read_dataset(path);
    let actor_graph = build_graph(movie_to_actors);

    println!("Graph has {} nodes and {} edges.", actor_graph.node_count(), actor_graph.edge_count());

    // degree centrality
    let degrees = degree_centrality(&actor_graph);
    let mut top_degree: Vec<_> = degrees.iter().collect();
    top_degree.sort_by(|a, b| b.1.cmp(a.1));
    println!("Top 10 actors by degree:");
    for (actor_id, degree) in top_degree.iter().take(10) {
        let name = actor_id_to_name.get(*actor_id).unwrap_or(&"Unknown".to_string());
        println!("{} ({}) - {}", actor_id, name, degree);
    }

    // closeness centrality
    let closeness = closeness_centrality(&actor_graph);
    let mut top_closeness: Vec<_> = closeness.iter().collect();
    top_closeness.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("Top 10 actors by closeness:");
    for (actor_id, score) in top_closeness.iter().take(10) {
        let name = actor_id_to_name.get(*actor_id).unwrap_or(&"Unknown".to_string());
        println!("{} ({}) - {:.4}", actor_id, name, score);
    }

    // connected components count
    let num_components = num_connected_components(&actor_graph);
    println!("Number of connected components: {}", num_components);
}
