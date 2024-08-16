use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
