use actix_web::{HttpResponse, Responder, get};

use crate::cache::get_watchpoint_status;

#[get("/status")]
pub async fn status() -> impl Responder {
    match get_watchpoint_status() {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(()) => HttpResponse::InternalServerError().finish(),
    }
}
