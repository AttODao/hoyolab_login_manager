FROM rust:alpine AS build
COPY ./ /
RUN apk update && apk add gcc
WORKDIR /hoyolab_login_manager
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=build /hoyolab_login_manager/target/release/hoyolab_login_manager ./
COPY ./config.yml ./
CMD [ "./hoyolab_login_manager" ]
