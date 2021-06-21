pub type Elf64Addr = u64;
pub type Elf64Off = u64;
pub type Elf64Half = u16;
pub type Elf64Word = u32;
pub type Elf64Sword = u64;
pub type Elf64Xword = u64;
pub type Elf64Sxword = u64;

#[repr(C)]
#[repr(packed)]
#[derive(Default)]
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
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
    UNKNOWN,
}

impl Elf64Ehdr {
    pub fn is_valid(&self) -> bool {
        const MAGIC: [u8; 4] = *b"\x7fELF";
        self.e_ident[..4] == MAGIC
    }

    pub fn elf_type(&self) -> ElfType {
        match self.e_type {
            0 => ElfType::NONE,
            1 => ElfType::REL,
            2 => ElfType::EXEC,
            3 => ElfType::DYN,
            4 => ElfType::CORE,
            _ => ElfType::UNKNOWN,
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

impl Elf64Phdr {
    
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
