s! {
    pub struct termios2 {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; 19],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }
}

// include/uapi/asm-generic/socket.h
// arch/alpha/include/uapi/asm/socket.h
// tools/include/uapi/asm-generic/socket.h
// arch/mips/include/uapi/asm/socket.h
pub const SOL_SOCKET: ::c_int = 1;

// Defined in unix/linux_like/mod.rs
// pub const SO_DEBUG: ::c_int = 1;
pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_TYPE: ::c_int = 3;
pub const SO_ERROR: ::c_int = 4;
pub const SO_DONTROUTE: ::c_int = 5;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_SNDBUF: ::c_int = 7;
pub const SO_RCVBUF: ::c_int = 8;
pub const SO_KEEPALIVE: ::c_int = 9;
pub const SO_OOBINLINE: ::c_int = 10;
pub const SO_NO_CHECK: ::c_int = 11;
pub const SO_PRIORITY: ::c_int = 12;
pub const SO_LINGER: ::c_int = 13;
pub const SO_BSDCOMPAT: ::c_int = 14;
pub const SO_REUSEPORT: ::c_int = 15;
pub const SO_PASSCRED: ::c_int = 16;
pub const SO_PEERCRED: ::c_int = 17;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_SNDTIMEO: ::c_int = 21;
// pub const SO_RCVTIMEO_OLD: ::c_int = 20;
// pub const SO_SNDTIMEO_OLD: ::c_int = 21;
pub const SO_SECURITY_AUTHENTICATION: ::c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: ::c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: ::c_int = 24;
pub const SO_BINDTODEVICE: ::c_int = 25;
pub const SO_ATTACH_FILTER: ::c_int = 26;
pub const SO_DETACH_FILTER: ::c_int = 27;
pub const SO_GET_FILTER: ::c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME: ::c_int = 28;
pub const SO_TIMESTAMP: ::c_int = 29;
// pub const SO_TIMESTAMP_OLD: ::c_int = 29;
pub const SO_ACCEPTCONN: ::c_int = 30;
pub const SO_PEERSEC: ::c_int = 31;
pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_PASSSEC: ::c_int = 34;
pub const SO_TIMESTAMPNS: ::c_int = 35;
// pub const SO_TIMESTAMPNS_OLD: ::c_int = 35;
pub const SO_MARK: ::c_int = 36;
pub const SO_TIMESTAMPING: ::c_int = 37;
// pub const SO_TIMESTAMPING_OLD: ::c_int = 37;
pub const SO_PROTOCOL: ::c_int = 38;
pub const SO_DOMAIN: ::c_int = 39;
pub const SO_RXQ_OVFL: ::c_int = 40;
pub const SO_WIFI_STATUS: ::c_int = 41;
pub const SCM_WIFI_STATUS: ::c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: ::c_int = 42;
pub const SO_NOFCS: ::c_int = 43;
pub const SO_LOCK_FILTER: ::c_int = 44;
pub const SO_SELECT_ERR_QUEUE: ::c_int = 45;
pub const SO_BUSY_POLL: ::c_int = 46;
pub const SO_MAX_PACING_RATE: ::c_int = 47;
pub const SO_BPF_EXTENSIONS: ::c_int = 48;
pub const SO_INCOMING_CPU: ::c_int = 49;
pub const SO_ATTACH_BPF: ::c_int = 50;
pub const SO_DETACH_BPF: ::c_int = SO_DETACH_FILTER;
pub const SO_ATTACH_REUSEPORT_CBPF: ::c_int = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: ::c_int = 52;
pub const SO_CNX_ADVICE: ::c_int = 53;
pub const SCM_TIMESTAMPING_OPT_STATS: ::c_int = 54;
pub const SO_MEMINFO: ::c_int = 55;
pub const SO_INCOMING_NAPI_ID: ::c_int = 56;
pub const SO_COOKIE: ::c_int = 57;
pub const SCM_TIMESTAMPING_PKTINFO: ::c_int = 58;
pub const SO_PEERGROUPS: ::c_int = 59;
pub const SO_ZEROCOPY: ::c_int = 60;
pub const SO_TXTIME: ::c_int = 61;
pub const SCM_TXTIME: ::c_int = SO_TXTIME;
pub const SO_BINDTOIFINDEX: ::c_int = 62;
cfg_if! {
    // Some of these platforms in CI already have these constants.
    // But they may still not have those _OLD ones.
    if #[cfg(all(
        any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "csky",
            target_arch = "loongarch64"
        ),
        not(any(target_env = "musl", target_env = "ohos"))
    ))] {
        pub const SO_TIMESTAMP_NEW: ::c_int = 63;
        pub const SO_TIMESTAMPNS_NEW: ::c_int = 64;
        pub const SO_TIMESTAMPING_NEW: ::c_int = 65;
        pub const SO_RCVTIMEO_NEW: ::c_int = 66;
        pub const SO_SNDTIMEO_NEW: ::c_int = 67;
        pub const SO_DETACH_REUSEPORT_BPF: ::c_int = 68;
    }
}
// pub const SO_PREFER_BUSY_POLL: ::c_int = 69;
// pub const SO_BUSY_POLL_BUDGET: ::c_int = 70;

cfg_if! {
    if #[cfg(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "s390x",
        target_arch = "csky",
        target_arch = "loongarch64"
    ))] {
        pub const FICLONE: ::c_ulong = 0x40049409;
        pub const FICLONERANGE: ::c_ulong = 0x4020940D;
        // getauxval bits flags - from asm/cpufeatures.h kernel header
        pub const X86_FEATURE_FPU: ::c_ulong = 0;
        pub const X86_FEATURE_TSC: ::c_ulong = 4;
        pub const X86_FEATURE_CX8: ::c_ulong = 8;
        pub const X86_FEATURE_CLFLUSH: ::c_ulong = 19;
        pub const X86_FEATURE_MMX: ::c_ulong = 23;
        pub const X86_FEATURE_AES: ::c_ulong = 128 + 25;
        pub const X86_FEATURE_ERMS: ::c_ulong = 288 + 9;
        pub const X86_FEATURE_F16C: ::c_ulong = 128 + 29;
        pub const X86_FEATURE_FMA4: ::c_ulong = 160 + 16;
        pub const X86_FEATURE_FMA: ::c_ulong = 128 + 12;
        pub const X86_FEATURE_VAES: ::c_ulong = 512 + 9;
        pub const X86_FEATURE_VPCLMULQDQ: ::c_ulong = 512 + 10;
        pub const X86_FEATURE_BMI1: ::c_ulong = 288 + 3;
        pub const X86_FEATURE_HLE: ::c_ulong = 288 + 4;
        pub const X86_FEATURE_BMI2: ::c_ulong = 288 + 8;
        pub const X86_FEATURE_RTM: ::c_ulong = 288 + 11;
        pub const X86_FEATURE_RDSEED: ::c_ulong = 288 + 18;
        pub const X86_FEATURE_CLFLUSHOPT: ::c_ulong = 288 + 23;
        pub const X86_FEATURE_CLWB: ::c_ulong = 288 + 24;
        pub const X86_FEATURE_XMM: ::c_ulong = 25;
        pub const X86_FEATURE_XMM2: ::c_ulong = 26;
        pub const X86_FEATURE_XMM3: ::c_ulong = 128 + 0;
        pub const X86_FEATURE_SSSE3: ::c_ulong = 128 + 9;
        pub const X86_FEATURE_XMM4_1: ::c_ulong = 128 + 19;
        pub const X86_FEATURE_XMM4_2: ::c_ulong = 128 + 20;
        pub const X86_FEATURE_SSE4A: ::c_ulong = 160 + 6;
        pub const X86_FEATURE_AVX: ::c_ulong = 128 + 28;
        pub const X86_FEATURE_AVX_VNNI: ::c_ulong = 384 + 4;
        pub const X86_FEATURE_AVX2: ::c_ulong = 288 + 5;
        pub const X86_FEATURE_AVX512F: ::c_ulong = 288 + 16;
        pub const X86_FEATURE_AVX512DQ: ::c_ulong = 288 + 17;
        pub const X86_FEATURE_AVX512IFMA: ::c_ulong = 288 + 21;
        pub const X86_FEATURE_AVX512PF: ::c_ulong = 288 + 26;
        pub const X86_FEATURE_AVX512ER: ::c_ulong = 288 + 27;
        pub const X86_FEATURE_AVX512CD: ::c_ulong = 288 + 28;
        pub const X86_FEATURE_AVX512BW: ::c_ulong = 288 + 30;
        pub const X86_FEATURE_AVX512VL: ::c_ulong = 288 + 31;
        pub const X86_FEATURE_AVX512_BF16: ::c_ulong = 384 + 5;
        pub const X86_FEATURE_AVX512_VBMI2: ::c_ulong = 512 + 6;
        pub const X86_FEATURE_AVX512_VNNI: ::c_ulong = 512 + 11;
        pub const X86_FEATURE_AVX512_BITALG: ::c_ulong = 512 + 12;
        pub const X86_FEATURE_AVX512_VPOPCNTDQ: ::c_ulong = 512 + 14;
        pub const X86_FEATURE_AVX512_4VNNIW: ::c_ulong = 576 + 2;
        pub const X86_FEATURE_AVX512_4FMAPS: ::c_ulong = 576 + 3;
        pub const X86_FEATURE_AVX512_VP2INTERSECT: ::c_ulong = 576 + 8;
        pub const X86_FEATURE_AVX512_FP16: ::c_ulong = 576 + 23;
        pub const X86_FEATURE_AMX_BF16: ::c_ulong = 576 + 22;
        pub const X86_FEATURE_AMX_TILE: ::c_ulong = 576 + 24;
        pub const X86_FEATURE_AMX_INT8: ::c_ulong = 576 + 25;
        pub const X86_FEATURE_PCLMULQDQ: ::c_ulong = 128 + 1;
        pub const X86_FEATURE_SMX: ::c_ulong = 128 + 6;
        pub const X86_FEATURE_SGX: ::c_ulong = 288 + 2;
        pub const X86_FEATURE_CX16: ::c_ulong = 128 + 13;
        pub const X86_FEATURE_SHA_NI: ::c_ulong = 288 + 29;
        pub const X86_FEATURE_POPCNT: ::c_ulong = 128 + 23;
        pub const X86_FEATURE_MOVBE: ::c_ulong = 128 + 22;
        pub const X86_FEATURE_RDRAND: ::c_ulong = 128 + 30;
        pub const X86_FEATURE_DCA: ::c_ulong = 128 + 18;
        pub const X86_FEATURE_SSBD: ::c_ulong = 224 + 17;
        pub const X86_FEATURE_ADX: ::c_ulong = 288 + 19;
        pub const X86_FEATURE_ABM: ::c_ulong = 160 + 5;
        pub const X86_FEATURE_GFNI: ::c_ulong = 512 + 8;
        pub const X86_FEATURE_MOVDIRI: ::c_ulong = 512 + 27;
        pub const X86_FEATURE_MOVDIR64B: ::c_ulong = 512 + 28;
    }
}

// Defined in unix/linux_like/mod.rs
// pub const SCM_TIMESTAMP: ::c_int = SO_TIMESTAMP;
pub const SCM_TIMESTAMPNS: ::c_int = SO_TIMESTAMPNS;
pub const SCM_TIMESTAMPING: ::c_int = SO_TIMESTAMPING;

// Ioctl Constants

pub const TCGETS: ::Ioctl = 0x5401;
pub const TCSETS: ::Ioctl = 0x5402;
pub const TCSETSW: ::Ioctl = 0x5403;
pub const TCSETSF: ::Ioctl = 0x5404;
pub const TCGETA: ::Ioctl = 0x5405;
pub const TCSETA: ::Ioctl = 0x5406;
pub const TCSETAW: ::Ioctl = 0x5407;
pub const TCSETAF: ::Ioctl = 0x5408;
pub const TCSBRK: ::Ioctl = 0x5409;
pub const TCXONC: ::Ioctl = 0x540A;
pub const TCFLSH: ::Ioctl = 0x540B;
pub const TIOCEXCL: ::Ioctl = 0x540C;
pub const TIOCNXCL: ::Ioctl = 0x540D;
pub const TIOCSCTTY: ::Ioctl = 0x540E;
pub const TIOCGPGRP: ::Ioctl = 0x540F;
pub const TIOCSPGRP: ::Ioctl = 0x5410;
pub const TIOCOUTQ: ::Ioctl = 0x5411;
pub const TIOCSTI: ::Ioctl = 0x5412;
pub const TIOCGWINSZ: ::Ioctl = 0x5413;
pub const TIOCSWINSZ: ::Ioctl = 0x5414;
pub const TIOCMGET: ::Ioctl = 0x5415;
pub const TIOCMBIS: ::Ioctl = 0x5416;
pub const TIOCMBIC: ::Ioctl = 0x5417;
pub const TIOCMSET: ::Ioctl = 0x5418;
pub const TIOCGSOFTCAR: ::Ioctl = 0x5419;
pub const TIOCSSOFTCAR: ::Ioctl = 0x541A;
pub const FIONREAD: ::Ioctl = 0x541B;
pub const TIOCINQ: ::Ioctl = FIONREAD;
pub const TIOCLINUX: ::Ioctl = 0x541C;
pub const TIOCCONS: ::Ioctl = 0x541D;
pub const TIOCGSERIAL: ::Ioctl = 0x541E;
pub const TIOCSSERIAL: ::Ioctl = 0x541F;
pub const TIOCPKT: ::Ioctl = 0x5420;
pub const FIONBIO: ::Ioctl = 0x5421;
pub const TIOCNOTTY: ::Ioctl = 0x5422;
pub const TIOCSETD: ::Ioctl = 0x5423;
pub const TIOCGETD: ::Ioctl = 0x5424;
pub const TCSBRKP: ::Ioctl = 0x5425;
pub const TIOCSBRK: ::Ioctl = 0x5427;
pub const TIOCCBRK: ::Ioctl = 0x5428;
pub const TIOCGSID: ::Ioctl = 0x5429;
pub const TCGETS2: ::Ioctl = 0x802c542a;
pub const TCSETS2: ::Ioctl = 0x402c542b;
pub const TCSETSW2: ::Ioctl = 0x402c542c;
pub const TCSETSF2: ::Ioctl = 0x402c542d;
pub const TIOCGRS485: ::Ioctl = 0x542E;
pub const TIOCSRS485: ::Ioctl = 0x542F;
pub const TIOCGPTN: ::Ioctl = 0x80045430;
pub const TIOCSPTLCK: ::Ioctl = 0x40045431;
pub const TIOCGDEV: ::Ioctl = 0x80045432;
pub const TCGETX: ::Ioctl = 0x5432;
pub const TCSETX: ::Ioctl = 0x5433;
pub const TCSETXF: ::Ioctl = 0x5434;
pub const TCSETXW: ::Ioctl = 0x5435;
pub const TIOCSIG: ::Ioctl = 0x40045436;
pub const TIOCVHANGUP: ::Ioctl = 0x5437;
pub const TIOCGPKT: ::Ioctl = 0x80045438;
pub const TIOCGPTLCK: ::Ioctl = 0x80045439;
pub const TIOCGEXCL: ::Ioctl = 0x80045440;
pub const TIOCGPTPEER: ::Ioctl = 0x5441;
// pub const TIOCGISO7816: ::Ioctl = 0x80285442;
// pub const TIOCSISO7816: ::Ioctl = 0xc0285443;
pub const FIONCLEX: ::Ioctl = 0x5450;
pub const FIOCLEX: ::Ioctl = 0x5451;
pub const FIOASYNC: ::Ioctl = 0x5452;
pub const TIOCSERCONFIG: ::Ioctl = 0x5453;
pub const TIOCSERGWILD: ::Ioctl = 0x5454;
pub const TIOCSERSWILD: ::Ioctl = 0x5455;
pub const TIOCGLCKTRMIOS: ::Ioctl = 0x5456;
pub const TIOCSLCKTRMIOS: ::Ioctl = 0x5457;
pub const TIOCSERGSTRUCT: ::Ioctl = 0x5458;
pub const TIOCSERGETLSR: ::Ioctl = 0x5459;
pub const TIOCSERGETMULTI: ::Ioctl = 0x545A;
pub const TIOCSERSETMULTI: ::Ioctl = 0x545B;
pub const TIOCMIWAIT: ::Ioctl = 0x545C;
pub const TIOCGICOUNT: ::Ioctl = 0x545D;
pub const BLKIOMIN: ::Ioctl = 0x1278;
pub const BLKIOOPT: ::Ioctl = 0x1279;
pub const BLKSSZGET: ::Ioctl = 0x1268;
pub const BLKPBSZGET: ::Ioctl = 0x127B;
// linux/if_tun.h
pub const TUNSETNOCSUM: ::Ioctl = 0x400454c8;
pub const TUNSETDEBUG: ::Ioctl = 0x400454c9;
pub const TUNSETIFF: ::Ioctl = 0x400454ca;
pub const TUNSETPERSIST: ::Ioctl = 0x400454cb;
pub const TUNSETOWNER: ::Ioctl = 0x400454cc;
pub const TUNSETLINK: ::Ioctl = 0x400454cd;
pub const TUNSETGROUP: ::Ioctl = 0x400454ce;
pub const TUNGETFEATURES: ::Ioctl = 0x800454cf;
pub const TUNSETOFFLOAD: ::Ioctl = 0x400454d0;
pub const TUNSETTXFILTER: ::Ioctl = 0x400454d1;
pub const TUNGETIFF: ::Ioctl = 0x800454d2;
pub const TUNGETSNDBUF: ::Ioctl = 0x800454d3;
pub const TUNSETSNDBUF: ::Ioctl = 0x400454d4;
pub const TUNGETVNETHDRSZ: ::Ioctl = 0x800454d7;
pub const TUNSETVNETHDRSZ: ::Ioctl = 0x400454d8;
pub const TUNSETQUEUE: ::Ioctl = 0x400454d9;
pub const TUNSETIFINDEX: ::Ioctl = 0x400454da;
pub const TUNSETVNETLE: ::Ioctl = 0x400454dc;
pub const TUNGETVNETLE: ::Ioctl = 0x800454dd;
/* The TUNSETVNETBE and TUNGETVNETBE ioctls are for cross-endian support on
 * little-endian hosts. Not all kernel configurations support them, but all
 * configurations that support SET also support GET.
 */
pub const TUNSETVNETBE: ::Ioctl = 0x400454de;
pub const TUNGETVNETBE: ::Ioctl = 0x800454df;
pub const TUNSETSTEERINGEBPF: ::Ioctl = 0x800454e0;
pub const TUNSETFILTEREBPF: ::Ioctl = 0x800454e1;

cfg_if! {
    // Those type are constructed using the _IOC macro
    // DD-SS_SSSS_SSSS_SSSS-TTTT_TTTT-NNNN_NNNN
    // where D stands for direction (either None (00), Read (01) or Write (11))
    // where S stands for size (int, long, struct...)
    // where T stands for type ('f','v','X'...)
    // where N stands for NR (NumbeR)
    if #[cfg(any(
        target_arch = "x86",
        target_arch = "arm",
        target_arch = "csky"
    ))] {
        pub const FS_IOC_GETFLAGS: ::Ioctl = 0x80046601;
        pub const FS_IOC_SETFLAGS: ::Ioctl = 0x40046602;
        pub const FS_IOC_GETVERSION: ::Ioctl = 0x80047601;
        pub const FS_IOC_SETVERSION: ::Ioctl = 0x40047602;
        pub const FS_IOC32_GETFLAGS: ::Ioctl = 0x80046601;
        pub const FS_IOC32_SETFLAGS: ::Ioctl = 0x40046602;
        pub const FS_IOC32_GETVERSION: ::Ioctl = 0x80047601;
        pub const FS_IOC32_SETVERSION: ::Ioctl = 0x40047602;
        pub const TUNATTACHFILTER: ::Ioctl = 0x400854d5;
        pub const TUNDETACHFILTER: ::Ioctl = 0x400854d6;
        pub const TUNGETFILTER: ::Ioctl = 0x800854db;
    } else if #[cfg(any(
        target_arch = "x86_64",
        target_arch = "riscv64",
        target_arch = "aarch64",
        target_arch = "s390x",
        target_arch = "loongarch64"
    ))] {
        pub const FS_IOC_GETFLAGS: ::Ioctl = 0x80086601;
        pub const FS_IOC_SETFLAGS: ::Ioctl = 0x40086602;
        pub const FS_IOC_GETVERSION: ::Ioctl = 0x80087601;
        pub const FS_IOC_SETVERSION: ::Ioctl = 0x40087602;
        pub const FS_IOC32_GETFLAGS: ::Ioctl = 0x80046601;
        pub const FS_IOC32_SETFLAGS: ::Ioctl = 0x40046602;
        pub const FS_IOC32_GETVERSION: ::Ioctl = 0x80047601;
        pub const FS_IOC32_SETVERSION: ::Ioctl = 0x40047602;
        pub const TUNATTACHFILTER: ::Ioctl = 0x401054d5;
        pub const TUNDETACHFILTER: ::Ioctl = 0x401054d6;
        pub const TUNGETFILTER: ::Ioctl = 0x801054db;
    }
}

cfg_if! {
    if #[cfg(any(target_arch = "arm", target_arch = "s390x"))] {
        pub const FIOQSIZE: ::Ioctl = 0x545E;
    } else {
        pub const FIOQSIZE: ::Ioctl = 0x5460;
    }
}

pub const TIOCM_LE: ::c_int = 0x001;
pub const TIOCM_DTR: ::c_int = 0x002;
pub const TIOCM_RTS: ::c_int = 0x004;
pub const TIOCM_ST: ::c_int = 0x008;
pub const TIOCM_SR: ::c_int = 0x010;
pub const TIOCM_CTS: ::c_int = 0x020;
pub const TIOCM_CAR: ::c_int = 0x040;
pub const TIOCM_CD: ::c_int = TIOCM_CAR;
pub const TIOCM_RNG: ::c_int = 0x080;
pub const TIOCM_RI: ::c_int = TIOCM_RNG;
pub const TIOCM_DSR: ::c_int = 0x100;

pub const BOTHER: ::speed_t = 0o010000;
pub const IBSHIFT: ::tcflag_t = 16;

// RLIMIT Constants

cfg_if! {
    if #[cfg(any(target_env = "gnu", target_env = "uclibc"))] {
        pub const RLIMIT_CPU: ::__rlimit_resource_t = 0;
        pub const RLIMIT_FSIZE: ::__rlimit_resource_t = 1;
        pub const RLIMIT_DATA: ::__rlimit_resource_t = 2;
        pub const RLIMIT_STACK: ::__rlimit_resource_t = 3;
        pub const RLIMIT_CORE: ::__rlimit_resource_t = 4;
        pub const RLIMIT_RSS: ::__rlimit_resource_t = 5;
        pub const RLIMIT_NPROC: ::__rlimit_resource_t = 6;
        pub const RLIMIT_NOFILE: ::__rlimit_resource_t = 7;
        pub const RLIMIT_MEMLOCK: ::__rlimit_resource_t = 8;
        pub const RLIMIT_AS: ::__rlimit_resource_t = 9;
        pub const RLIMIT_LOCKS: ::__rlimit_resource_t = 10;
        pub const RLIMIT_SIGPENDING: ::__rlimit_resource_t = 11;
        pub const RLIMIT_MSGQUEUE: ::__rlimit_resource_t = 12;
        pub const RLIMIT_NICE: ::__rlimit_resource_t = 13;
        pub const RLIMIT_RTPRIO: ::__rlimit_resource_t = 14;
        pub const RLIMIT_RTTIME: ::__rlimit_resource_t = 15;
        #[allow(deprecated)]
        #[deprecated(since = "0.2.64", note = "Not stable across OS versions")]
        pub const RLIMIT_NLIMITS: ::__rlimit_resource_t = RLIM_NLIMITS;
    } else if #[cfg(any(target_env = "musl", target_env = "ohos"))] {
        pub const RLIMIT_CPU: ::c_int = 0;
        pub const RLIMIT_FSIZE: ::c_int = 1;
        pub const RLIMIT_DATA: ::c_int = 2;
        pub const RLIMIT_STACK: ::c_int = 3;
        pub const RLIMIT_CORE: ::c_int = 4;
        pub const RLIMIT_RSS: ::c_int = 5;
        pub const RLIMIT_NPROC: ::c_int = 6;
        pub const RLIMIT_NOFILE: ::c_int = 7;
        pub const RLIMIT_MEMLOCK: ::c_int = 8;
        pub const RLIMIT_AS: ::c_int = 9;
        pub const RLIMIT_LOCKS: ::c_int = 10;
        pub const RLIMIT_SIGPENDING: ::c_int = 11;
        pub const RLIMIT_MSGQUEUE: ::c_int = 12;
        pub const RLIMIT_NICE: ::c_int = 13;
        pub const RLIMIT_RTPRIO: ::c_int = 14;
        pub const RLIMIT_RTTIME: ::c_int = 15;
        #[deprecated(since = "0.2.64", note = "Not stable across OS versions")]
        #[cfg(not(target_arch = "loongarch64"))]
        pub const RLIM_NLIMITS: ::c_int = 15;
        #[cfg(target_arch = "loongarch64")]
        pub const RLIM_NLIMITS: ::c_int = 16;
        #[allow(deprecated)]
        #[deprecated(since = "0.2.64", note = "Not stable across OS versions")]
        pub const RLIMIT_NLIMITS: ::c_int = RLIM_NLIMITS;
    }
}

cfg_if! {
    if #[cfg(target_env = "gnu")] {
        #[deprecated(since = "0.2.64", note = "Not stable across OS versions")]
        pub const RLIM_NLIMITS: ::__rlimit_resource_t = 16;
    } else if #[cfg(target_env = "uclibc")] {
        #[deprecated(since = "0.2.64", note = "Not stable across OS versions")]
        pub const RLIM_NLIMITS: ::__rlimit_resource_t = 15;
    }
}

pub const RLIM_INFINITY: ::rlim_t = !0;
