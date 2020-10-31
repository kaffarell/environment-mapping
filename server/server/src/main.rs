#![feature(decl_macro)]
#[macro_use] extern crate rocket;
use posixmq::PosixMq;


#[post("/degree", data = "<degree>")]
fn degree(degree: String) -> &'static str {
    let mq = PosixMq::create("/degree").unwrap();
    mq.send(0, (degree.trim().parse::<f32>().unwrap()).to_string().as_bytes()).unwrap();
    return "OK";
}

fn main() {
    // Setup messagequeue
    // Setup rocket
    rocket::ignite().mount("/", routes![degree]).launch();
}
