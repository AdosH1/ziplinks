FROM rust:1.66.0 as build

WORKDIR /
COPY . .

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --target=x86_64-unknown-linux-gnu --release

FROM debian:stable-slim
EXPOSE 80

COPY --from=build ./target/x86_64-unknown-linux-gnu/release/ ./target/x86_64-unknown-linux-gnu/release/
COPY --from=build ./src/data/templates/ ./src/data/templates/
COPY --from=build ./src/data/resource/ ./src/data/resource/
COPY --from=build ./config/ ./config/

ENTRYPOINT ["/target/x86_64-unknown-linux-gnu/release/ziplinks"]