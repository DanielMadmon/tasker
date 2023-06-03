#!/bin/bash
# a simple installation script for the binaries

INSTALL_TASKER(){
    wget https://github.com/DanielMadmon/tasker/releases/download/linux_release/taskerctl
    wget https://github.com/DanielMadmon/tasker/releases/download/linux_release/tasker_service
    sudo chmod 111 taskerctl
    sudo chmod 111 tasker_service
    mkdir ~.cargo/bin/
    mv tasker_service ~.cargo/bin/
    sudo mv taskerctl /usr/bin/
    taskerctl install
    systemctl --user enable tasker
    sudo systemctl enable tasker
    systemctl --user start tasker
    sudo systemctl start tasker 
}
GET_PM
PM=$?
INSTALL_THEMIS "$PM"
INSTALL_TASKER