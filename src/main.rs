#![feature(proc_macro_hygiene, decl_macro)]

// Modules
#[allow(dead_code)]
mod requests;
#[allow(dead_code)]
mod responses;
mod game;
mod logic;
mod node;
mod graph;
mod logging;
#[cfg(test)]
mod test;

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
    logging::log(&req);
    Json(logic::get_move(req.into_inner()))
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
