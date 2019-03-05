FROM rust:stretch as build
WORKDIR /usr/src/dumbot
COPY . .
RUN cargo install --path . --root .

FROM debian:stretch
RUN apt-get update && apt-get install -y \
    openssl \
    libssl1.1 \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/dumbot/bin/dumbot /
CMD ["/dumbot"]
