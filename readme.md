<p align="center">
<h1>senpy-api</h1>
</p>

<p align="center">
<a href="https://discord.com/invite/yWKgRT6">
<img src="https://img.shields.io/discord/246524734718738442" alt="discord" />
</a>
<a href="https://www.codefactor.io/repository/github/senpy-club/api">
<img src="https://www.codefactor.io/repository/github/senpy-club/api/badge" alt="codefactor" />
</a>
<a href="https://saythanks.io/to/fuwnzy@gmail.com">
<img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="Say Thanks" />
</a>
<a href="./license">
<img src="https://img.shields.io/github/license/Whirlsplash/whirl" alt="license" />
</a>
</p>

## notice
nix integration is currently broken. until [rocket](https://crates.io/crates/rocket) officially
releases version `0.5.0`, it will stay broken.

## nix
- build: `nix-build senpy-api.nix`
- docker: `nix-build default.nix`

## usage
- run (dev): `ROCKET_ENV=dev cargo run`
- build (prod): `ROCKET_ENV=prod cargo build --release`

also note that this api wrapper leverages the official github api, meaning that rate-limits are
very much present.

if you are going to be self hosting this project, it is highly encouraged that
you generate yourself a github personal access token and set the environment variable `GITHUB_TOKEN`
as your pat.

## contributing
please reference the [contribution guidelines](./contributing.md) of this repository.

### license
[gnu general public license v3.0](https://github.com/senpy-club/api/blob/main/license)
