#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Colors

PATH_TO_INSTALL="/usr/local/bin"

if [[ $(id -u) -ne 0 ]]; then
    echo -e "${RED}Please run as root${NC}"
    exit 1
fi

echo -e "${BLUE}Installing sbbw${NC}"
sleep 2

if [[ -f "$PATH_TO_INSTALL/sbbw" ]]; then
    echo -e "${RED}sbbw already installed${NC}"
    exit 1
fi

if [[ ! -f "sbbw" ]]; then
    echo -e "${RED}sbbw binary not found${NC}"
    exit 1
fi
if [[ ! -f "sbbw-widget" ]]; then
    echo -e "${RED}sbbw-widget binary not found${NC}"
    exit 1
fi

chmod +x sbbw
chmod +x sbbw-widget
echo -e "[ ${GREEN}✓${NC} ] ${GREEN}Made executable${NC}"

sleep 2

cp {sbbw,sbbw-widget} $PATH_TO_INSTALL
echo -e "[ ${GREEN}✓${NC} ] ${GREEN}Sbbw Installed${NC}"
sleep 2

echo -e "[ ${GREEN}✓${NC} ] ${GREEN}Done${NC}"
