#!/bin/bash

DIR="bios"

if [ ! -d $DIR ];then
    mkdir $DIR
fi
cd bios
if [ ! -e "RELEASEX64_OVMF.fd" ];then
    wget "https://github.com/retrage/edk2-nightly/raw/master/bin/RELEASEX64_OVMF.fd"
fi

