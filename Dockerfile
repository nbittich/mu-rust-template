FROM rust:1.69 as builder

WORKDIR /app

RUN cargo new mu-rust-template

WORKDIR /app/mu-rust-template

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build --release 

RUN rm -rf ./src

COPY ./src/ ./src

RUN rm ./target/release/deps/mu_rust_template*

RUN cargo build --release 

FROM debian:bullseye-slim AS runtime
RUN apt  update && apt upgrade -y
RUN apt install -y ca-certificates 

# Set timezone
ENV TZ="Europe/Brussels"

ENV RUST_LOG=info

COPY --from=builder  /app/mu-rust-template/target/release/mu-rust-template .

ENTRYPOINT [ "./mu-rust-template" ]
