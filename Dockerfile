FROM rust:1.93-bookworm AS builder

WORKDIR /workspace
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates procps \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home appuser

WORKDIR /app
COPY --from=builder /workspace/target/release/rust-mcp-azure /usr/local/bin/rust-mcp-azure

ENV PORT=8080
ENV RUST_LOG=info
ENV MCP_SANDBOX_ROOT=/app

EXPOSE 8080
USER appuser

ENTRYPOINT ["/usr/local/bin/rust-mcp-azure"]