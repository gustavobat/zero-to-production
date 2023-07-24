//! lib.rs
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run() -> std::io::Result<Server> {
    Ok(
        HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
            .bind("127.0.0.1:8000")?
            .run(),
    )
}
