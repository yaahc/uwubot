FROM rust:latest as builder
WORKDIR /usr/src/uwubot
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/uwubot /usr/local/bin/uwubot
CMD ["uwubot"]
