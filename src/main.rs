mod parser;
mod graph;

use parser::read_dataset;
use graph::build_graph;

fn main() {
    let path = "data/actors.csv"; // adjust as needed
    let movie_to_actors = read_dataset(path);
    let actor_graph = build_graph(movie_to_actors);

    println!("Graph has {} nodes and {} edges.", actor_graph.node_count(), actor_graph.edge_count());
}
