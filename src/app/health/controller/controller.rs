use actix_web::HttpResponse;

pub async fn engine_check() -> HttpResponse {
    HttpResponse::Ok().body("Engine Running")
}