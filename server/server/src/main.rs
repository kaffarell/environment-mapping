#![feature(decl_macro)]
#[macro_use] extern crate rocket;
use posixmq::PosixMq;

// Post route where the degrees are send
#[post("/degree", data = "<degree>")]
fn degree(degree: String) -> &'static str {
    // Create messagequeue with name /degree 
    let mq = PosixMq::create("/degree").unwrap();
    // Convert to integer and send degree
    mq.send(0, (degree.trim().parse::<f32>().unwrap()).to_string().as_bytes()).unwrap();
    return "OK";
}

fn main() {
    // Setup rocket
    rocket::ignite().mount("/", routes![degree]).launch();
}
