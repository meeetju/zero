use actix_web::{HttpResponse, Responder};

//curl -v http://127.0.0.1:8000/health_check
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
