use actix_web::web::Json;
use actix_web::HttpResponse;

use super::models::{CursorMove, CursorPosition, MouseAction};

#[get("/cursor/pos")]
pub async fn get_cursor_position() -> HttpResponse {
    match super::service::get_cursor_position() {
        Ok(pos) => HttpResponse::Ok()
            .content_type("application/json")
            .json(CursorPosition {
                x: pos[1],
                y: pos[3],
                screen: pos[5] as u16,
                window: pos[7],
            }),
        Err(_) => HttpResponse::InternalServerError().body("Failed to parse data"),
    }
}

#[post("/cursor/mov")]
pub async fn move_cursor(move_req: Json<CursorMove>) -> HttpResponse {
    super::service::move_cursor(move_req.into_inner());
    HttpResponse::Ok().into()
}

#[post("/cursor/click")]
pub async fn click(move_req: Json<MouseAction>) -> HttpResponse {
    super::service::click(move_req.into_inner());
    HttpResponse::Ok().into()
}

#[post("/cursor/click_down")]
pub async fn click_down(move_req: Json<MouseAction>) -> HttpResponse {
    super::service::click_down(move_req.into_inner());
    HttpResponse::Ok().into()
}

#[post("/cursor/click_up")]
pub async fn click_up(move_req: Json<MouseAction>) -> HttpResponse {
    super::service::click_up(move_req.into_inner());
    HttpResponse::Ok().into()
}
