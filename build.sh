#!/bin/bash
# a simple installation script for the binaries

GET_PM(){
    declare -A osInfo;
osInfo[/etc/redhat-release]=yum
osInfo[/etc/arch-release]=pacman
osInfo[/etc/gentoo-release]=emerge
osInfo[/etc/SuSE-release]=zypp
osInfo[/etc/debian_version]=apt-get
osInfo[/etc/alpine-release]=apk

for f in "${!osInfo[@]}"
do
    if [[ -f $f ]];then
        if [ "${osInfo[$f]}" == "yum" ];then
            return 0
        fi
        if [ "${osInfo[$f]}" == "pacman" ];then
            return 1
        fi
        if [ "${osInfo[$f]}" == "emerge" ];then
            return 2
        fi
        if [ "${osInfo[$f]}" == "zypp" ];then
            return 3
        fi
        if [ "${osInfo[$f]}" == "apt-get" ];then
            return 4
        fi
        if [ "${osInfo[$f]}" == "apk" ];then
            return 5
        fi
    fi
done
}
INSTALL_THEMIS_FROM_SOURCE(){
    git clone https://github.com/cossacklabs/themis.git
    cd themis || exit
    make
    sudo make install
}
INSTALL_THEMIS(){
    if [ "$1" == 0 ]; then
        sudo rpm --import https://pkgs-ce.cossacklabs.com/gpg
        wget -qO - https://pkgs-ce.cossacklabs.com/stable/centos/cossacklabs.repo | sudo tee /etc/yum.repos.d/cossacklabs.repo
        sudo yum install libthemis-devel
    fi
    if [ "$1" == 1 ]; then
        git clone https://aur.archlinux.org/libthemis.git
        cd libthemis || exit
        makepkg -si
        cd .. || exit
        rm -r libthemis
    fi
    if [ "$1" == 2 ]; then
        INSTALL_THEMIS_FROM_SOURCE
    fi
    if [ "$1" == 3 ]; then
        INSTALL_THEMIS_FROM_SOURCE
    fi
    if [ "$1" == 4 ]; then
        wget -qO - https://pkgs-ce.cossacklabs.com/gpg | sudo apt-key add -
        sudo apt install apt-transport-https
        sudo touch /etc/apt/sources.list.d/cossacklabs.list
        echo "deb https://pkgs-ce.cossacklabs.com/stable/debian focal main" | sudo tee /etc/apt/sources.list.d/cossacklabs.list
        sudo apt update
        sudo apt install libthemis-dev
    fi
    if [ "$1" == 5 ]; then
        INSTALL_THEMIS_FROM_SOURCE
    fi
}
BUILD_TASKER(){
    git clone https://github.com/DanielMadmon/tasker.git
    cd tasker || exit
    cargo build --release
}
GET_PM
PM=$?
INSTALL_THEMIS "$PM"
INSTALL_TASKER