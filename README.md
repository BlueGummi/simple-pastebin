# simple-pastebin
A simple pastebin server, with a backend written in Rust  🚀 🚀 🚀

## How do I get this to work?

``git clone https://github.com/BlueGummi/simple-pastebin.git``

``cd simple-pastebin``

Edit config.toml with the editor of choice, it currently supports a port and IP address.
port and IP address in config.toml must be strings

``cargo run --release``

navigate to the IP address and port in your browser

this is also compatible with cURL

to paste data,
``curl -X POST -d "text here" <server name>``

to read the pastebin,
``curl <server name>/input.log``

to delete the pastebin,
``curl -X POST <server name>/clear``
