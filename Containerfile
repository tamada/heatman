FROM rust:1-bullseye AS builder

COPY . /app
WORKDIR /app
RUN    cargo build --release

FROM debian:bullseye-slim

ARG VERSION=0.1.0

LABEL   org.opencontainers.image.source=https://github.com/tamada/heatman \
        org.opencontainers.image.version=${VERSION} \
        org.opencontainers.image.title=heatman \
        org.opencontainers.image.description="Create heat map image from given csv file"

RUN    adduser --disabled-password --disabled-login --home /opt/heatman nonroot \
    && apt-get update \
    && apt-get install --no-install-recommends -y $APT_OPTIONAL ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/heatman /opt/heatman/heatman

RUN  chown -R nonroot:nonroot /opt/heatman

USER nonroot

WORKDIR /app

ENTRYPOINT [ "/opt/heatman/heatman" ]
