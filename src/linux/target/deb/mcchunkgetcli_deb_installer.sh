#!/usr/bin/env sh
if [ `id -u` -ne 0 ]; then
    exec sudo "$0"
    if [ $? -gt 0 ]; then
        printf ' %s\n' 'Failed to run script as root.'
        exit 1
    fi
    exit 0
fi
if ! `which curl 2>&1 > /dev/null`; then
    printf ' %s.\n %s:\n %s\n' 'Curl is not installed' \
    'In Debian based systems you can install with' \
    "'sudo apt install curl'"
    printf ' %s\n' 'Would you like to install "curl"?'
    read -p " [`printf "\033[32m%s\033[0m" "Y"`]es or [`printf "\033[31m%s\033[0m" "N"`]o > " install
    case "$install" in
        [Yy]|[Yy][Ee][Ss]) printf ' %s\n' 'Installing "curl".'
            apt install curl -y;;
        *) printf ' %s\n' 'Refused to install "curl".' && exit 2;;
    esac
fi
printf ' Downloading "\033[33m%s\033[0m" to "\033[35m%s\033[0m".\n' 'mcchunkgetcli.deb' '/tmp/mcchunkgetcli.deb'
curl -o /tmp/mcchunkgetcli.deb 'https://raw.githubusercontent.com/Lateralus138/MC-Chunk-Get-V3-CLI/master/src/linux/target/deb/mcchunkgetcli.deb' 
if [ ! -f "/tmp/mcchunkgetcli.deb" ]; then
   printf ' %s "\033[31m%s\033[0m ".\n' 'Failed to download' 'mcchunkgetcli.deb'
   exit 3
fi
printf ' %s "\033[32m%s\033[0m"\n' 'Running' 'dpkg -i /tmp/mcchunkgetcli.deb'
exec dpkg -i /tmp/mcchunkgetcli.deb
