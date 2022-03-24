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

use std::lazy::SyncLazy;

pub const GITHUB_REPOSITORY: &str =
  "flyingcakes85/Anime-Boys-Holding-Programming-Books";

pub static GITHUB_USER_CONTENT: SyncLazy<String> = SyncLazy::new(|| {
  format!(
    "https://raw.githubusercontent.com/{}/master/",
    GITHUB_REPOSITORY
  )
});
pub static GITHUB_API_ENDPOINT: SyncLazy<String> = SyncLazy::new(|| {
  format!(
    "https://api.github.com/repos/{}/git/trees/main?recursive=1",
    GITHUB_REPOSITORY,
  )
});
