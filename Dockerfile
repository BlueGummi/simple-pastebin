FROM alpine:latest
RUN apk add --no-cache libstdc++ libc6-compat
WORKDIR /usr/local/bin
COPY ./target/x86_64-unknown-linux-musl/release/simple_pastebin .
COPY ./logs/ ./logs/
COPY ./assets/ ./assets/
COPY ./config.toml .
EXPOSE 6060
CMD ["./simple_pastebin"]
