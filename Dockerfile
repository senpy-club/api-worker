# # NOTICE
# This Dockerfile is *probably* not stable as of 2021. 04. 26.
#
# Can you use it? Yes, but you probably shouldn't.

FROM rustlang/rust:nightly-slim AS build

WORKDIR /src/senpy-api

COPY . .

RUN cargo build --release

FROM ubuntu:18.04

COPY --from=build /src/senpy-api/target/release/senpy-api /usr/local/bin/senpy-api

CMD ["/usr/local/bin/senpy-api"]
