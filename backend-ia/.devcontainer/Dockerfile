FROM mcr.microsoft.com/devcontainers/rust:latest

USER root

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    git \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*