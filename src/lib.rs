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

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

mod constants;
mod routes;
mod structures;
mod utils;

use worker::Response;

/// # Errors
/// if `worker::Router` errors
#[worker::event(fetch)]
pub async fn main(
  request: worker::Request,
  environment: worker::Env,
  _: worker::Context,
) -> worker::Result<Response> {
  dotenv::dotenv().ok();

  worker::Router::new()
    .get("/", |_, _| routes::index())
    .get("/v2", |_, _| routes::index())
    .get_async("/v2/github", |_, _| async move { routes::github().await })
    .get_async(
      "/v2/languages",
      |_, _| async move { routes::languages().await },
    )
    .get_async("/v2/language/:language", |_, ctx| {
      async move {
        routes::language(ctx.param("language").unwrap_or(&"null".to_string()))
          .await
      }
    })
    .get_async("/v2/random", |_, _| async move { routes::random().await })
    .get("/v2/version", |_, _| {
      Response::from_json(&serde_json::json!({
        "crate_version": env!("CARGO_PKG_VERSION"),
        "git_commit_hash": env!("VERGEN_GIT_SHA"),
      }))?
      .with_cors(&utils::cors())
    })
    .run(request, environment)
    .await
}
