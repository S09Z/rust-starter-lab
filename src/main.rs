#[macro_use] extern crate rocket;

use rocket::{get, post, routes};
use rocket::serde::json::Json;
use log::{info, error};
use prometheus::{Encoder, IntCounter, Opts, Registry, TextEncoder};

#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref REQUEST_COUNTER: IntCounter = {
        let opts = Opts::new("request_count", "Number of requests received");
        IntCounter::with_opts(opts).unwrap()
    };
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Chat App"
}

#[post("/message", format = "json", data = "<message>")]
fn send_message(message: Json<String>) -> &'static str {
    // Logic to handle sending a message
    REQUEST_COUNTER.inc(); // Increment the request counter
    "Message sent"
}

#[get("/metrics")]
fn metrics() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&Registry::new().gather(), &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[launch]
fn rocket() -> _ {
    // Initialize logging
    env_logger::init();

    info!("Starting the chat app");
    
    rocket::build().mount("/", routes![index, send_message, metrics])
}
