FROM rust:1.78 as build

WORKDIR /
COPY ./src ./src
COPY ./config ./config
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock


RUN cargo build --release 

FROM debian:stable-slim
EXPOSE 80

COPY --from=build ./target/release/ ./target/release/
COPY --from=build ./src/data/templates/ ./src/data/templates/
COPY --from=build ./src/data/resource/ ./src/data/resource/
COPY --from=build ./config/ ./config/

ENTRYPOINT ["/target/release/ziplinks"]