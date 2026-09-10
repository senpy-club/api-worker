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

use rand::thread_rng;
use serde_json::json;
use worker::{Response, Result};
use crate::{
  structures::Type,
  utils::{
    cors,
    github_api,
    images,
    languages as catalog_languages,
    random_entry,
  },
};

fn unavailable() -> Result<Response> {
  Response::from_json(
    &json!({"error": "The image catalogue is temporarily unavailable."}),
  )?
  .with_status(503)
  .with_cors(&cors())
}

pub fn index() -> Result<Response> {
  Response::ok(&*crate::constants::INDEX)?.with_cors(&cors())
}

pub async fn github(repository: Type) -> Result<Response> {
  match github_api(repository).await {
    Ok(catalog) => Response::from_json(&catalog)?.with_cors(&cors()),
    Err(_) => unavailable(),
  }
}

pub async fn languages(repository: Type) -> Result<Response> {
  match github_api(repository).await {
    Ok(catalog) =>
      Response::from_json(&catalog_languages(&catalog))?.with_cors(&cors()),
    Err(_) => unavailable(),
  }
}

pub async fn language(language: &str, repository: Type) -> Result<Response> {
  let Ok(language) = urlparse::unquote(language) else {
    return Response::from_json(
      &json!({"error": "The language is not valid URL-encoded text."}),
    )?
    .with_status(400)
    .with_cors(&cors());
  };

  match github_api(repository).await {
    Ok(catalog) =>
      Response::from_json(&images(&language, repository, &catalog))?
        .with_cors(&cors()),
    Err(_) => unavailable(),
  }
}

pub async fn random(repository: Type) -> Result<Response> {
  match github_api(repository).await {
    Ok(catalog) => {
      match random_entry(repository, &catalog, &mut thread_rng()) {
        Some(entry) => Response::from_json(&entry)?.with_cors(&cors()),
        None => unavailable(),
      }
    }

    Err(_) => unavailable(),
  }
}
