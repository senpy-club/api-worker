// Copyleft 2021-2021 The Senpy Club
// SPDX-License-Identifier: GPL-3.0-only

#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use]
extern crate rocket;

pub mod constants;
pub mod routes;
pub mod structures;
pub mod utils;

#[launch]
fn rocket() -> _ {
  dotenv::dotenv().ok();

  rocket::build().mount("/", routes![routes::index]).mount(
    "/api/v1",
    routes![
      routes::github,
      routes::languages,
      routes::language,
      routes::random
    ],
  )
}
