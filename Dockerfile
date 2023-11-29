FROM rust:alpine AS build
COPY ./ /
WORKDIR /hoyolab_login_manager
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=build /hoyolab_login_manager/target/release/hoyolab_login_manager ./
CMD [ "./hoyolab_login_manager" ]
