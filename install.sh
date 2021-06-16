#!/bin/bash

DIR="mnt"

if [ ! -d $DIR ];then
    mkdir -p $DIR/EFI/BOOT
fi

if [ -n "$1" ]; then
    cp $1 $DIR/EFI/BOOT/BOOTX64.EFI
else
    echo "No loader file."
    exit
fi

if [ -n "$2" ]; then
    cp $2 $DIR
else
    echo "No kernel file."
    exit
fi

