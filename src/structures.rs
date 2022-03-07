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

#![allow(clippy::used_underscore_binding)]

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
