pub type c_char = i8;
pub type wchar_t = i32;

s! {
    pub struct stat64 {
        pub st_dev: ::c_ulong,
        st_pad1: [::c_long; 3],
        pub st_ino: ::ino64_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::c_ulong,
        st_pad2: [::c_long; 2],
        pub st_size: ::off64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        st_pad3: ::c_long,
        pub st_blocks: ::blkcnt64_t,
        st_pad5: [::c_long; 14],
    }

    pub struct statfs {
        pub f_type: ::c_long,
        pub f_bsize: ::c_long,
        pub f_frsize: ::c_long,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_files: ::fsblkcnt_t,
        pub f_ffree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_fsid: ::fsid_t,

        pub f_namelen: ::c_long,
        f_spare: [::c_long; 6],
    }

    pub struct statfs64 {
        pub f_type: ::c_long,
        pub f_bsize: ::c_long,
        pub f_frsize: ::c_long,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_bavail: u64,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_long,
        pub f_flags: ::c_long,
        pub f_spare: [::c_long; 5],
    }

    pub struct statvfs64 {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_favail: u64,
        pub f_fsid: ::c_ulong,
        __f_unused: ::c_int,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct sigaction {
        pub sa_flags: ::c_int,
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        pub sa_restorer: ::Option<extern fn()>,
        _resv: [::c_int; 1],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_size: ::size_t,
        pub ss_flags: ::c_int,
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_code: ::c_int,
        pub si_errno: ::c_int,
        pub _pad: [::c_int; 29],
    }

    pub struct ipc_perm {
        pub __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::c_uint,
        pub __seq: ::c_ushort,
        __pad1: ::c_ushort,
        __unused1: ::c_ulong,
        __unused2: ::c_ulong
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_atime: ::time_t,
        pub shm_dtime: ::time_t,
        pub shm_ctime: ::time_t,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        __unused4: ::c_ulong,
        __unused5: ::c_ulong
    }

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        #[cfg(target_endian = "big")]
        __glibc_reserved1: ::c_ulong,
        pub msg_stime: ::time_t,
        #[cfg(target_endian = "little")]
        __glibc_reserved1: ::c_ulong,
        #[cfg(target_endian = "big")]
        __glibc_reserved2: ::c_ulong,
        pub msg_rtime: ::time_t,
        #[cfg(target_endian = "little")]
        __glibc_reserved2: ::c_ulong,
        #[cfg(target_endian = "big")]
        __glibc_reserved3: ::c_ulong,
        pub msg_ctime: ::time_t,
        #[cfg(target_endian = "little")]
        __glibc_reserved3: ::c_ulong,
        __msg_cbytes: ::c_ulong,
        pub msg_qnum: ::msgqnum_t,
        pub msg_qbytes: ::msglen_t,
        pub msg_lspid: ::pid_t,
        pub msg_lrpid: ::pid_t,
        __glibc_reserved4: ::c_ulong,
        __glibc_reserved5: ::c_ulong,
    }

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_sysid: ::c_long,
        pub l_pid: ::pid_t,
        pad: [::c_long; 4],
    }

    pub struct termios2 {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; 23],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }
}

pub const O_LARGEFILE: ::c_int = 0x2000;

pub const O_DIRECT: ::c_int = 0x8000;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_NOFOLLOW: ::c_int = 0x20000;

pub const RLIM_INFINITY: ::rlim_t = 0x7fffffff;
pub const RLIMIT_NOFILE: ::__rlimit_resource_t = 5;
pub const RLIMIT_AS: ::__rlimit_resource_t = 6;
pub const RLIMIT_RSS: ::__rlimit_resource_t = 7;
pub const RLIMIT_NPROC: ::__rlimit_resource_t = 8;
pub const RLIMIT_MEMLOCK: ::__rlimit_resource_t = 9;

pub const O_APPEND: ::c_int = 8;
pub const O_CREAT: ::c_int = 256;
pub const O_EXCL: ::c_int = 1024;
pub const O_NOCTTY: ::c_int = 2048;
pub const O_NONBLOCK: ::c_int = 128;
pub const O_SYNC: ::c_int = 0x4010;
pub const O_RSYNC: ::c_int = 0x4010;
pub const O_DSYNC: ::c_int = 0x10;
pub const O_FSYNC: ::c_int = 0x4010;
pub const O_ASYNC: ::c_int = 0x1000;
pub const O_NDELAY: ::c_int = 0x80;

pub const EDEADLK: ::c_int = 45;
pub const ENAMETOOLONG: ::c_int = 78;
pub const ENOLCK: ::c_int = 46;
pub const ENOSYS: ::c_int = 89;
pub const ENOTEMPTY: ::c_int = 93;
pub const ELOOP: ::c_int = 90;
pub const ENOMSG: ::c_int = 35;
pub const EIDRM: ::c_int = 36;
pub const ECHRNG: ::c_int = 37;
pub const EL2NSYNC: ::c_int = 38;
pub const EL3HLT: ::c_int = 39;
pub const EL3RST: ::c_int = 40;
pub const ELNRNG: ::c_int = 41;
pub const EUNATCH: ::c_int = 42;
pub const ENOCSI: ::c_int = 43;
pub const EL2HLT: ::c_int = 44;
pub const EBADE: ::c_int = 50;
pub const EBADR: ::c_int = 51;
pub const EXFULL: ::c_int = 52;
pub const ENOANO: ::c_int = 53;
pub const EBADRQC: ::c_int = 54;
pub const EBADSLT: ::c_int = 55;
pub const EDEADLOCK: ::c_int = 56;
pub const EMULTIHOP: ::c_int = 74;
pub const EOVERFLOW: ::c_int = 79;
pub const ENOTUNIQ: ::c_int = 80;
pub const EBADFD: ::c_int = 81;
pub const EBADMSG: ::c_int = 77;
pub const EREMCHG: ::c_int = 82;
pub const ELIBACC: ::c_int = 83;
pub const ELIBBAD: ::c_int = 84;
pub const ELIBSCN: ::c_int = 85;
pub const ELIBMAX: ::c_int = 86;
pub const ELIBEXEC: ::c_int = 87;
pub const EILSEQ: ::c_int = 88;
pub const ERESTART: ::c_int = 91;
pub const ESTRPIPE: ::c_int = 92;
pub const EUSERS: ::c_int = 94;
pub const ENOTSOCK: ::c_int = 95;
pub const EDESTADDRREQ: ::c_int = 96;
pub const EMSGSIZE: ::c_int = 97;
pub const EPROTOTYPE: ::c_int = 98;
pub const ENOPROTOOPT: ::c_int = 99;
pub const EPROTONOSUPPORT: ::c_int = 120;
pub const ESOCKTNOSUPPORT: ::c_int = 121;
pub const EOPNOTSUPP: ::c_int = 122;
pub const EPFNOSUPPORT: ::c_int = 123;
pub const EAFNOSUPPORT: ::c_int = 124;
pub const EADDRINUSE: ::c_int = 125;
pub const EADDRNOTAVAIL: ::c_int = 126;
pub const ENETDOWN: ::c_int = 127;
pub const ENETUNREACH: ::c_int = 128;
pub const ENETRESET: ::c_int = 129;
pub const ECONNABORTED: ::c_int = 130;
pub const ECONNRESET: ::c_int = 131;
pub const ENOBUFS: ::c_int = 132;
pub const EISCONN: ::c_int = 133;
pub const ENOTCONN: ::c_int = 134;
pub const ESHUTDOWN: ::c_int = 143;
pub const ETOOMANYREFS: ::c_int = 144;
pub const ETIMEDOUT: ::c_int = 145;
pub const ECONNREFUSED: ::c_int = 146;
pub const EHOSTDOWN: ::c_int = 147;
pub const EHOSTUNREACH: ::c_int = 148;
pub const EALREADY: ::c_int = 149;
pub const EINPROGRESS: ::c_int = 150;
pub const ESTALE: ::c_int = 151;
pub const EUCLEAN: ::c_int = 135;
pub const ENOTNAM: ::c_int = 137;
pub const ENAVAIL: ::c_int = 138;
pub const EISNAM: ::c_int = 139;
pub const EREMOTEIO: ::c_int = 140;
pub const EDQUOT: ::c_int = 1133;
pub const ENOMEDIUM: ::c_int = 159;
pub const EMEDIUMTYPE: ::c_int = 160;
pub const ECANCELED: ::c_int = 158;
pub const ENOKEY: ::c_int = 161;
pub const EKEYEXPIRED: ::c_int = 162;
pub const EKEYREVOKED: ::c_int = 163;
pub const EKEYREJECTED: ::c_int = 164;
pub const EOWNERDEAD: ::c_int = 165;
pub const ENOTRECOVERABLE: ::c_int = 166;
pub const ERFKILL: ::c_int = 167;

pub const MAP_NORESERVE: ::c_int = 0x400;
pub const MAP_ANON: ::c_int = 0x800;
pub const MAP_ANONYMOUS: ::c_int = 0x800;
pub const MAP_GROWSDOWN: ::c_int = 0x1000;
pub const MAP_DENYWRITE: ::c_int = 0x2000;
pub const MAP_EXECUTABLE: ::c_int = 0x4000;
pub const MAP_LOCKED: ::c_int = 0x8000;
pub const MAP_POPULATE: ::c_int = 0x10000;
pub const MAP_NONBLOCK: ::c_int = 0x20000;
pub const MAP_STACK: ::c_int = 0x40000;

pub const SOCK_STREAM: ::c_int = 2;
pub const SOCK_DGRAM: ::c_int = 1;

pub const SOL_SOCKET: ::c_int = 0xffff;

pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_REUSEPORT: ::c_int = 0x0200;
pub const SO_TYPE: ::c_int = 0x1008;
pub const SO_STYLE: ::c_int = SO_TYPE;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDLOWAT: ::c_int = 0x1003;
pub const SO_RCVLOWAT: ::c_int = 0x1004;
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
pub const SO_ACCEPTCONN: ::c_int = 0x1009;
pub const SO_PROTOCOL: ::c_int = 0x1028;
pub const SO_DOMAIN: ::c_int = 0x1029;
pub const SO_NO_CHECK: ::c_int = 11;
pub const SO_PASSCRED: ::c_int = 17;
pub const SO_PEERCRED: ::c_int = 18;
pub const SO_SECURITY_AUTHENTICATION: ::c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: ::c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: ::c_int = 24;
pub const SO_ATTACH_FILTER: ::c_int = 26;
pub const SO_DETACH_FILTER: ::c_int = 27;
pub const SO_GET_FILTER: ::c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME: ::c_int = 28;
pub const SO_PEERSEC: ::c_int = 30;
pub const SO_SNDBUFFORCE: ::c_int = 31;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_PASSSEC: ::c_int = 34;
pub const SO_TIMESTAMPNS: ::c_int = 35;
pub const SCM_TIMESTAMPNS: ::c_int = SO_TIMESTAMPNS;

pub const SO_WIFI_STATUS: ::c_int = 41;
pub const SCM_WIFI_STATUS: ::c_int = SO_WIFI_STATUS;
pub const SO_NOFCS: ::c_int = 43;
pub const SO_LOCK_FILTER: ::c_int = 44;
pub const SO_SELECT_ERR_QUEUE: ::c_int = 45;
pub const SO_MAX_PACING_RATE: ::c_int = 47;
pub const SO_BPF_EXTENSIONS: ::c_int = 48;
pub const SO_INCOMING_CPU: ::c_int = 49;
pub const SO_ATTACH_BPF: ::c_int = 50;
pub const SO_DETACH_BPF: ::c_int = SO_DETACH_FILTER;

pub const FIOCLEX: ::c_ulong = 0x6601;
pub const FIONCLEX: ::c_ulong = 0x6602;
pub const FIONBIO: ::c_ulong = 0x667e;

pub const SA_SIGINFO: ::c_int = 0x00000008;
pub const SA_NOCLDWAIT: ::c_int = 0x00010000;

pub const SIGCHLD: ::c_int = 18;
pub const SIGBUS: ::c_int = 10;
pub const SIGTTIN: ::c_int = 26;
pub const SIGTTOU: ::c_int = 27;
pub const SIGXCPU: ::c_int = 30;
pub const SIGXFSZ: ::c_int = 31;
pub const SIGVTALRM: ::c_int = 28;
pub const SIGPROF: ::c_int = 29;
pub const SIGWINCH: ::c_int = 20;
pub const SIGUSR1: ::c_int = 16;
pub const SIGUSR2: ::c_int = 17;
pub const SIGCONT: ::c_int = 25;
pub const SIGSTOP: ::c_int = 23;
pub const SIGTSTP: ::c_int = 24;
pub const SIGURG: ::c_int = 21;
pub const SIGIO: ::c_int = 22;
pub const SIGSYS: ::c_int = 12;
pub const SIGPOLL: ::c_int = 22;
pub const SIGPWR: ::c_int = 19;
pub const SIG_SETMASK: ::c_int = 3;
pub const SIG_BLOCK: ::c_int = 0x1;
pub const SIG_UNBLOCK: ::c_int = 0x2;

pub const POLLWRNORM: ::c_short = 0x004;
pub const POLLWRBAND: ::c_short = 0x100;

pub const VEOF: usize = 16;
pub const VEOL: usize = 17;
pub const VEOL2: usize = 6;
pub const VMIN: usize = 4;
pub const IEXTEN: ::tcflag_t = 0x00000100;
pub const TOSTOP: ::tcflag_t = 0x00008000;
pub const FLUSHO: ::tcflag_t = 0x00002000;
pub const EXTPROC: ::tcflag_t = 0o200000;
pub const TCSANOW: ::c_int = 0x540e;
pub const TCSADRAIN: ::c_int = 0x540f;
pub const TCSAFLUSH: ::c_int = 0x5410;

pub const PTRACE_GETFPXREGS: ::c_uint = 18;
pub const PTRACE_SETFPXREGS: ::c_uint = 19;

pub const MAP_HUGETLB: ::c_int = 0x080000;

pub const EFD_NONBLOCK: ::c_int = 0x80;

pub const F_GETLK: ::c_int = 14;
pub const F_GETOWN: ::c_int = 23;
pub const F_SETOWN: ::c_int = 24;

pub const SFD_NONBLOCK: ::c_int = 0x80;

pub const TCGETS: ::c_ulong = 0x540d;
pub const TCSETS: ::c_ulong = 0x540e;
pub const TCSETSW: ::c_ulong = 0x540f;
pub const TCSETSF: ::c_ulong = 0x5410;
pub const TCGETA: ::c_ulong = 0x5401;
pub const TCSETA: ::c_ulong = 0x5402;
pub const TCSETAW: ::c_ulong = 0x5403;
pub const TCSETAF: ::c_ulong = 0x5404;
pub const TCSBRK: ::c_ulong = 0x5405;
pub const TCXONC: ::c_ulong = 0x5406;
pub const TCFLSH: ::c_ulong = 0x5407;
pub const TIOCGSOFTCAR: ::c_ulong = 0x5481;
pub const TIOCSSOFTCAR: ::c_ulong = 0x5482;
pub const TIOCINQ: ::c_ulong = 0x467f;
pub const TIOCLINUX: ::c_ulong = 0x5483;
pub const TIOCGSERIAL: ::c_ulong = 0x5484;
pub const TIOCEXCL: ::c_ulong = 0x740d;
pub const TIOCNXCL: ::c_ulong = 0x740e;
pub const TIOCSCTTY: ::c_ulong = 0x5480;
pub const TIOCGPGRP: ::c_ulong = 0x40047477;
pub const TIOCSPGRP: ::c_ulong = 0x80047476;
pub const TIOCOUTQ: ::c_ulong = 0x7472;
pub const TIOCSTI: ::c_ulong = 0x5472;
pub const TIOCGWINSZ: ::c_ulong = 0x40087468;
pub const TIOCSWINSZ: ::c_ulong = 0x80087467;
pub const TIOCMGET: ::c_ulong = 0x741d;
pub const TIOCMBIS: ::c_ulong = 0x741b;
pub const TIOCMBIC: ::c_ulong = 0x741c;
pub const TIOCMSET: ::c_ulong = 0x741a;
pub const FIONREAD: ::c_ulong = 0x467f;
pub const TIOCCONS: ::c_ulong = 0x80047478;

pub const RTLD_DEEPBIND: ::c_int = 0x10;
pub const RTLD_GLOBAL: ::c_int = 0x4;
pub const RTLD_NOLOAD: ::c_int = 0x8;

pub const MCL_CURRENT: ::c_int = 0x0001;
pub const MCL_FUTURE: ::c_int = 0x0002;

pub const SIGSTKSZ: ::size_t = 8192;
pub const MINSIGSTKSZ: ::size_t = 2048;
pub const CBAUD: ::tcflag_t = 0o0010017;
pub const TAB1: ::tcflag_t = 0x00000800;
pub const TAB2: ::tcflag_t = 0x00001000;
pub const TAB3: ::tcflag_t = 0x00001800;
pub const CR1: ::tcflag_t = 0x00000200;
pub const CR2: ::tcflag_t = 0x00000400;
pub const CR3: ::tcflag_t = 0x00000600;
pub const FF1: ::tcflag_t = 0x00008000;
pub const BS1: ::tcflag_t = 0x00002000;
pub const VT1: ::tcflag_t = 0x00004000;
pub const VWERASE: usize = 14;
pub const VREPRINT: usize = 12;
pub const VSUSP: usize = 10;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VDISCARD: usize = 13;
pub const VTIME: usize = 5;
pub const IXON: ::tcflag_t = 0x00000400;
pub const IXOFF: ::tcflag_t = 0x00001000;
pub const ONLCR: ::tcflag_t = 0x4;
pub const CSIZE: ::tcflag_t = 0x00000030;
pub const CS6: ::tcflag_t = 0x00000010;
pub const CS7: ::tcflag_t = 0x00000020;
pub const CS8: ::tcflag_t = 0x00000030;
pub const CSTOPB: ::tcflag_t = 0x00000040;
pub const CREAD: ::tcflag_t = 0x00000080;
pub const PARENB: ::tcflag_t = 0x00000100;
pub const PARODD: ::tcflag_t = 0x00000200;
pub const HUPCL: ::tcflag_t = 0x00000400;
pub const CLOCAL: ::tcflag_t = 0x00000800;
pub const ECHOKE: ::tcflag_t = 0x00000800;
pub const ECHOE: ::tcflag_t = 0x00000010;
pub const ECHOK: ::tcflag_t = 0x00000020;
pub const ECHONL: ::tcflag_t = 0x00000040;
pub const ECHOPRT: ::tcflag_t = 0x00000400;
pub const ECHOCTL: ::tcflag_t = 0x00000200;
pub const ISIG: ::tcflag_t = 0x00000001;
pub const ICANON: ::tcflag_t = 0x00000002;
pub const PENDIN: ::tcflag_t = 0x00004000;
pub const NOFLSH: ::tcflag_t = 0x00000080;
pub const CIBAUD: ::tcflag_t = 0o02003600000;
pub const CBAUDEX: ::tcflag_t = 0o010000;
pub const VSWTC: usize = 7;
pub const OLCUC: ::tcflag_t = 0o000002;
pub const NLDLY: ::tcflag_t = 0o000400;
pub const CRDLY: ::tcflag_t = 0o003000;
pub const TABDLY: ::tcflag_t = 0o014000;
pub const BSDLY: ::tcflag_t = 0o020000;
pub const FFDLY: ::tcflag_t = 0o100000;
pub const VTDLY: ::tcflag_t = 0o040000;
pub const XTABS: ::tcflag_t = 0o014000;

pub const B0: ::speed_t = 0o000000;
pub const B50: ::speed_t = 0o000001;
pub const B75: ::speed_t = 0o000002;
pub const B110: ::speed_t = 0o000003;
pub const B134: ::speed_t = 0o000004;
pub const B150: ::speed_t = 0o000005;
pub const B200: ::speed_t = 0o000006;
pub const B300: ::speed_t = 0o000007;
pub const B600: ::speed_t = 0o000010;
pub const B1200: ::speed_t = 0o000011;
pub const B1800: ::speed_t = 0o000012;
pub const B2400: ::speed_t = 0o000013;
pub const B4800: ::speed_t = 0o000014;
pub const B9600: ::speed_t = 0o000015;
pub const B19200: ::speed_t = 0o000016;
pub const B38400: ::speed_t = 0o000017;
pub const EXTA: ::speed_t = B19200;
pub const EXTB: ::speed_t = B38400;
pub const BOTHER: ::speed_t = 0o010000;
pub const B57600: ::speed_t = 0o010001;
pub const B115200: ::speed_t = 0o010002;
pub const B230400: ::speed_t = 0o010003;
pub const B460800: ::speed_t = 0o010004;
pub const B500000: ::speed_t = 0o010005;
pub const B576000: ::speed_t = 0o010006;
pub const B921600: ::speed_t = 0o010007;
pub const B1000000: ::speed_t = 0o010010;
pub const B1152000: ::speed_t = 0o010011;
pub const B1500000: ::speed_t = 0o010012;
pub const B2000000: ::speed_t = 0o010013;
pub const B2500000: ::speed_t = 0o010014;
pub const B3000000: ::speed_t = 0o010015;
pub const B3500000: ::speed_t = 0o010016;
pub const B4000000: ::speed_t = 0o010017;

pub const TIOCM_ST: ::c_int = 0x010;
pub const TIOCM_SR: ::c_int = 0x020;
pub const TIOCM_CTS: ::c_int = 0x040;
pub const TIOCM_CAR: ::c_int = 0x100;
pub const TIOCM_RNG: ::c_int = 0x200;
pub const TIOCM_DSR: ::c_int = 0x400;

pub const EHWPOISON: ::c_int = 168;

cfg_if! {
    if #[cfg(libc_align)] {
        mod align;
        pub use self::align::*;
    }
}
