# Configuration

This webserver is able to take CLI arguments, as well as reading off of `config.toml`, with CLI arguments taking priority.
(e.g., passing `--port 1234` will override `port = 9876` in `config.toml`)

## Fields

All fields that are present in config.toml can also be passed as CLI arguments.

All fields are optional, and they each have their default value.

| Field       | In config file | CLI arguments              | Variable type | Default value |
| :---------- | :--------------| :------------------------- | :------------ | :-----------: |
| Address     | `address`      | `--address` **or** `-a`    | String        | `"0.0.0.0"`   |
| Port        | `port`         | `--port` **or** `-p`       | Integer       | `6060`        |
| Expiration  | `expiration`   | `--expiration` **or** `-e` | String        | `"10m"`       |
| Log Name    | `log_name`     | `--log_name` **or** `-l`   | String        | `"input.log"` |
| Void mode   | `void_mode`    | `--void_mode`              | Boolean       | N/A           |
| History     | `history`      | `--history`                | Boolean       | N/A           |
| History log | `history_log`  | `--history_log`            | String        | N/A           |
| Log level   | `log_level`    | `--log_level`              | String        | `"info"`      |

### Address:

Defines the **IP address** for the server to *bind to*. If the address **cannot be bound**, the server will gracefully shutdown.

### Port:

Defines the **port** for the server to listen on. If the port **cannot be bound**, the server will gracefully shutdown.


### Expiration:

Defines the **time after which** the **main** pastebin is cleared. Days, hours, minutes, and seconds all are accepted.


e.g. `20d5h2m1s`


### Log name:

Defines the **name** of the main pastebin. If it does not exist, the file will be created.


e.g. `logs/input.log`


### Void mode:

Defines whether or not the pastebin will **send no data to the server**. 


### History:

Defines whether or not the **server will log information** to a file, defined by `history_log`.


### History log:

Defines the **name of the log** that the history will be sent to, if `history` is true.


### Log level: 

Defines the **level of logging** to the terminal on the server. Can be set to either error or info (uses the `log` crate for **Rust**).
