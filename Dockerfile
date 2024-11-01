FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install libc6 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/bin
COPY assets/ ./assets/
COPY target/release/simple_pastebin .
COPY pastes.db .
COPY config.toml .
CMD ["./simple_pastebin"]
