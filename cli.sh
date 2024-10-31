#!/bin/bash

# Function to display the menu
echo ""
echo "Commands:"
echo "  new"
echo "  get <number>"
echo "  add"
echo "  delete <number>"
echo "  view"
echo "  clear"
echo "  exit"


# Ask for the web server link
read -p "Enter the pastebin link: " server_link

# Main loop
while true; do
    read -p "> " command arg1

    case $command in
        new)
            read -p "Enter data for new: " data
            curl -X POST -d "$data" "$server_link/new"
	    echo ""
            ;;
        get)
            curl "$server_link/raw/$arg1"
	    echo ""
            ;;
        add)
            read -p "Enter data to add: " data
            curl -X POST -d "$data" "$server_link"
	    echo ""
            ;;
        delete)
            curl -X POST "$server_link/$arg1/delete"
	    echo ""
            ;;
        clear)
            curl -X POST "$server_link/clear"
	    echo ""
            ;;
	view)
	    curl "$server_link/log"
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
