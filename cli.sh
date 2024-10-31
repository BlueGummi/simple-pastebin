#!/bin/bash

help() {
	echo ""
	echo "Commands:"
	echo "  new <optional: 'main'> [data]"
	echo "  get <number or 'main'>"
	echo "  delete <number or 'main'>"
	echo "  clear"
	echo "  exit"
	echo "  help"
	echo "  cmd '<command>'"
	echo "  history"
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
		echo "Error: Server link is empty."
		exit 1
	fi

	echo "Using server link: $SERVER_LINK"
else
	read -p "Enter the pastebin link: " server_link
	server_link=$(trim_whitespace "$server_link")
	echo "$server_link" >"$SERVER_FILE"
	echo "Server link saved to $SERVER_FILE."
	SERVER_LINK="$server_link"
fi

history=()
history_index=0

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
			echo "Enter data to paste to main (press Enter on an empty line to submit):"
		else
			echo "Enter data to paste to new (press Enter on an empty line to submit):"
		fi

		data=""
		while true; do
			IFS= read -r line
			if [[ -z "$line" ]]; then
				break
			fi
			data+="$line"$'\n'
		done

		# Post the data to the server
		if [[ "$arg1" == "main" ]]; then
			curl -X POST -d "$data" "$SERVER_LINK"
		else
			curl -X POST -d "$data" "$SERVER_LINK/new"
		fi
		echo ""
		;;
	get)
		if [[ "$arg1" =~ ^[0-9]+$ ]]; then
			response=$(curl -s "$SERVER_LINK/raw/$arg1")
			echo "$response"
		elif [[ "$arg1" == "main" ]]; then
			response=$(curl -s "$SERVER_LINK/log")
			echo "$response"
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
	help)
		help
		echo ""
		;;
	cmd)
		bash -c "$arg1"
		;;
	history)
		for i in "${!history[@]}"; do
			echo "$i: ${history[$i]}"
		done
		echo ""
		;;
	*)
		echo "Invalid command. Please try again."
		;;
	esac
done
