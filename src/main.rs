use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    data: String,
}

// TODO: Replace this with real functionality
async fn hello() -> Result<HttpResponse> {
    let response = HelloResponse {
        data: "hello world".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

// Health check endpoint: returns a simple "OK" response
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("OK"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let host = "127.0.0.1";

    println!("Starting server at http://{host}:{port}");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/health", web::get().to(health))
    })
    .bind((host, port))?
    .run()
    .await
}
