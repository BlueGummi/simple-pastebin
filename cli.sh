    #!/bin/bash

    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BLUE='\033[0;34m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    RESET='\033[0m'

    help() {
        echo ""
        echo -e "${CYAN}${BOLD}Commands:${RESET}"
        echo -e "  ${GREEN}${BOLD}new${RESET} ${BLUE}<optional: 'main'>"
        echo -e "  ${GREEN}${BOLD}get${RESET} ${BLUE}<number or 'main'>"
        echo -e "  ${GREEN}${BOLD}delete${RESET} ${BLUE}<number or 'main'>"
        echo -e "  ${GREEN}${BOLD}clear${RESET}"
        echo -e "  ${GREEN}${BOLD}exit${RESET}"
        echo -e "  ${GREEN}${BOLD}help${RESET}"
        echo -e "  ${GREEN}${BOLD}cmd ${YELLOW}'<command>'"
        echo -e "  ${GREEN}${BOLD}history${RESET}"
    }

    clear
    SERVER_FILE="server.txt"
    trim_whitespace() {
        echo "$1" | xargs
    }

    if [ -f "$SERVER_FILE" ]; then
        SERVER_LINK=$(<"$SERVER_FILE")
        SERVER_LINK=$(trim_whitespace "$SERVER_LINK")

        if [[ -z "$SERVER_LINK" ]]; then
            echo -e "${RED}${BOLD}Error: Server link is empty.${RESET}"
            exit 1
        fi

        echo -e "Using server link: ${GREEN}${BOLD}$SERVER_LINK${RESET}"
    else
        read -p "Enter the pastebin link: " server_link
        server_link=$(trim_whitespace "$server_link")
        echo "$server_link" >"$SERVER_FILE"
        echo -e "${GREEN}${BOLD}Server link saved to $SERVER_FILE.${RESET}"
        SERVER_LINK="$server_link"
    fi

    history=()
    history_index=-1
    while true; do
        read -e -p "> " command arg1 arg2
        if [[ -n "$command" ]]; then
            history+=("$command $arg1 $arg2")
            history_index=${#history[@]}
        fi
        case $command in
        clear)
            clear
            ;;
        new)
            if [[ "$arg1" == "main" ]]; then
                echo -e "Enter data to paste to ${GREEN}${BOLD}main${RESET} (press Enter on an empty line to submit):"
            else
                echo -e "Enter data to paste to ${GREEN}${BOLD}new${RESET} (press Enter on an empty line to submit):"
            fi

            data=""
            while true; do
                IFS= read -r line
                if [[ -z "$line" ]]; then
                    break
                fi
                data+="$line"$'\n'
            done
            if [[ "$arg1" == "main" ]]; then
                response=$(curl -s -X POST -d "$data" "$SERVER_LINK")
                echo "$response"
                continue
            else
                response=$(curl -s -X POST -d "$data" "$SERVER_LINK/new")
            fi

            if [[ $response =~ http://[^/]+/([0-9]+) ]]; then
                paste_number="${BASH_REMATCH[1]}"
                echo -e "${GREEN}${BOLD}Paste $paste_number created.${RESET}"
            else
                echo -e "${RED}${BOLD}Failed to create paste.${RESET}"
            fi
            echo ""
            ;;
        get)
            if [[ "$arg1" =~ ^[0-9]+$ ]]; then
                response=$(curl -s "$SERVER_LINK/raw/$arg1")
                if [[ "$response" == "The requested paste does not exist" ]]; then
                    echo -e "${RED}${BOLD}ERR: Paste not found.${RESET}"
                else
                    echo "$response"
                fi
            elif [[ "$arg1" == "main" ]]; then
                response=$(curl -s "$SERVER_LINK/log")
                echo "$response"
            else
                echo -e "${RED}${BOLD}Invalid argument for get.${RESET}"
            fi
            echo ""
            ;;
        delete)
            if [[ "$arg1" =~ ^[0-9]+$ ]]; then
                curl -X POST "$SERVER_LINK/$arg1/delete"
            elif [[ "$arg1" == "main" ]]; then
                curl -X POST "$SERVER_LINK/clear"
            else
                echo -e "${RED}${BOLD}Invalid argument for delete.${RESET}"
            fi
            echo ""
            ;;
        exit)
            break
            ;;
        help)
            help
            ;;
        cmd)
            bash -c "$arg1"
            ;;
        history)
            for i in "${!history[@]}"; do
                echo -e "$i: ${history[$i]}"
            done
            echo ""
            ;;
        *)
            echo -e "${RED}${BOLD}Invalid command. Please try again.${RESET}"
            ;;
        esac
    done
