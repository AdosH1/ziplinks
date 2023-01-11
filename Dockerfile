FROM rust:1.66.0 as build

WORKDIR /
COPY . .

RUN cargo build --release

FROM debian:buster-slim
COPY --from=build ./target/release/ziplinks ./target/release/ziplinks
COPY --from=build ./src/data/templates/ ./src/data/templates/
COPY --from=build ./src/data/resource/ ./src/data/resource/

CMD ["/target/release/ziplinks"]