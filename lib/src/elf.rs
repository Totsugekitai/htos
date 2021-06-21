#![allow(non_camel_case_types)]
pub type Elf64_Addr = u64;
pub type Elf64_Off = u64;
pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Sword = u64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = u64;

#[repr(C)]
#[derive(Debug, Default)]
pub struct Elf64_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}

impl Elf64_Ehdr {
    pub fn is_valid(&self) -> bool {
        const MAGIC: [u8; 4] = *b"\x7fELF";
        self.e_ident[..3] == MAGIC
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}

impl Elf64_Phdr {
    
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct Elf64_Shdr {
    pub sh_name: Elf64_Word,
    pub sh_type: Elf64_Word,
    pub sh_flags: Elf64_Xword,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: Elf64_Off,
    pub sh_size: Elf64_Xword,
    pub sh_link: Elf64_Word,
    pub sh_info: Elf64_Word,
    pub sh_addralign: Elf64_Xword,
    pub sh_entsize: Elf64_Xword,
}
