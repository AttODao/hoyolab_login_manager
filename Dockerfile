FROM rust:latest AS build
COPY ./ /
RUN apk update && apk add gcc
WORKDIR /hoyolab_login_manager
RUN cargo build --release

FROM alpine:bullseye-slim
WORKDIR /app
COPY --from=build /hoyolab_login_manager/target/release/hoyolab_login_manager ./
COPY ./config.yml ./
CMD [ "./hoyolab_login_manager" ]
