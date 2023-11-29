FROM rust:alpine as build
RUN apk update && apk add alpine-sdk libressl-dev
COPY ./ /hoyolab_login_manager
WORKDIR /hoyolab_login_manager
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=build /hoyolab_login_manager/target/release/hoyolab_login_manager ./
CMD ["./hoyolab_login_manager"]
