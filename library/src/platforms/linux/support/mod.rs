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

// TODO: Derive?
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

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub type La_x86_64_xmm = [f32; 4usize];
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub type La_x86_64_ymm = [f32; 8usize];
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub type La_x86_64_zmm = [f64; 8usize];
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union La_x86_64_vector {
    pub ymm: [La_x86_64_ymm; 2usize],
    pub zmm: [La_x86_64_zmm; 1usize],
    pub xmm: [La_x86_64_xmm; 4usize]
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct La_x86_64_regs {
    pub lr_rdx: u64,
    pub lr_r8: u64,
    pub lr_r9: u64,
    pub lr_rcx: u64,
    pub lr_rsi: u64,
    pub lr_rdi: u64,
    pub lr_rbp: u64,
    pub lr_rsp: u64,
    pub lr_xmm: [La_x86_64_xmm; 8usize],
    pub lr_vector: [La_x86_64_vector; 8usize],
    pub lr_bnd: [i128; 4usize]
}

#[cfg(target_arch = "x86")]
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct La_x32_regs
{
    pub lr_rdx: u64,
    pub lr_r8: u64,
    pub lr_r9: u64,
    pub lr_rcx: u64,
    pub lr_rsi: u64,
    pub lr_rdi: u64,
    pub lr_rbp: u64,
    pub lr_rsp: u64,
    pub lr_xmm: [La_x86_64_xmm; 8usize],
    pub lr_vector: [La_x86_64_vector; 8usize]
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct La_aarch64_regs {
    pub lr_xreg: [u64; 8usize],
    pub lr_dreg: [u64; 8usize],
    pub lr_sp: u64,
    pub lr_lr: u64
}