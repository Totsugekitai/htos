use super::error::Error;

pub type Elf64Addr = u64;
pub type Elf64Off = u64;
pub type Elf64Half = u16;
pub type Elf64Word = u32;
pub type Elf64Sword = u64;
pub type Elf64Xword = u64;
pub type Elf64Sxword = u64;

#[repr(C)]
#[derive(Debug, Default)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

#[derive(Debug)]
pub enum ElfType {
    None = 0,
    Rel = 1,
    Exec = 2,
    Dyn = 3,
    Core = 4,
    Unknown,
}

impl Elf64Ehdr {
    pub fn is_valid(&self) -> bool {
        const MAGIC: [u8; 4] = *b"\x7fELF";
        self.e_ident[..4] == MAGIC
    }

    pub fn elf_type(&self) -> ElfType {
        use self::ElfType::*;
        match self.e_type {
            0 => None,
            1 => Rel,
            2 => Exec,
            3 => Dyn,
            4 => Core,
            _ => Unknown,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct Elf64Phdr {
    pub p_type: Elf64Word,
    pub p_flags: Elf64Word,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: Elf64Xword,
    pub p_memsz: Elf64Xword,
    pub p_align: Elf64Xword,
}

pub enum PhdrType {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    Shlib = 5,
    Phdr = 6,
    Tls = 7,
    Loos = 0x60000000,
    Hios = 0x6fffffff,
    Loproc = 0x70000000,
    Hiproc = 0x7fffffff,
    GnuEhFrame = 0x6474e550,
    GnuStack = 0x6474e551,
    GnuRelRo = 0x6474e552,
    Unknown,
}

impl Elf64Phdr {
    pub fn get_type(&self) -> PhdrType {
        use self::PhdrType::*;
        match self.p_type {
            0 => Null,
            1 => Load,
            2 => Dynamic,
            3 => Interp,
            4 => Note,
            5 => Shlib,
            6 => Phdr,
            7 => Tls,
            0x60000000 => Loos,
            0x6fffffff => Hios,
            0x70000000 => Loproc,
            0x7fffffff => Hiproc,
            0x6474e550 => GnuEhFrame,
            0x6474e551 => GnuStack,
            0x6474e552 => GnuRelRo,
            _ => Unknown,
        }
    }

    pub fn is_valid_flag(&self) -> bool {
        let out_of_bound_bits = 0xfffffff8;
        if out_of_bound_bits & self.p_flags != 0 {
            false
        } else {
            true
        }
    }

    pub fn is_valid(&self) -> bool {
        if let PhdrType::Unknown = self.get_type() {
            return false;
        }
        if !self.is_valid_flag() {
            return false;
        }
        true
    }

    pub fn load_segmemt(&self, head: &[u8]) -> Result<(), Error> {
        use super::error::ErrorKind::*;
        if !self.is_valid() {
            return Err(Error {
                kind: InvalidParameter,
            });
        }
        match self.get_type() {
            PhdrType::Load => {
                for i in 0..self.p_memsz {
                    let dst = self.p_vaddr + i;
                    unsafe {
                        *(dst as *mut u8) = if i < self.p_filesz {
                            head[self.p_offset as usize + i as usize]
                        } else {
                            0
                        };
                    }
                }
                Ok(())
            }
            _ => Err(Error { kind: NotFound }),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct Elf64Shdr {
    pub sh_name: Elf64Word,
    pub sh_type: Elf64Word,
    pub sh_flags: Elf64Xword,
    pub sh_addr: Elf64Addr,
    pub sh_offset: Elf64Off,
    pub sh_size: Elf64Xword,
    pub sh_link: Elf64Word,
    pub sh_info: Elf64Word,
    pub sh_addralign: Elf64Xword,
    pub sh_entsize: Elf64Xword,
}
