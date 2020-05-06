use super::rocket;
use crate::{responses, requests, game};
use rocket::http::{ContentType, Status};
use rocket::local::Client;

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
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let mut response = client
        .post("/move")
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
    let body = response.body_string().unwrap();
    let _move: responses::Move = serde_json::from_str(&body).unwrap();
}

#[test]
fn end() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let response = client.post("/end").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn update_map() {
    let turn_data = r#"{
        "game": {
            "id": "game-id-string"
        },
        "turn": 1,
        "board": {
            "height": 2,
            "width": 2,
            "food": [
            {
                "x": 1,
                "y": 1
            }
            ],
            "snakes": [
                {
                    "id": "snake-id-string",
                    "name": "Sneky Snek",
                    "health": 90,
                    "body": [
                        {
                            "x": 0,
                            "y": 1
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
                "x": 0,
                "y": 1
            }
            ]
        }
    }"#;
    let turn: requests::Turn = serde_json::from_str(turn_data).unwrap();
    let mut g = game::UnGraph::<i32, i32>::new_undirected();
    let mut map: game::HashMap<requests::Point, game::NodeIndex> = game::HashMap::new();
    let points = game::all_points(&turn);

    game::update_map(&mut g, &mut map, & points);

    let mut test_map: game::HashMap<requests::Point, game::NodeIndex> = game::HashMap::new();
    let mut test_g = game::UnGraph::<i32, i32>::new_undirected();
    let test_points = vec![
        requests::Point{x:0, y:0},
        requests::Point{x:0, y:1},
        requests::Point{x:1, y:0},
        requests::Point{x:1, y:1}
    ];
    for p in test_points{
        test_map.insert(p, test_g.add_node(0));
    }

    assert_eq!(map, test_map);
}
