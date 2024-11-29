# simple-pastebin

A *simple pastebin* webserver, with a backend written in Rust  🚀 🚀 🚀 and Python frontend CLI

## Quickstart

To get started, **RustC** and **Cargo** must be installed, as well as **Python** if the frontend CLI is to be used.


```
git clone https://github.com/BlueGummi/simple-pastebin
```


```
cargo run --release
```


This will **start** the server on the **default IP Address and port**, being `0.0.0.0:6060`.

## Use

This pastebin has a **CLI** (written in Python), and a **Web UI**, which can be accessed by visiting the address of the server.

Pastes can **either** be sent to the **main pastebin**, or they can be sent to a **new paste**. Links will be embedded.

### CLI

This server features a **CLI client**, which can be run via `python cli.py`

For a quickstart, `help` can be entered **in the CLI**.

## Further documentation:

### Configuration - [docs/configuring.md](https://github.com/BlueGummi/simple-pastebin/blob/main/docs/configuring.md)

### Docker - [docs/docker.md](https://github.com/BlueGummi/simple-pastebin/blob/main/docs/docker.md)

### Maintenance - [docs/maintenance.md](https://github.com/BlueGummi/simple-pastebin/blob/main/docs/maintenance.md)

### CLI - [docs/cli.md](https://github.com/BlueGummi/simple-pastebin/blob/main/docs/cli.md)
