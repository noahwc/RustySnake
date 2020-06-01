use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Copy, Clone, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn index(&self, width: usize) -> usize {
        width * self.y + self.x
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Turn {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Snake,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Game {
    pub id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Board {
    pub height: usize,
    pub width: usize,
    pub food: Vec<Point>,
    pub snakes: Vec<Snake>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Snake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Point>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_turn() {
        let turn1 = r#"{
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
        }"#;

        let correct: Turn = Turn {
            game: Game {
                id: "game-id-string".to_string(),
            },
            turn: 4,
            board: Board {
                height: 15,
                width: 15,
                food: vec![Point { x: 1, y: 3 }],
                snakes: vec![Snake {
                    id: "snake-id-string".to_string(),
                    name: "Sneky Snek".to_string(),
                    health: 90,
                    body: vec![Point { x: 1, y: 3 }],
                }],
            },
            you: Snake {
                id: "snake-id-string".to_string(),
                name: "Sneky Snek".to_string(),
                health: 90,
                body: vec![Point { x: 1, y: 3 }],
            },
        };

        let result: serde_json::Result<Turn> = serde_json::from_str(turn1);
        match result {
            Err(e) => {
                eprintln!("Returned value is Err: {}", e);
                assert!(false);
            }
            Ok(val) => {
                assert_eq!(correct, val);
            }
        }
    }
}
