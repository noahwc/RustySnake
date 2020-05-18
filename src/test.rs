use super::rocket;
use crate::responses;
use rocket::http::{ContentType, Status};
use rocket::local::Client;
use std::env;
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
        .header(ContentType::JSON)
        .body(
            r#"{
            "game": {
                "id": "game-id-string"
            },
            "turn": 4,
            "board": {
                "height": 15,
                "width": 15,
                "food": [
                {
                    "x": 1,
                    "y": 3
                }
                ],
                "snakes": [
                    {
                        "id": "snake-id-string",
                        "name": "Sneky Snek",
                        "health": 90,
                        "body": [
                            {
                                "x": 1,
                                "y": 3
                            }
                        ]
                    }
                ]
            },
            "you": {
                "id": "snake-id-string",
                "name": "Sneky Snek",
                "health": 90,
                "body": [
                {
                    "x": 1,
                    "y": 3
                }
                ]
            }
        }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // test the response to match the regex
    let _start: responses::Start = serde_json::from_str(&response.body_string().unwrap()).unwrap();
}

#[test]
fn movement() {
    // set game id and turn here
    let game_id = "db2e56ff-ead4-443b-9f60-902edeed5472";
    let turn: usize = 16;

    let mut f = File::open(format!("./logs/{}.txt", game_id)).expect("failed to open file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("failed to read file");
    let turns: Vec<&str> = buffer.lines().collect();
    let turn_data = turns[turn];

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
}

#[test]
fn end() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let response = client.post("/end").dispatch();
    assert_eq!(response.status(), Status::Ok);
}