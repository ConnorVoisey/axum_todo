FROM rust:1.73-alpine as build

RUN apk update
RUN apk add libressl-dev
#RUN apk add libpq-dev

RUN apk add --update alpine-sdk
RUN apk add pkgconfig

RUN USER=root cargo new --bin axum_todo
WORKDIR /axum_todo
RUN cargo install sqlx-cli

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./.sqlx ./.sqlx
ENV SQLX_OFFLINE true
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src
COPY ./.env ./.env
RUN cargo build --release

FROM alpine:3.16.0 as runtime

COPY --from=build /axum_todo/target/release/axum_todo ./target/release/axum_todo
RUN touch .env

ENTRYPOINT [ "/target/release/axum_todo" ]
#ENTRYPOINT ["tail", "-f", "/dev/null"]
