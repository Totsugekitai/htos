### Makefile_template ###

# don't use TAB
.RECIPEPREFIX = >
# change shell to bash
SHELL := bash
# shell flags
.SHELLFLAGS := -eu -o pipefail -c
# one shell for one target rule
.ONESHELL:
# warning undefined variables
MAKEFLAGS += --warn-undefined-variables
# delete intermediate files on error
.DELETE_ON_ERROR:
# delete implicit rules
MAKEFLAGS += -r

# MAKEFILE_DIR is directory Makefile located in
MAKEFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

### Makefile_template end ###

EFI_ARCH = x86_64-unknown-uefi
KERNEL_ARCH = x86_64-unknown-none

MNT = mnt
OVMF = ./bios/RELEASEX64_OVMF.fd

LOADER_NAME = htloader
KERNEL_NAME = htkernel
LIB_NAME = htlib

TARGET_EFI = boot/target/$(EFI_ARCH)/release/$(LOADER_NAME).efi
TARGET_KERNEL = kernel/target/$(KERNEL_ARCH)/release/$(KERNEL_NAME).elf

.PHONY: default all clean run install boot kernel lib

default: lib kernel boot

clean:
> rm -rf kernel/target boot/target lib/target $(MNT)

all: default install run

run:
> qemu-system-x86_64 -bios $(OVMF) -drive format=raw,file=fat:rw:$(MNT)

install:
> ./dl_ovmf.sh
> ./install.sh $(TARGET_EFI) $(TARGET_KERNEL)

boot:
> cd boot; cargo build --release

kernel:
> cd kernel; cargo build --release --target $(KERNEL_ARCH).json

lib:
> cd lib; cargo build --release
