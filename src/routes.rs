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
  structures::SenpyRandom,
  utils::{filter_images_by_language, filter_languages, github_api},
};

pub fn index() -> Result<Response> {
  Response::ok(
    r#"senpy-club/api-worker
=====================

routes
------
if a language requires a parameter, it will be notated like ":this". for
example; if a route is notated as "/v1/route/:parameter", you can access that
route via the url "http://this.domain/v1/route/something".

- /
  - /: index page (you are here)

- /v2
  - /github: github api mirror
  - /languages: a list of all languages that appear in _the_ repository
  - /language/:language: get a list of all images that pertain to the language
    ":language"

notes
-----

contributing
^^^^^^^^^^^^

if you'd like to support the project in any way, check out the repository!
<https://github.com/senpy-club/api-worker>

supporting
^^^^^^^^^^

if you would like to support my development ventures, visit my github profile
`here <https://github.com/fuwn>`_.

license
^^^^^^^

`gnu general public license v3.0 (:code:`gpl-3.0-only`)
<https://github.com/senpy-club/api-worker/blob/main/LICENSE>`_"#,
  )
}

pub async fn github() -> Result<Response> {
  Response::from_json(&github_api().await.unwrap())
}

pub async fn languages() -> Result<Response> {
  Response::from_json(&filter_languages().await)
}

pub async fn language(language: &str) -> Result<Response> {
  Response::from_json(&filter_images_by_language(language).await)
}

pub async fn random() -> Result<Response> {
  let filtered_languages = filter_languages().await;
  let random_language = &filtered_languages
    [thread_rng().gen_range(0..filtered_languages.len() - 1)];
  let filtered_images = filter_images_by_language(random_language).await;
  let random_image = if filtered_images.len() == 1 {
    &filtered_images[0]
  } else {
    &filtered_images[thread_rng().gen_range(0..filtered_images.len() - 1)]
  };

  Response::from_json(&SenpyRandom {
    language: random_language.clone(),
    image:    random_image.clone(),
  })
}
