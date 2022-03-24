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

use std::{lazy::SyncLazy, sync::Mutex};

use worker::Cors;

use crate::{constants, structures::GitHubAPIResponse};

static CACHE_UNSET: SyncLazy<Mutex<bool>> = SyncLazy::new(|| Mutex::new(true));
static CACHE_ACCESS_COUNT: SyncLazy<Mutex<usize>> =
  SyncLazy::new(|| Mutex::new(0));
static GITHUB_API_CACHE: SyncLazy<Mutex<GitHubAPIResponse>> =
  SyncLazy::new(|| Mutex::new(GitHubAPIResponse::default()));

/// # Errors
/// if GitHub API is unresponsive
pub async fn github_api(
) -> Result<GitHubAPIResponse, Box<dyn std::error::Error>> {
  if *CACHE_UNSET.lock().unwrap()
    || *CACHE_ACCESS_COUNT.lock().unwrap() % 50 == 0
  {
    *CACHE_UNSET.lock().unwrap() = false;

    let mut client = reqwest::Client::new()
      .get(&*constants::GITHUB_API_ENDPOINT)
      .header(
        "User-Agent",
        format!("senpy-club/api-worker - {}", env!("VERGEN_GIT_SHA")),
      );

    if std::env::var("GITHUB_TOKEN").is_ok() {
      client = client.header(
        "Authorization",
        format!(
          "token {}",
          std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| "Null".to_string())
        ),
      );
    }

    *GITHUB_API_CACHE.lock().unwrap() = client
      .send()
      .await?
      .json::<GitHubAPIResponse>()
      .await
      .unwrap_or_default();
  }

  *CACHE_ACCESS_COUNT.lock().unwrap() += 1;

  Ok((*GITHUB_API_CACHE.lock().unwrap()).clone())
}

/// # Panics
/// if GitHub API is unresponsive
pub async fn filter_languages() -> Vec<String> {
  let mut languages = vec![];

  for i in github_api().await.unwrap().tree {
    if i.r#type == "tree" {
      languages.push(i.path);
    }
  }

  languages
}

/// # Panics
/// if GitHub API is unresponsive
pub async fn filter_images_by_language(language: &str) -> Vec<String> {
  let mut images = vec![];

  // URL (percent) encoding of pound symbol to pound symbol
  let language = language.replace("%23", "#");

  for item in github_api().await.unwrap().tree {
    if item.path.split('/').collect::<Vec<&str>>()[0] == language
      && item.path.contains('/')
    {
      images.push(format!(
        "{}{}",
        *constants::GITHUB_USER_CONTENT,
        // Pound symbols to URL (percent) encoding of pound symbol because we
        // are pushing a URL, not a string
        item.path.replace('#', "%23")
      ));
    }
  }

  images
}

pub fn cors() -> Cors {
  Cors::default()
    .with_origins(vec!["*"])
    .with_methods(vec![worker::Method::Get])
}
