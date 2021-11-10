#!/bin/bash

dd if=/dev/zero of=fat.img bs=1k count=1440
mformat -i fat.img -f 1440 ::
mmd -i fat.img ::/EFI
mmd -i fat.img ::/EFI/BOOT
mcopy -i fat.img ./mnt/EFI/BOOT/BOOTX64.EFI ::/EFI/BOOT
mcopy -i fat.img ./mnt/htkernel.elf ::/
mkdir -p iso
cp fat.img iso
xorriso -as mkisofs -R -f -e fat.img -no-emul-boot -o htos.iso iso
