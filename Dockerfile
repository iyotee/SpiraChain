FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/spira /usr/local/bin/
COPY --from=builder /app/crates/spirapi /app/crates/spirapi
COPY scripts/ /app/scripts/

RUN pip3 install --no-cache-dir -r /app/crates/spirapi/requirements.txt

EXPOSE 30333 9615

ENTRYPOINT ["spira"]
CMD ["node", "--validator"]

