use std::collections::BTreeMap;

use actix_web::{cookie::Cookie, dev::ServiceRequest, web::Json, HttpResponse, HttpRequest};

use super::models::Credentials;

#[post("/auth/login")]
pub async fn login(creds: Json<Credentials>) -> HttpResponse {
    match super::service::login(&creds.into_inner()) {
        Ok(token) => HttpResponse::Accepted()
            .cookie(Cookie::build("access_token", token).path("/").finish())
            .body("You are now logged in"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[post("/auth/refresh")]
pub async fn refresh(req: HttpRequest) -> HttpResponse {
    if let Some(header_value) = req.headers().get("access_token") {
        if let Ok(token) = header_value.to_str() {
            return match super::service::refresh(token.to_string()) {
                Ok(token) => HttpResponse::Accepted()
                    .cookie(Cookie::build("access_token", token).path("/").finish())
                    .body("Token Refreshed"),
                Err(err) => HttpResponse::BadRequest().body(err.to_string()),
            };
        }
    }
    return HttpResponse::BadRequest().body("Unauthorized".to_string());
}

pub fn check_auth(req: &ServiceRequest) -> Result<BTreeMap<String, String>, String> {
    if let Some(header_value) = req.headers().get("access_token") {
        if let Ok(token) = header_value.to_str() {
            return super::service::verify(token.to_string());
        }
    }
    return Err("UnAuthorized".to_string());
}
