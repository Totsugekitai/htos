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
MAKEFLAGS += --no-builtin-rules --no-builtin-variables

# MAKEFILE_DIR is directory Makefile located in
MAKEFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
### Makefile_template end ###

EFI_ARCH = x86_64-unknown-uefi
KERNEL_ARCH = x64

MNT = mnt
OVMF = ./bios/RELEASEX64_OVMF.fd

LOADER_NAME = htloader
KERNEL_NAME = htkernel
LIB_NAME = htlib

TARGET_EFI = target/$(EFI_ARCH)/release/$(LOADER_NAME).efi
TARGET_KERNEL = target/$(KERNEL_ARCH)/release/$(KERNEL_NAME).elf
TARGET_KERNEL_DEBUG = target/$(KERNEL_ARCH)/debug/$(KERNEL_NAME).elf

QEMU_ARGS = \
  -bios $(OVMF) \
  -drive format=raw,file=fat:rw:$(MNT) \
  -device nec-usb-xhci,id=xhci -device usb-kbd

QEMU_DEBUG_ARGS = -gdb tcp::1234 -monitor telnet::5556,server,nowait

export RELEASE ?=
export ARCH ?= x64

kernel_target_json := kernel/arch/$(ARCH)/$(ARCH).json
build_mode := $(if $(RELEASE),release,debug)

export RUSTFLAGS = -Z emit-stack-sizes
CARGO ?= cargo +nightly
CARGOFLAGS += -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
CARGOFLAGS += $(if $(RELEASE),--release,)

.PHONY: build
build: build-kernel build-boot

.PHONY: build-kernel
build-kernel:
> $(CARGO) build $(CARGOFLAGS) --target $(kernel_target_json) --manifest-path kernel/Cargo.toml

.PHONY: build-boot
build-boot:
> $(CARGO) build $(CARGOFLAGS) --target $(EFI_ARCH) --manifest-path boot/Cargo.toml

clean:
> rm -rf target $(MNT) iso *.iso *.img *.map
#
#all: default install run
#
#debug-all: debug-boot debug-kernel debug-install debug-run
#
#debug-all-stop: debug-boot debug-kernel debug-install debug-stop
#
run:
> qemu-system-x86_64 $(QEMU_ARGS)
#
#debug-run:
#> qemu-system-x86_64 $(QEMU_ARGS) $(QEMU_DEBUG_ARGS)
#
#debug-stop:
#> qemu-system-x86_64 $(QEMU_ARGS) -S $(QEMU_DEBUG_ARGS)
#
install:
> ./dl_ovmf.sh
> ./install.sh $(TARGET_EFI) $(TARGET_KERNEL)
#
#debug-install:
#> ./dl_ovmf.sh
#> ./install.sh $(TARGET_EFI) $(TARGET_KERNEL_DEBUG)
#
#boot:
#> cd boot; cargo build --release
#
#debug-boot:
#> cd boot; cargo build
#
#kernel:
#> cd kernel; cargo build --release --target $(KERNEL_ARCH).json
#
#debug-kernel:
#> cd kernel; cargo build --target $(KERNEL_ARCH).json
