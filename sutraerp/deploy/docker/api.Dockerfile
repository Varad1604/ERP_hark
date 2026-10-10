# syntax=docker/dockerfile:1.7
# ---- build ----
FROM rust:1-bookworm AS build
WORKDIR /src
ENV SQLX_OFFLINE=true CARGO_INCREMENTAL=0
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY .cargo .cargo
COPY crates crates
COPY modules modules
COPY apps/api apps/api
COPY migrations migrations
COPY catalog catalog
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked -p sutra-api && cp target/release/sutra-api /sutra-api

# ---- runtime ----
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /sutra-api /usr/local/bin/sutra-api
ENV SUTRA_ENV=production SUTRA_HTTP_ADDR=0.0.0.0:8080 SUTRA_LOG_FORMAT=json
EXPOSE 8080
USER nonroot
ENTRYPOINT ["/usr/local/bin/sutra-api"]
CMD ["serve"]
