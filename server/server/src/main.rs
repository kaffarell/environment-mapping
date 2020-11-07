use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use posixmq::PosixMq;

// Post route where the degrees are send
#[post("/degree")]
async fn degree(req_body: String) -> impl Responder {
    // Create messagequeue with name /degree 
    let mq = PosixMq::create("/degree").unwrap();
    // Convert to integer and send degree
    mq.send(0, (req_body.trim().parse::<f32>().unwrap()).to_string().as_bytes()).unwrap();
    return HttpResponse::Ok().body("OK");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(degree)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

