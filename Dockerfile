# Build the release server binary
FROM rust:1.85-alpine AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY demo-policy.json ./demo-policy.json
RUN cargo build --release --locked --bin server

# Minimal runtime image
FROM alpine:3.20
RUN addgroup -S reflex && adduser -S reflex -G reflex
WORKDIR /app
COPY --from=builder /app/target/release/server /usr/local/bin/reflex-server
COPY --from=builder /app/demo-policy.json /app/demo-policy.json
RUN mkdir -p /app/artifacts && chown -R reflex:reflex /app
ENV REFLEX_ADDR=0.0.0.0:18080
EXPOSE 18080
VOLUME ["/app/artifacts"]
USER reflex
CMD ["reflex-server"]
