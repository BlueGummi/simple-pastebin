#!/bin/bash

clear_screen() {
    clear
    echo ""
    echo "Commands:"
    echo "  new <optional: main>"
    echo "  get <number or "main">"
    echo "  delete <number or "main">"
    echo "  exit"
}
clear_screen
SERVER_FILE="server.txt"
trim_whitespace() {
    echo "$1" | xargs
}

if [ -f "$SERVER_FILE" ]; then
    SERVER_LINK=$(<"$SERVER_FILE")
    SERVER_LINK=$(trim_whitespace "$SERVER_LINK")

    if [[ -z "$SERVER_LINK" ]]; then
        echo "Error: Server link is empty."
        exit 1
    fi

    echo "Using server link: $SERVER_LINK"
else
    read -p "Enter the pastebin link: " server_link
    server_link=$(trim_whitespace "$server_link")
    echo "$server_link" > "$SERVER_FILE"
    echo "Server link saved to $SERVER_FILE."
    SERVER_LINK="$server_link"
fi

while true; do
    read -p "> " command arg1
    case $command in
        new)
            if [[ "$arg1" == "main" ]]; then
                read -p "Enter data for main: " data
                curl -X POST -d "$data" "$SERVER_LINK"
                echo ""
            else
                read -p "Enter data for new: " data
                curl -X POST -d "$data" "$SERVER_LINK/new"
                echo ""
            fi
            ;;
        get)
            if [[ "$arg1" =~ ^[0-9]+$ ]]; then
                curl "$SERVER_LINK/raw/$arg1"
            elif [[ "$arg1" == "main" ]]; then
                curl "$SERVER_LINK/log"
            else
                echo "Invalid argument for get."
            fi
            echo ""
            ;;
        delete)
            if [[ "$arg1" =~ ^[0-9]+$ ]]; then
                curl -X POST "$SERVER_LINK/$arg1/delete"
            elif [[ "$arg1" == "main" ]]; then
                curl -X POST "$SERVER_LINK/clear"
            else
                echo "Invalid argument for delete."
            fi
            echo ""
            ;;
        exit)
            echo "Exiting..."
            echo ""
            break
            ;;
        *)
            echo "Invalid command. Please try again."
            ;;
    esac
done
