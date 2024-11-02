FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install libc6 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
WORKDIR ~

COPY assets/ ./assets/

RUN echo 'Assets directory copied.'
COPY target/release/simple_pastebin .

RUN echo 'Binary copied.'

COPY pastes.db* .
COPY config.toml* .

CMD ["./simple_pastebin"]
