pub const LA_FLG_BINDTO: libc::c_uint = 0x01;
pub const LA_FLG_BINDFROM: libc::c_uint = 0x02;

pub const DN_CREATE: u32 = 0x00000004;
pub const DN_MULTISHOT: u32 = 0x80000000;

pub const F_SETSIG: libc::c_int = 10;

#[cfg(target_pointer_width = "32")]
pub type ElfW_Sym = libc::Elf32_Sym;
#[cfg(target_pointer_width = "32")]
pub type ElfW_Addr = libc::Elf32_Addr;
#[cfg(target_pointer_width = "64")]
pub type ElfW_Sym = libc::Elf64_Sym;
#[cfg(target_pointer_width = "64")]
pub type ElfW_Addr = libc::Elf64_Addr;

#[repr(C)]
pub struct LinkMap {
    pub l_addr: usize,
    pub l_name: *const libc::c_char,
    pub l_ld: usize,
    pub l_next: *mut LinkMap,
    pub l_prev: *mut LinkMap
}

#[cfg(any(target_arch = "i386", target_arch = "i586", target_arch = "i686"))]
#[repr(C)]
pub struct La_i86_regs
{
    pub lr_edx: u32,
    pub lr_ecx: u32,
    pub lr_eax: u32,
    pub lr_ebp: u32,
    pub lr_esp: u32
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
pub struct La_aarch64_regs
{
    pub lr_xreg: [u64; 8],
    pub lr_dreg: [u64; 8],
    pub lr_sp: u64,
    pub lr_lr: u64
}