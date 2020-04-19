use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let bind = "0.0.0.0:3000";
    println!("Starting server on {}", bind);

    HttpServer::new(|| App::new().service(home))
        .bind(bind)?
        .run()
        .await
}
