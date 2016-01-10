pub type c_long = i64;
pub type c_ulong = u64;
pub type time_t = i64;
pub type suseconds_t = i64;

pub type clock_t = u64;
pub type ino_t = u64;
pub type nlink_t = u32;
pub type blksize_t = u64;

pub const PTHREAD_STACK_MIN: ::size_t = 1024;
pub const KERN_PROC_PATHNAME: ::c_int = 9;
pub const SIGSTKSZ: ::size_t = 40960;
pub const MADV_INVAL: ::c_int = 10;

s!{
    pub struct stat {
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_dev: ::dev_t,
        pub st_mode: ::mode_t,
        pub st_padding1: ::uint16_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::int64_t,
        pub st_blksize: ::uint32_t,
        pub st_flags: ::uint32_t,
        pub st_gen: ::uint32_t,
        pub st_lspare: ::int32_t,
        pub st_qspare1: ::int64_t,
        pub st_qspare2: ::int64_t,
    }

    pub struct dirent {
        pub d_fileno: ::ino_t,
        pub d_namlen: u16,
        pub d_type: u8,
        __unused1: u8,
        __unused2: u32,
        pub d_name: [::c_char; 256],
    }

    pub struct utsname {
        pub sysname: [::c_char; 32],
        pub nodename: [::c_char; 32],
        pub release: [::c_char; 32],
        pub version: [::c_char; 32],
        pub machine: [::c_char; 32],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_char,
        pub ss_size: ::size_t,
        pub ss_flags: ::c_int,
    }
}

extern {
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn clock_gettime(clk_id: ::uint64_t, tp: *mut ::timespec) -> ::c_int;

    pub fn __dfly_error() -> *const ::c_int;
}
