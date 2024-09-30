# simple-pastebin
A simple pastebin server, with a backend written in Rust  🚀 🚀 🚀

## How do I get this to work?

``git clone https://github.com/BlueGummi/simple-pastebin.git``

``cd simple-pastebin``

Edit config.toml with the editor of choice, it currently supports a port IP address, and a time to automatically clear the log file.
Port, IP address, and expiration in config.toml must be strings
Format the expiration like this

"2d6h2m3s"

There is also a 'log_name' variable that allows changing of the log name.
It also has to be a string, like this

"input.txt"

``cargo run --release``

navigate to the IP address and port in your browser

this is also compatible with cURL

to paste data,
``curl -X POST -d "text here" <server name>``

to read the pastebin,
``curl <server name>/<log name> # an example is 127.0.0.1:5050/input.log``

to delete the pastebin,
``curl -X POST <server name>/clear``
