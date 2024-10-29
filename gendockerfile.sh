#!/bin/bash
get_port() {
    awk -F '=' '/^port/ {gsub(/[^0-9]/, "", $2); print $2}' config.toml
}
PORT=$(get_port)

if [[ -z "$PORT" ]]; then
    echo "Error: Port not found in config.toml."
    PORT=6060
    exit 1
fi
ARCH=$(uname -m)
cat <<EOF > Dockerfile
FROM debian:latest
RUN apt-get update && apt-get install -y \
    libstdc++6 \
    libc6 && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin
COPY ./target/release/simple_pastebin .
COPY ./logs/ ./logs/
COPY ./assets/ ./assets/
COPY ./config.toml .
EXPOSE ${PORT}
CMD ["./simple_pastebin"]
EOF

echo "Dockerfile generated successfully."
