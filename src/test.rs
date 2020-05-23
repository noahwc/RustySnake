use super::rocket;
use crate::responses;
use rocket::http::{ContentType, Status};
use rocket::local::Client;
use std::fs::File;
use std::io::Read;

#[test]
fn ping() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let response = client.post("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn start() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let mut response = client
        .post("/start")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // test the response to match the regex
    let _start: responses::Start = serde_json::from_str(&response.body_string().unwrap()).unwrap();
}

#[test]
fn movement() {
    // set game id and turn here
    let game_id = "4ff11f4a-bc02-454c-83c4-a76e92211103";
    let turn: usize = 10;

    let mut f = File::open(format!("./logs/{}.txt", game_id)).expect("failed to open file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("failed to read file");
    let turns: Vec<&str> = buffer.lines().collect();
    let turn_data = turns[turn-1];

    let client = Client::new(rocket()).expect("Failed to create client instance");
    let mut response = client
        .post("/move")
        .header(ContentType::JSON)
        .body(turn_data)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // test the response to match the regex
    let body = response.body_string().unwrap();
    let _move: responses::Move = serde_json::from_str(&body).unwrap();
    println!("MOVE: {:?}", _move);
}

#[test]
fn end() {
    // set game id and turn here
    let game_id = "19f1c9ea-c539-49eb-a09d-39647c3043de";
    let turn: usize = 64;

    let mut f = File::open(format!("./logs/{}.txt", game_id)).expect("failed to open file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("failed to read file");
    let turns: Vec<&str> = buffer.lines().collect();
    let turn_data = turns[turn-2];
    
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let response = client
        .post("/end")
        .header(ContentType::JSON)
        .body(turn_data)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}