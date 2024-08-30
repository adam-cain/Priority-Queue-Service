FROM rust:1.67 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/priority-queue-service /usr/local/bin/priority-queue-service
CMD ["priority-queue-service"]