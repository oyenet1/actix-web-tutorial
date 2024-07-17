use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    data: serde_json::Value, // Use serde_json::Value for flexible data structure
    status_code: u16,
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[get("/test")]
async fn test() -> impl Responder {
    let response = ApiResponse {
        message: "Request successful".to_string(),
        data: serde_json::json!({
            "student_id": 12345,
            "answer": "A",
        }),
        status_code: 201,
    };
    HttpResponse::Ok().json(response) // Return JSON response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello).service(test))
        .bind(("actix-web.test", 8080))?
        .run()
        .await
}
