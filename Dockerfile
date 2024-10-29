FROM debian:latest
RUN apt-get update && apt-get install -y     libstdc++6     libc6 &&     rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin
COPY ./target/release/simple_pastebin .
COPY ./logs/ ./logs/
COPY ./assets/ ./assets/
COPY ./config.toml .
EXPOSE 8080
CMD ["./simple_pastebin"]
