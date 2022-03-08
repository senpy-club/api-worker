// This file is part of api-worker <https://github.com/senpy-club/api-worker>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

#![feature(type_ascription)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code
)]
#![deny(clippy::all, clippy::pedantic)] // clippy::nursery
#![recursion_limit = "128"]

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
