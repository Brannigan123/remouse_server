use actix_web::web::Json;
use actix_web::HttpResponse;

use super::models::{KeyPress, TextTyping};

#[post("/keyboard/press")]
pub async fn press_key(press_req: Json<KeyPress>) -> HttpResponse {
    super::service::press_key(press_req.into_inner());
    HttpResponse::Ok().into()
}

#[post("/keyboard/type")]
pub async fn type_text(typing_req: Json<TextTyping>) -> HttpResponse {
    super::service::type_text(typing_req.into_inner());
    HttpResponse::Ok().into()
}
