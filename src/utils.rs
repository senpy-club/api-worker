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

use std::sync::{LazyLock, Mutex};
use rand::{seq::SliceRandom, Rng};
use worker::Cors;
use crate::{
  boys,
  constants,
  structures::{GitHubAPIResponse, SenpyRandom, Type},
};

type CatalogResult = Result<GitHubAPIResponse, Box<dyn std::error::Error>>;

#[derive(Default)]
struct CatalogCache {
  catalog: Option<GitHubAPIResponse>,
  accesses:  usize,
}

impl CatalogCache {
  fn should_refresh(&mut self) -> bool {
    let refresh = self.catalog.is_none() || self.accesses == 0;

    self.accesses = (self.accesses + 1) % 50;

    refresh
  }

  fn accept(&mut self, fetched: CatalogResult) -> CatalogResult {
    match fetched {
      Ok(catalog) => {
        self.catalog = Some(catalog.clone());

        Ok(catalog)
      }

      Err(error) => self.catalog.clone().ok_or(error),
    }
  }
}

static CATALOGS: LazyLock<Mutex<[CatalogCache; 2]>> = LazyLock::new(|| {
  Mutex::new([CatalogCache::default(), CatalogCache::default()])
});

fn validate_catalog(catalog: GitHubAPIResponse) -> CatalogResult {
  if catalog.truncated || languages(&catalog).is_empty() {
    return Err("GitHub returned an incomplete or empty catalogue.".into());
  }

  Ok(catalog)
}

pub async fn github_api(repository: Type) -> CatalogResult {
  let index = usize::from(repository == Type::Boys);
  let refresh = CATALOGS.lock().unwrap()[index].should_refresh();

  if !refresh {
    return Ok(CATALOGS.lock().unwrap()[index].catalog.clone().unwrap());
  }

  let fetched = async {
    let mut request = reqwest::Client::new()
      .get(if repository == Type::Girls {
        &*constants::GITHUB_API_ENDPOINT
      } else {
        &*boys::GITHUB_API_ENDPOINT
      })
      .header(
        "User-Agent",
        format!("senpy-club/api-worker - {}", env!("VERGEN_GIT_SHA")),
      );

    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
      request = request.header("Authorization", format!("Bearer {token}"));
    }

    let catalog = request
      .send()
      .await?
      .error_for_status()?
      .json::<GitHubAPIResponse>()
      .await?;

    validate_catalog(catalog)
  }
  .await;

  CATALOGS.lock().unwrap()[index].accept(fetched)
}

pub fn languages(catalog: &GitHubAPIResponse) -> Vec<String> {
  catalog
    .tree
    .iter()
    .filter(|entry| entry.r#type == "tree" && !entry.path.contains('/'))
    .map(|entry| entry.path.clone())
    .collect()
}

pub fn images(
  language: &str,
  repository: Type,
  catalog: &GitHubAPIResponse,
) -> Vec<String> {
  let prefix = if repository == Type::Girls {
    &*constants::GITHUB_USER_CONTENT
  } else {
    &*boys::GITHUB_USER_CONTENT
  };

  catalog
    .tree
    .iter()
    .filter(|entry| {
      entry.r#type == "blob"
        && entry.path.split_once('/').is_some_and(|(directory, _)| {
          directory.eq_ignore_ascii_case(language)
        })
    })
    .map(|entry| format!("{prefix}{}", entry.path))
    .collect()
}

pub fn random_entry(
  repository: Type,
  catalog: &GitHubAPIResponse,
  random: &mut impl Rng,
) -> Option<SenpyRandom> {
  let available: Vec<_> = languages(catalog)
    .into_iter()
    .filter_map(|language| {
      let images = images(&language, repository, catalog);

      if images.is_empty() {
        None
      } else {
        Some((language, images))
      }
    })
    .collect();

  let (language, images) = available.choose(random)?;
  let image = images.choose(random)?;

  Some(SenpyRandom {
    language: language.clone(),
    image:    image.clone(),
  })
}

pub fn cors() -> Cors {
  Cors::default()
    .with_origins(vec!["*"])
    .with_methods(vec![worker::Method::Get])
}

#[cfg(test)]
mod tests {
  use rand::{rngs::StdRng, SeedableRng};
  use super::*;
  use crate::structures::GitHubAPIResponseTree;

  fn catalog(entries: &[(&str, &str)]) -> GitHubAPIResponse {
    GitHubAPIResponse {
      sha:       "fixture".into(),
      url:       "fixture".into(),
      truncated: false,
      tree:      entries
        .iter()
        .map(|(path, kind)| {
          GitHubAPIResponseTree {
            path: (*path).into(),
            r#type: (*kind).into(),
            ..GitHubAPIResponseTree::default()
          }
        })
        .collect(),
    }
  }

  #[test]
  fn empty_and_single_item_random_selection_are_safe() {
    let mut random = StdRng::seed_from_u64(1);

    assert!(random_entry(Type::Girls, &catalog(&[]), &mut random).is_none());
    assert!(random_entry(
      Type::Girls,
      &catalog(&[("Empty", "tree")]),
      &mut random
    )
    .is_none());

    let one = catalog(&[("Python", "tree"), ("Python/one.png", "blob")]);
    let selected = random_entry(Type::Girls, &one, &mut random).unwrap();

    assert_eq!(selected.language, "Python");
    assert!(selected.image.ends_with("Python/one.png"));
  }

  #[test]
  fn random_selection_can_reach_the_last_language_and_image() {
    let data = catalog(&[
      ("A", "tree"),
      ("A/one.png", "blob"),
      ("A/two.png", "blob"),
      ("B", "tree"),
      ("B/one.png", "blob"),
      ("B/two.png", "blob"),
    ]);
    let mut random = StdRng::seed_from_u64(7);
    let mut selected = std::collections::HashSet::new();

    for _ in 0..100 {
      selected
        .insert(random_entry(Type::Girls, &data, &mut random).unwrap().image);
    }

    assert_eq!(selected.len(), 4);
  }

  #[test]
  fn language_matching_ignores_case_and_excludes_directories() {
    let data = catalog(&[
      ("Python", "tree"),
      ("Python/nested", "tree"),
      ("Python/nested/image.png", "blob"),
      ("README.md", "blob"),
    ]);

    assert_eq!(languages(&data), vec!["Python"]);
    assert_eq!(images("python", Type::Girls, &data).len(), 1);
    assert!(images("unknown", Type::Girls, &data).is_empty());
  }

  #[test]
  fn failed_refresh_preserves_good_data_and_retries_on_schedule() {
    let mut cache = CatalogCache::default();

    assert!(cache.should_refresh());
    assert!(cache.accept(Err("upstream failed".into())).is_err());
    assert!(cache.should_refresh());

    cache.accept(Ok(catalog(&[("Python", "tree")]))).unwrap();

    let stale = cache.accept(Err("upstream failed".into())).unwrap();

    assert_eq!(languages(&stale), vec!["Python"]);

    for _ in 2..50 {
      assert!(!cache.should_refresh());
    }

    assert!(cache.should_refresh());
  }

  #[test]
  fn invalid_or_truncated_responses_are_not_catalogs() {
    assert!(serde_json::from_str::<GitHubAPIResponse>(
      r#"{"message":"API rate limit exceeded"}"#
    )
    .is_err());
    assert!(validate_catalog(catalog(&[])).is_err());

    let mut partial = catalog(&[("Python", "tree")]);

    partial.truncated = true;

    assert!(validate_catalog(partial).is_err());
  }
}
