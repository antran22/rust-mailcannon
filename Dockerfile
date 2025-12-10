FROM rust:1.91.1 AS build

RUN cargo install cargo-chef sccache --locked
ENV RUSTC_WRAPPER=sccache \
    SCCACHE_DIR=/sccache
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json
COPY . .
# We'll use the release profile to make it faaaast 
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM ubuntu:24.04 AS runtime
RUN groupadd -g 1001 appgroup && \
    useradd -u 1001 -g appgroup -m -d /home/appuser -s /bin/bash appuser
COPY --from=build --chown=appuser:appgroup /app/target/release/mailcannon /usr/local/bin/mailcannon
USER appuser
ENTRYPOINT ["/usr/local/bin/mailcannon"]

