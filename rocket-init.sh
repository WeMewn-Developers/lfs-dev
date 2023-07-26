#!/bin/bash

BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Waiting for Database to Initialize${NC}"
sleep 5

echo -e "${BLUE}Initializing File Structure${NC}"
for file_type in "wav" "mp3" "ogg" "aac" "png" "jpeg"; do
    mkdir -p "/upload/${file_type}"
done

echo -e "${BLUE}Starting File Server${NC}"
/app/rocket_lfs