// Copyleft (ɔ) 2021-2021 The Senpy Club
// SPDX-License-Identifier: GPL-3.0-only

use actix_web::{HttpRequest, HttpResponse};
use rand::{thread_rng, Rng};

use crate::{
  structures::SenpyRandom,
  utils::{filter_images_by_language, filter_languages, github_api},
};

#[get("/")]
pub fn index() -> HttpResponse {
  HttpResponse::Ok().body(
    r#"# senpy-api
## routes
if a language requires a parameter, it will be notated like <this>.
for example; if a route is notated as "/api/v1/route?<parameter>", you can
access that route via the url
"http://this.domain/api/v1/route?parameter=something"

- /
    - /: index page (you are here)

- /api/v1
    - /github: github api mirror
    - /languages: a list of all languages that appear in _the_ repository
    - /language?<lang>: get a list of all images that pertain to the language "<lang>"

## notes
### rate-limit (s)
there aren't any rate-limits or whatnot on api usage but don't abuse it, it only takes one bad
apple to spoil the lot.

### contributing
if you'd like to support the project in anyway, check out the repository!
https://github.com/senpy-club/api

### supporting
if you would like to support my development ventures, visit my github profile here :3
https://github.com/fuwn

### license
gnu general public license v3.0 (gpl-3.0-only)
https://github.com/senpy-club/api/blob/main/license"#,
  )
}

#[get("/github")]
pub async fn github() -> HttpResponse { HttpResponse::Ok().json(github_api().await.unwrap()) }

#[get("/languages")]
pub async fn languages() -> HttpResponse { HttpResponse::Ok().json(filter_languages().await) }

#[get("/language")]
pub async fn language(req: HttpRequest) -> HttpResponse {
  let queries = qstring::QString::from(req.query_string());
  let lang = queries.get("lang").unwrap_or("null");

  return if lang == "null" {
    HttpResponse::Ok().json(vec!["invalid language or no language specified".to_string()])
  } else {
    HttpResponse::Ok().json(filter_images_by_language(lang).await)
  };
}

#[get("/random")]
pub async fn random() -> HttpResponse {
  let filtered_languages = filter_languages().await;
  let random_language =
    &filtered_languages[thread_rng().gen_range(0..filtered_languages.len() - 1)];
  let filtered_images = filter_images_by_language(random_language).await;
  let random_image = &filtered_images[thread_rng().gen_range(0..filtered_images.len() - 1)];

  HttpResponse::Ok().json(SenpyRandom {
    language: random_language.clone(),
    image:    random_image.clone(),
  })
}
