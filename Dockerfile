FROM rust:alpine as build
RUN apk update && apk add alpine-sdk libressl-dev
COPY ./ /hoyolab_login_manager
WORKDIR /hoyolab_login_manager
RUN cargo build --release

FROM alpine:latest
RUN apk update && apk add tzdata && \
  cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && \
  echo "Asia/Tokyo" > /etc/timezone && \
  apk del tzdata
COPY --from=build /hoyolab_login_manager/target/release/hoyolab_login_manager /
CMD ["./hoyolab_login_manager"]
