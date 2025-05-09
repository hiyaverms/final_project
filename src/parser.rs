//parses the tsv such that each movie id is mapped to a list of actor ids and each actor id is mapped to a name
use std::collections::HashMap;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]   //a struct representing one row of the file
pub struct Record {             //used to link an actor to a movie and give the actor's name
    #[serde(rename = "nconst")]
    pub actor_id: String,

    #[serde(rename = "primaryName")]
    pub actor_name: String,

    #[serde(rename = "tconst")]
    pub movie_id: String,
}

// Reads the TSV dataset (using parameter path) and returns(as a tuple):
// 1. a hashmap mapping each movie_id to a list of actor_ids to construct the graph
// 2. a hashmap mapping actor_id to actor_name so its readable
pub fn read_dataset(path: &str) -> (HashMap<String, Vec<String>>, HashMap<String, String>) {
    let file = File::open(path).expect("Cannot open file");
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t') //because it is a tsv
        .has_headers(true)
        .from_reader(file);

    let mut movie_to_actors: HashMap<String, Vec<String>> = HashMap::new();
    let mut actor_id_to_name: HashMap<String, String> = HashMap::new();

    for result in rdr.deserialize() {
        let record: Record = result.expect("Failed to deserialize record");

        // map movie ID to list of actor IDs
        movie_to_actors
            .entry(record.movie_id.clone())
            .or_default()
            .push(record.actor_id.clone());

        // map actor ID to name
        actor_id_to_name
            .entry(record.actor_id)
            .or_insert(record.actor_name);
    }

    (movie_to_actors, actor_id_to_name)
}
