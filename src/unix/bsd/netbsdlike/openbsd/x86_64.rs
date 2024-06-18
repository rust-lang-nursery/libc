use PT_FIRSTMACH;

pub type c_long = i64;
pub type c_ulong = u64;
pub type c_char = i8;
pub type ucontext_t = sigcontext;

s! {
    pub struct sigcontext {
        pub sc_rdi: ::c_long,
        pub sc_rsi: ::c_long,
        pub sc_rdx: ::c_long,
        pub sc_rcx: ::c_long,
        pub sc_r8: ::c_long,
        pub sc_r9: ::c_long,
        pub sc_r10: ::c_long,
        pub sc_r11: ::c_long,
        pub sc_r12: ::c_long,
        pub sc_r13: ::c_long,
        pub sc_r14: ::c_long,
        pub sc_r15: ::c_long,
        pub sc_rbp: ::c_long,
        pub sc_rbx: ::c_long,
        pub sc_rax: ::c_long,
        pub sc_gs: ::c_long,
        pub sc_fs: ::c_long,
        pub sc_es: ::c_long,
        pub sc_ds: ::c_long,
        pub sc_trapno: ::c_long,
        pub sc_err: ::c_long,
        pub sc_rip: ::c_long,
        pub sc_cs: ::c_long,
        pub sc_rflags: ::c_long,
        pub sc_rsp: ::c_long,
        pub sc_ss: ::c_long,
        pub sc_fpstate: *mut fxsave64,
        __sc_unused: ::c_int,
        pub sc_mask: ::c_int,
        pub sc_cookie: ::c_long,
    }

    #[repr(packed)]
    pub struct fxsave64 {
        pub fx_fcw: u16,
        pub fx_fsw: u16,
        pub fx_ftw: u8,
        __fx_unused1: u8,
        pub fx_fop: u16,
        pub fx_rip: u64,
        pub fx_rdp: u64,
        pub fx_mxcsr: u32,
        pub fx_mxcsr_mask: u32,
        pub fx_st: [[u64; 2]; 8],
        pub fx_xmm: [[u64; 2]; 16],
        __fx_unused3: [u8; 96],
    }
}

pub(crate) const _ALIGNBYTES: usize = ::mem::size_of::<::c_long>() - 1;

pub const _MAX_PAGE_SHIFT: u32 = 12;

pub const PT_STEP: ::c_int = PT_FIRSTMACH + 0;
pub const PT_GETREGS: ::c_int = PT_FIRSTMACH + 1;
pub const PT_SETREGS: ::c_int = PT_FIRSTMACH + 2;
pub const PT_GETFPREGS: ::c_int = PT_FIRSTMACH + 3;
pub const PT_SETFPREGS: ::c_int = PT_FIRSTMACH + 4;
