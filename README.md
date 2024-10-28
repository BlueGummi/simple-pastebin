# simple-pastebin

A simple pastebin webserver, with a backend written in Rust  🚀 🚀 🚀

## How do I get this to work?

Before continuing with the rest of the setup steps, clone the repository and `cd` inside it.

`git clone https://github.com/BlueGummi/simple-pastebin.git`


`cd simple-pastebin`


## Configuration

This webserver can be configured by editing fields in `config.toml`

### `config.toml` fields

The IP address the server will be hosted on:

`address` - Must be a string, e.g. `"0.0.0.0"`

The port that the server will be hosted on:

`port` - Must be an integer, e.g. `80`

The time until the input log will auto-delete:

`expiration` - Must be a string, e.g. `"50d"`


`expiration` supports days, hours, minutes, and seconds. e.g. `"50d4h10m2s`

The name of the log where user input is stored:

`log_name` - Must be a string, e.g. `"logs/input.log"`

Whether or not the user input is displayed on the server-side:

`display_data` - Must be a boolean, e.g. `true`

Whether or not server information will be displayed on the server-side:

`display_info` - Must be a boolean, e.g. `true`

Do not log any data, all user input does not get sent to the server:

`void_mode` - Must be a boolean, e.g. `false`

Whether or not to keep a history log file that stores log information:

`history` - Must be a boolean, e.g. `true`

The log location of the history, must be set even if history is false:

`history_log` - Must be a string, e.g. `"logs/history.log"`

Missing fields will be set to their default values.

## Compiling/Running

To compile for the host machine architecture, `cargo` must be installed, along with the Rust compiler.


`cargo run --release`

Navigate to the IP address and port in your browser

### CLI

This webserver is also compatible with cURL

To paste data,

`curl -X POST -d "text here" <server name> # e.g. curl -X POST -d "Hello, world!" http://127.0.0.1:6060`

To read the current contents of the pastebin,

`curl <server name>/<log name> # e.g. 127.0.0.1:6060/input.log`

To delete the pastebin contents,

`curl -X POST <server name>/clear # e.g. curl -X POST 127.0.0.1:6060/clear`



# Running in docker

To run this in docker, please make sure the docker daemon is running and docker is installed.

To compile for a musl docker image (it's Alpine Linux),

`rustup target add x86_64-unknown-linux-musl`



`cargo build --release --target=x86_64-unknown-linux-musl`


To build the docker image,

`docker build -t simple_pastebin .`

To run the docker image,

`docker run -p HOST_PORT:CONTAINER_PORT simple_pastebin # e.g. docker run -p 8080:6060 simple_pastebin`

## Notice:

In the `Dockerfile`, there is a field, `EXPOSE`. Please make sure that the value matches the value for the `port` in `config.toml`. e.g., if the `port` in `config.toml` is `8080`, set the `EXPOSE` value in `Dockerfile` to `8080`.
