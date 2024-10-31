#!/bin/bash

echo ""
echo "Commands:"
echo "  new"
echo "  get <number>"
echo "  add"
echo "  delete <number>"
echo "  view"
echo "  clear"
echo "  exit"
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
            read -p "Enter data for new: " data
            curl -X POST -d "$data" "$SERVER_LINK/new"
            echo ""
            ;;
        get)
            curl "$SERVER_LINK/raw/$arg1"
            echo ""
            ;;
        add)
            read -p "Enter data to add: " data
            curl -X POST -d "$data" "$SERVER_LINK"
            echo ""
            ;;
        delete)
            curl -X POST "$SERVER_LINK/$arg1/delete"
            echo ""
            ;;
        clear)
            curl -X POST "$SERVER_LINK/clear"
            echo ""
            ;;
        view)
            curl "$SERVER_LINK/log"
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
