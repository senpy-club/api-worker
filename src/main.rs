// Copyleft (ɔ) 2021-2021 The Senpy Club
// SPDX-License-Identifier: GPL-3.0-only

#![feature(type_ascription)]

#[macro_use]
extern crate actix_web;

pub mod constants;
pub mod routes;
pub mod structures;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();

  actix_web::HttpServer::new(|| {
    actix_web::App::new()
      .wrap(actix_cors::Cors::default().allow_any_origin())
      .service(routes::index)
      .service(
        actix_web::web::scope("/api/v1")
          .service(routes::github)
          .service(routes::languages)
          .service(routes::language)
          .service(routes::random),
      )
  })
  .bind(format!(
    "0.0.0.0:{}",
    std::env::var("PORT").expect("no port was provided... ~why~")
  ))?
  .run()
  .await
}
