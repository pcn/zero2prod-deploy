FROM lukemathwalker/cargo-chef AS planner

WORKDIR app
COPY . .
# compute a lock-like file for the project
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef AS cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
# Build project dependencies, not the application
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust AS builder
WORKDIR app
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod

FROM debian:buster-slim AS runtime
WORKDIR app
# ENV APP_ENVIRONMENT production
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
# And configuration files
COPY configuration configuration
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./zero2prod"]
