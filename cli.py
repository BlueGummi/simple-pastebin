import os
import requests
import signal
import sys

RED = '\033[0;31m'
GREEN = '\033[0;32m'
YELLOW = '\033[0;33m'
BLUE = '\033[0;34m'
CYAN = '\033[0;36m'
BOLD = '\033[1m'
RESET = '\033[0m'

SERVER_FILE = "server.txt"

def help_command():
    print("")
    print(f"{CYAN}{BOLD}Commands:{RESET}")
    print(f"  {GREEN}{BOLD}new{RESET} {BLUE}<optional: 'main'>")
    print(f"  {GREEN}{BOLD}get{RESET} {BLUE}<number or 'main'>")
    print(f"  {GREEN}{BOLD}delete{RESET} {BLUE}<number or 'main'>")
    print(f"  {GREEN}{BOLD}clear{RESET}")
    print(f"  {GREEN}{BOLD}exit{RESET}")
    print(f"  {GREEN}{BOLD}help{RESET}")
    print(f"  {GREEN}{BOLD}cmd {YELLOW}'<command>'")
    print(f"  {GREEN}{BOLD}history{RESET}")

def trim_whitespace(s):
    return ' '.join(s.split())

def signal_handler(sig, frame):
    sys.exit(0)

def main():
    signal.signal(signal.SIGINT, signal_handler)

    os.system('clear')

    if os.path.isfile(SERVER_FILE):
        with open(SERVER_FILE, 'r') as file:
            server_link = trim_whitespace(file.read())
        if not server_link:
            print(f"{RED}{BOLD}Error: Server link is empty.{RESET}")
            return
        print(f"Using server link: {GREEN}{BOLD}{server_link}{RESET}")
    else:
        server_link = input("Enter the pastebin link: ")
        server_link = trim_whitespace(server_link)
        with open(SERVER_FILE, 'w') as file:
            file.write(server_link)
        print(f"{GREEN}{BOLD}Server link saved to {SERVER_FILE}.{RESET}")

    if not server_link.startswith("http://") and not server_link.startswith("https://"):
        server_link = f"http://{server_link}"

    history = []
    while True:
        command_input = input("> ").strip().split()
        if not command_input:
            continue

        command = command_input[0]
        args = command_input[1:]

        if command == 'clear':
            os.system('clear')
        elif command == 'new':
            if args and args[0] == 'main':
                print(f"Enter data to paste to {GREEN}{BOLD}main{RESET} (press Enter on an empty line to submit):")
            else:
                print(f"Enter data to paste to {GREEN}{BOLD}new{RESET} (press Enter on an empty line to submit):")

            data = []
            while True:
                line = input()
                if not line:
                    break
                data.append(line)
            data = '\n'.join(data)

            if args and args[0] == 'main':
                response = requests.post(server_link, data=data)
                print(response.text)
                continue
            else:
                response = requests.post(f"{server_link}/new", data=data)

            if response.ok and "http" in response.text:
                paste_number = response.text.split('/')[-1]
                print(f"{GREEN}{BOLD}Paste {paste_number} created.{RESET}\n")
            else:
                print(f"{RED}{BOLD}Failed to create paste.{RESET}")
        elif command == 'get':
            if args and args[0].isdigit():
                response = requests.get(f"{server_link}/raw/{args[0]}")
                if response.text == "The requested paste does not exist":
                    print(f"{RED}{BOLD}ERR: Paste not found.{RESET}\n")
                else:
                    print(f"{response.text}\n")
            elif args and args[0] == 'main':
                response = requests.get(f"{server_link}/log")
                print(response.text)
            else:
                print(f"{RED}{BOLD}Invalid argument for get.{RESET}")
        elif command == 'delete':
            if args and args[0].isdigit():
                response = requests.post(f"{server_link}/{args[0]}/delete")
                print(response.text)  # Print the server's response
            elif args and args[0] == 'main':
                response = requests.post(f"{server_link}/clear")
                print(response.text)  # Print the server's response
            else:
                print(f"{RED}{BOLD}Invalid argument for delete.{RESET}")
        elif command == 'exit':
            break
        elif command == 'help':
            help_command()
        elif command == 'cmd':
            if args:
                os.system(' '.join(args))
        elif command == 'history':
            for i, item in enumerate(history):
                command_str = ' '.join(item)
                print(f"{i}: {command_str.strip()}")
            print("")
        else:
            print(f"{RED}{BOLD}Invalid command. Please try again.{RESET}")

        history.append(command_input)

if __name__ == "__main__":
    main()
