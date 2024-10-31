#!/bin/bash

# Function to display the menu
function show_menu {
    echo ""
    echo "Commands:"
    echo "1. new <data>"
    echo "2. get <number>"
    echo "3. add <data>"
    echo "4. delete <number>"
    echo "5. clear"
    echo "6. exit"
}

# Ask for the web server link
read -p "Enter the web server link: " server_link

# Main loop
while true; do
    show_menu
    read -p "Enter command: " command arg1

    case $command in
        new)
            read -p "Enter data for new: " data
            curl -X POST -d "$data" "$server_link/new"
            ;;
        get)
            curl "$server_link/raw/$arg1"
            ;;
        add)
            read -p "Enter data to add: " data
            curl -X POST -d "$data" "$server_link"
            ;;
        delete)
            curl -X POST "$server_link/$arg1/delete"
            ;;
        clear)
            curl -X POST "$server_link/clear"
            ;;
        exit)
            echo "Exiting..."
            break
            ;;
        *)
            echo "Invalid command. Please try again."
            ;;
    esac
done
