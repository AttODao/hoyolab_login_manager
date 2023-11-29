FROM rust:latest AS build
COPY ./ /
WORKDIR /hoyolab_login_manager
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=build /hoyolab_login_manager/target/release/hoyolab_login_manager ./
COPY ./config.yml ./
CMD [ "./hoyolab_login_manager" ]
