#![feature(let_chains)]
#[macro_use]
extern crate actix_web;

pub mod auth;
pub mod keyboard;
pub mod mouse;

use actix_web::{dev::Service, middleware, web, App, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslAcceptorBuilder};
use std::{env, io};

fn server_port() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1)
        .map(|s| s.to_owned())
        .unwrap_or("9090".to_string())
}

fn openssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    builder
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        let logger = middleware::Logger::default();

        App::new()
            .wrap(logger)
            .service(auth::controller::login)
            .service(
                web::scope("")
                    .wrap_fn(|req, srv| match auth::controller::check_auth(&req) {
                        Ok(_) => srv.call(req),
                        Err(e) => Box::pin(async move {
                            Ok(req.into_response(HttpResponse::Unauthorized().body(e)))
                        }),
                    })
                    .service(auth::controller::refresh)
                    .service(mouse::controller::get_cursor_position)
                    .service(mouse::controller::move_cursor)
                    .service(mouse::controller::click)
                    .service(keyboard::controller::press_key)
                    .service(keyboard::controller::type_text),
            )
    })
    .bind_openssl(format!("0.0.0.0:{}", server_port()), openssl_builder())?
    .run()
    .await
}
