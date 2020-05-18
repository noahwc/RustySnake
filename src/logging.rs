use std::fs::OpenOptions;
use std::io::Write;
use crate::requests;
use serde_json::json;

pub fn log(turn: &requests::Turn) {
    // name file from game id
    let path = format!("./logs/{}.txt", turn.game.id);

    // Open a file in write only mode
    let mut file = OpenOptions::new().create(true).append(true).open(path).expect("unable to open file");

    // Write the current turn data to file
    file.write_all(json!(turn).to_string().as_bytes()).expect("unable to write to file");
    
    // Add newline
    file.write_all("\n".as_bytes()).expect("unable to write newline");
}