FROM debian:latest
RUN apt-get update && apt-get install -y     libstdc++6     libc6 &&     rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin
COPY ./target/x86_64-unknown-linux-musl/release/simple_pastebin .
COPY ./logs/ ./logs/
COPY ./assets/ ./assets/
COPY ./config.toml .
EXPOSE 
CMD ["./simple_pastebin"]
