use actix_web::{get, post, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Test server index"
}

#[post("/echo")]
async fn echo(data: String) -> impl Responder {
    data
}

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello from test server"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(echo)
        .service(hello))
        .bind("0.0.0.0:9000")?
        .run()
        .await
}