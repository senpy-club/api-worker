// Copyright (C) 2021-2021 the senpy club
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

  let store = actix_ratelimit::MemoryStore::new();

  actix_web::HttpServer::new(move || {
    actix_web::App::new()
      .wrap(actix_cors::Cors::default().allow_any_origin())
      .wrap(
        actix_ratelimit::RateLimiter::new(
          actix_ratelimit::MemoryStoreActor::from(store.clone()).start(),
        )
        .with_interval(std::time::Duration::from_secs(60))
        .with_max_requests(100),
      )
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
