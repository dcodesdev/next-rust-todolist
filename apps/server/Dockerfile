FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates libssl-dev pkg-config
RUN rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY .env .env
COPY target/release/server server

RUN chmod +x ./server

CMD ["./server"]