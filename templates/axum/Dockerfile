# env development
FROM rust:1.66.0-slim as development
WORKDIR /opt/app
RUN apt update -qq && apt install -y libssl-dev build-essential libpq-dev vim apt-transport-https
RUN cargo install cargo-watch
RUN cargo install sqlx-cli
RUN rustup component add rustfmt
COPY . .

# env build
FROM development as build
RUN cargo build --release

# env production
FROM rust:1.66.0-slim as production
RUN apt update -qq && apt install -y build-essential libpq-dev apt-transport-https
COPY --from=build /app/target/release/rust-axum-api .
EXPOSE 3000
CMD ["./rust-axum-api"]
