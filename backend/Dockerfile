FROM rust:1.63.0-bullseye AS builder

WORKDIR /app

COPY . .

RUN user=root cargo init
RUN cargo fetch
RUN cargo build --release

FROM debian:bullseye AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/hacktoberfest hacktoberfest

EXPOSE 8080

CMD ["/app/hacktoberfest"]