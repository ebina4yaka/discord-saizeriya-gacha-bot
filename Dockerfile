FROM rust:1.78.0-alpine3.19 as builder
WORKDIR /build
COPY . .
RUN apk --no-cache add musl-dev
RUN cargo build --release


FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY .env .
COPY --from=builder /build/target/release/discord-saizeriya-gacha-bot .
COPY --from=builder /build/src/data/menu.json .
CMD ["./discord-saizeriya-gacha-bot"]
