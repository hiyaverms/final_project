use std::collections::HashMap;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub actor_id: String, //nconst
    pub actor_name: String, //primary name
    pub movie_id: String, //tconst
}

pub fn read_dataset(path: &str) -> (HashMap<String, Vec<String>>, HashMap<String, String>) {
    let file = File::open(path).expect("Cannot open file");
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t') 
        .from_reader(file);

    let mut movie_to_actors: HashMap<String, Vec<String>> = HashMap::new();
    let mut actor_id_to_name: HashMap<String, String> = HashMap::new();

    for result in rdr.deserialize() {
        let record: Record = result.expect("Failed to deserialize record");
        movie_to_actors
            .entry(record.movie_id.clone())
            .or_default()
            .push(record.actor_id.clone());
        actor_id_to_name
            .entry(record.actor_id)
            .or_insert(record.actor_name);
    }

    (movie_to_actors, actor_id_to_name)
}