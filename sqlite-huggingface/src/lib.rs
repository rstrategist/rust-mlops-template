/*
Hugging Face Rust library to analyse lyrics to songs and put them into a sqlite database.
*/
use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use rust_bert::RustBertError;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// Create zero shot classification candidates
// Creating in memory just for this demo, would typically persist to disk
fn create_db() -> sqlite::Connection {
    let db = sqlite::open(":memory:").unwrap();
    db.execute("CREATE TABLE zeroshotcandidates (id INTEGER PRIMARY KEY, label TEXT)")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('rock')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('pop')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('hip hop')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('country')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('latin')")
        .unwrap();
    db
}

// Return all zero shot classification candidates as a vector of strings
pub fn get_all_zeroshotcandidates() -> Vec<String> {
    let db = create_db();
    let query = "SELECT label FROM zeroshotcandidates";
    let mut candidates: Vec<String> = Vec::new();
    db.iterate(query, |pairs| {
        for &(_column, value) in pairs.iter() {
            let value = value.unwrap();
            candidates.push(value.to_string());
        }
        true
    })
    .unwrap();
    candidates
}

// Read lyrics from a file and return a vector of strings
pub fn read_lyrics(file: &str) -> Vec<String> {
    let mut lyrics: Vec<String> = Vec::new();
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lyrics.push(line.unwrap());
    }
    lyrics
}

/*
Use hugging face to classify lyrics using zero shot classification
Accepts a vector of strings as lyrics and grabs candidates from the in memory sqlite database
*/

pub fn classify_lyrics(lyrics: Vec<String>) -> Result<Vec<Vec<Label>>, RustBertError> {
    // Extract candidate lables from sqlite database put in an array
    let candidates = get_all_zeroshotcandidates();
    let candidate_labels: Vec<&str> = candidates.iter().map(|s| s.as_str()).collect();
    // Join lyrics into a single string
    let lyrics = lyrics.join(" ");
    // Convert to type std::convert::AsRef<[&str]>
    let lyrics = lyrics.as_ref();
    // Create a zero shot classification model
    let zero_shot_model = ZeroShotClassificationModel::new(Default::default()).unwrap();
    // Classify lyrics
    zero_shot_model.predict_multilabel([lyrics], candidate_labels, None, 128)
}
