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

use urlparse::unquote;
use worker::Cors;

use crate::{
  constants,
  structures::{GitHubAPIResponse, Type},
};

static CACHE_UNSET: SyncLazy<Mutex<(bool, bool)>> =
  SyncLazy::new(|| Mutex::new((true, true)));
static CACHE_ACCESS_COUNT: SyncLazy<Mutex<(usize, usize)>> =
  SyncLazy::new(|| Mutex::new((0, 0)));
static GITHUB_API_CACHE: SyncLazy<
  Mutex<(GitHubAPIResponse, GitHubAPIResponse)>,
> = SyncLazy::new(|| {
  Mutex::new((GitHubAPIResponse::default(), GitHubAPIResponse::default()))
});

use crate::boys;

/// # Errors
/// if GitHub API is unresponsive
pub async fn github_api(
  repository: Type,
) -> Result<GitHubAPIResponse, Box<dyn std::error::Error>> {
  let unset = if repository == Type::Girls {
    (*CACHE_UNSET.lock().unwrap()).0
  } else {
    (*CACHE_UNSET.lock().unwrap()).1
  };
  let access_count = if repository == Type::Girls {
    (*CACHE_ACCESS_COUNT.lock().unwrap()).0
  } else {
    (*CACHE_ACCESS_COUNT.lock().unwrap()).1
  };

  if unset || access_count % 50 == 0 {
    if repository == Type::Girls {
      (*CACHE_UNSET.lock().unwrap()).0 = false;
    } else {
      (*CACHE_UNSET.lock().unwrap()).1 = false;
    };

    let mut client = reqwest::Client::new()
      .get(if repository == Type::Girls {
        &*constants::GITHUB_API_ENDPOINT
      } else {
        &*boys::GITHUB_API_ENDPOINT
      })
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

    let response = client
      .send()
      .await?
      .json::<GitHubAPIResponse>()
      .await
      .unwrap_or_default();

    if repository == Type::Girls {
      (*GITHUB_API_CACHE.lock().unwrap()).0 = response;
    } else {
      (*GITHUB_API_CACHE.lock().unwrap()).1 = response;
    }
  }

  if repository == Type::Girls {
    (*CACHE_ACCESS_COUNT.lock().unwrap()).0 += 1;
  } else {
    (*CACHE_ACCESS_COUNT.lock().unwrap()).1 += 1;
  };

  Ok(if repository == Type::Girls {
    (*GITHUB_API_CACHE.lock().unwrap()).0.clone()
  } else {
    (*GITHUB_API_CACHE.lock().unwrap()).1.clone()
  })
}

/// # Panics
/// if GitHub API is unresponsive
pub async fn filter_languages(repository: Type) -> Vec<String> {
  let mut languages = vec![];

  for i in github_api(repository).await.unwrap().tree {
    if i.r#type == "tree" {
      languages.push(i.path);
    }
  }

  languages
}

/// # Panics
/// if GitHub API is unresponsive
pub async fn filter_images_by_language(
  language: &str,
  repository: Type,
) -> Vec<String> {
  let mut images = vec![];

  // URL (percent) decoding
  let language = unquote(language).ok().unwrap();

  for item in github_api(repository.clone()).await.unwrap().tree {
    if item.path.split('/').collect::<Vec<&str>>()[0] == language
      && item.path.contains('/')
    {
      images.push(format!(
        "{}{}",
        if repository == Type::Girls {
          &*constants::GITHUB_USER_CONTENT
        } else {
          &*boys::GITHUB_USER_CONTENT
        },
        item.path
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
