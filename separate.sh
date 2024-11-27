# !/bin/bash

rm -rf ~/spb && mkdir ~/spb && rm -rf ~/spb/* && cp -r assets ~/spb && cp config.toml ~/spb && cp target/release/simple_pastebin ~/spb && cp pastes.db ~/spb  
