#![feature(proc_macro_hygiene, decl_macro)]

// Modules
#[allow(dead_code)]
mod requests;
#[allow(dead_code)]
mod responses;
#[cfg(test)]
mod test;
mod graph;

// External crates
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

// Uses
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Helo World"
}

#[post("/start", format = "json", data = "<req>")]
fn start(req: Json<requests::Turn>) -> Json<responses::Start> {
    Json(responses::Start::new(
        "#CE3D16".to_string(),
        responses::HeadType::Regular,
        responses::TailType::Regular,
    ))
}

#[post("/move", format = "json", data = "<req>")]
fn movement(req: Json<requests::Turn>) -> Json<responses::Move> {
    let movement = responses::Move::new(responses::Movement::Left);
    // Logic goes here
    let (mut map, mut g) = graph::new(&(req.into_inner()));
    //println!("{}", serde_json::to_string_pretty(&g).unwrap());
    println!("{:?}", graph::Dot::new(&g));
    // Return
    Json(movement)
}

#[post("/end")]
fn end() -> &'static str {
    "Thanks for the game"
}

#[post("/ping")]
fn ping() -> &'static str {
    "Alive and well"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, start, movement, end, ping])
}

fn main() {
    println!("Hello World");
    rocket().launch();
}
