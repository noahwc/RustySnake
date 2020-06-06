use super::rocket;
use crate::{responses, logic, requests};
use rocket::http::{ContentType, Status};
use rocket::local::Client;
use std::fs::File;
use std::io::Read;

#[test]
fn get_move() {
    let turn: usize = 131;
    let mut f = File::open(format!("./log.txt")).expect("failed to open file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("failed to read file");
    let turns: Vec<&str> = buffer.lines().collect();
    let turn_data = turns[turn];
    let result: serde_json::Result<requests::Turn> = serde_json::from_str(turn_data);
    match result {
        Err(e) => { 
            println!("failed to parse turn_data: {}", e);
            assert!(false)
        },
        Ok(t) => {
            match logic::get_move(t) {
                Some(m) => { 
                    println!("{:?}",m);
                    assert!(true)
                },
                None => assert!(false),
            }
        }
    }

}

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
    // set turn here
    let turn: usize = 10;

    let mut f = File::open(format!("./log.txt")).expect("failed to open file");
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
    let turn: usize = 64;

    let mut f = File::open(format!("./log.txt")).expect("failed to open file");
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