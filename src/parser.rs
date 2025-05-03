use std::collections::HashMap;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub actor_id: String,
    pub actor_name: String,
    pub movie_id: String,
}

pub fn read_dataset(path: &str) -> HashMap<String, Vec<String>> {
    let file = File::open(path).expect("Cannot open file");
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut movie_to_actors: HashMap<String, Vec<String>> = HashMap::new();

    for result in rdr.deserialize() {
        let record: Record = result.expect("Failed to deserialize record");
        movie_to_actors.entry(record.movie_id)
            .or_default()
            .push(record.actor_id);
    }

    movie_to_actors
}
