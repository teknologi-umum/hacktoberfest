FROM rust:1.72.1-bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y curl

WORKDIR /app

COPY --from=builder /app/target/release/hacktoberfest hacktoberfest
COPY --from=builder /app/target.yml target.yml

EXPOSE 8080

ENV CONFIG_PATH=/app/target.yml

CMD ["/app/hacktoberfest"]