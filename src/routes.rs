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

use rand::{thread_rng, Rng};
use worker::{Response, Result};

use crate::{
  structures::{SenpyRandom, Type},
  utils::{cors, filter_images_by_language, filter_languages, github_api},
};

pub fn index() -> Result<Response> {
  Response::ok(&*crate::constants::INDEX)?.with_cors(&cors())
}

pub async fn github(repository: Type) -> Result<Response> {
  Response::from_json(&github_api(repository).await.unwrap())?
    .with_cors(&cors())
}

pub async fn languages(repository: Type) -> Result<Response> {
  Response::from_json(&filter_languages(repository).await)?.with_cors(&cors())
}

pub async fn language(language: &str, repository: Type) -> Result<Response> {
  Response::from_json(&filter_images_by_language(language, repository).await)?
    .with_cors(&cors())
}

pub async fn random(repository: Type) -> Result<Response> {
  let filtered_languages = filter_languages(repository.clone()).await;
  let random_language = &filtered_languages
    [thread_rng().gen_range(0..filtered_languages.len() - 1)];
  let filtered_images =
    filter_images_by_language(random_language, repository).await;
  let random_image = if filtered_images.len() == 1 {
    &filtered_images[0]
  } else {
    &filtered_images[thread_rng().gen_range(0..filtered_images.len() - 1)]
  };

  Response::from_json(&SenpyRandom {
    language: random_language.clone(),
    image:    random_image.clone(),
  })?
  .with_cors(&cors())
}
