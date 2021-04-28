// Copyleft (ɔ) 2021-2021 The Senpy Club
// SPDX-License-Identifier: GPL-3.0-only

use crate::{
  constants::{GITHUB_API_ENDPOINT, GITHUB_USER_CONTENT, USER_AGENT},
  structures::GitHubAPIResponse,
};

pub async fn github_api() -> Result<GitHubAPIResponse, reqwest::Error> {
  Ok(
    reqwest::Client::new()
      .get(GITHUB_API_ENDPOINT)
      .header("User-Agent", USER_AGENT)
      .header(
        "Authorization",
        format!("token {}", std::env::var("GITHUB_TOKEN").unwrap()),
      )
      .send()
      .await?
      .json::<GitHubAPIResponse>()
      .await?,
  )
}

pub async fn filter_languages() -> Vec<String> {
  let mut languages = vec![];

  for i in github_api().await.unwrap().tree {
    if i._type == "tree" {
      languages.push(i.path);
    }
  }

  languages
}

pub async fn filter_images_by_language(language: String) -> Vec<String> {
  let mut images = vec![];

  for i in github_api().await.unwrap().tree {
    // Example:
    //  "Language/Image.png" would become ["Language", "Image.png"]

    // TODO: Fix this with type_ascription
    let x: Vec<&str> = i.path.split("/").collect();
    if x[0] == language && i.path.contains('/') {
      images.push(format!("{}{}", GITHUB_USER_CONTENT, i.path))
    }
  }

  images
}
