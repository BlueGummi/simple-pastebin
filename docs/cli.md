# Command-Line interface (client)

## Initialization

This webserver has a **Python frontend CLI**, which can be **executed** with `python cli.py`

The CLI will put the user into a **command-prompt**, where certain commands can be executed.


## Commands

| Command | Arguments | Description | Example |
| :------ | :-------- | :---------- | :------ |
| `new`   | none or `main` | Creates a **new pastebin**, if the command is `new main`, data will be added to the **main pastebin**. This will prompt the user to **input text** when executed | `new` | 
| `get`   | number or `main` | Gets the data of a **paste** or of the **main pastebin**. | `get 42` |
| `delete`| number or `main` | **Deletes** the contents of a pastebin or clears the main pastebin if `main` is specified. | `delete 42` |
| `clear` | none | Clears the screen. | `clear` |
| `exit`  | none | **Exits** the CLI. | `exit | 
| `help   | none | Displays all commands and their arguments. | `help` |
| `cmd`   | a command | Runs a **command in the terminal**. | `cmd whoami` |
| `history` | none | Prints out **all previous commands** executed in the session. | `history` |
| `list`  | number | Prints out **all pastes** from 0 to a given number. | `list 50` |
### Configuration

Configuration for the CLI is ***very*** **minimal**, as only the **server address** is stored in `server.txt` in the directory of the CLI program. If no `server.txt` is detected, the CLI will **prompt the user for a server address**.
