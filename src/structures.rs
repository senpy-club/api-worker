// Copyright (C) 2021-2021 the senpy club
// SPDX-License-Identifier: GPL-3.0-only

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubAPIResponse {
  pub sha:       String,
  pub url:       String,
  pub tree:      Vec<GitHubAPIResponseTree>,
  pub truncated: bool,
}
impl Default for GitHubAPIResponse {
  fn default() -> Self {
    GitHubAPIResponse {
      sha:       "rate limited".to_string(),
      url:       "rate limited".to_string(),
      tree:      vec![],
      truncated: false,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubAPIResponseTree {
  pub path:  String,
  pub mode:  String,
  #[serde(rename = "type")]
  pub _type: String,
  pub sha:   String,
  pub url:   String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SenpyRandom {
  pub language: String,
  pub image:    String,
}
