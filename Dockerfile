FROM rust:latest as build
COPY ./ /
WORKDIR /hoyolab_login_manager
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=build /usr/local/cargo/bin/hoyolab_login_manager /usr/local/bin/hoyolab_login_manager
RUN ["hoyolab_login_manager"]
