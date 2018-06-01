//! Android-specific definitions for linux-like values

use dox::{mem, Option};

pub type clock_t = ::c_long;
pub type time_t = ::c_long;
pub type suseconds_t = ::c_long;
pub type off_t = ::c_long;
pub type blkcnt_t = ::c_ulong;
pub type blksize_t = ::c_ulong;
pub type nlink_t = u32;
pub type useconds_t = u32;
pub type pthread_t = ::c_long;
pub type pthread_mutexattr_t = ::c_long;
pub type pthread_rwlockattr_t = ::c_long;
pub type pthread_condattr_t = ::c_long;
pub type fsfilcnt_t = ::c_ulong;
pub type fsblkcnt_t = ::c_ulong;
pub type nfds_t = ::c_uint;
pub type rlim_t = ::c_ulong;
pub type dev_t = ::c_ulong;
pub type ino_t = ::c_ulong;
pub type __CPU_BITTYPE = ::c_ulong;
pub type idtype_t = ::c_int;
pub type loff_t = ::c_longlong;

s! {
    pub struct dirent {
        pub d_ino: u64,
        pub d_off: i64,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct dirent64 {
        pub d_ino: u64,
        pub d_off: i64,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub _pad: [::c_int; 29],
        _align: [usize; 0],
    }

    pub struct __fsid_t {
        __val: [::c_int; 2],
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::size_t,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::size_t,
        pub msg_flags: ::c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: ::size_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; ::NCCS],
    }

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

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
    }

    pub struct cpu_set_t {
        #[cfg(target_pointer_width = "64")]
        __bits: [__CPU_BITTYPE; 16],
        #[cfg(target_pointer_width = "32")]
        __bits: [__CPU_BITTYPE; 1],
    }

    pub struct sem_t {
        count: ::c_uint,
        #[cfg(target_pointer_width = "64")]
        __reserved: [::c_int; 3],
    }

    pub struct lastlog {
        ll_time: ::time_t,
        ll_line: [::c_char; UT_LINESIZE],
        ll_host: [::c_char; UT_HOSTSIZE],
    }

    pub struct exit_status {
        pub e_termination: ::c_short,
        pub e_exit: ::c_short,
    }

    pub struct utmp {
        pub ut_type: ::c_short,
        pub ut_pid: ::pid_t,
        pub ut_line: [::c_char; UT_LINESIZE],
        pub ut_id: [::c_char; 4],

        pub ut_user: [::c_char; UT_NAMESIZE],
        pub ut_host: [::c_char; UT_HOSTSIZE],
        pub ut_exit: exit_status,
        pub ut_session: ::c_long,
        pub ut_tv: ::timeval,

        pub ut_addr_v6: [::int32_t; 4],
        unused: [::c_char; 20],
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        #[cfg(target_pointer_width = "64")]
        __f_reserved: [u32; 6],
    }

    pub struct signalfd_siginfo {
        pub ssi_signo: ::uint32_t,
        pub ssi_errno: ::int32_t,
        pub ssi_code: ::int32_t,
        pub ssi_pid: ::uint32_t,
        pub ssi_uid: ::uint32_t,
        pub ssi_fd: ::int32_t,
        pub ssi_tid: ::uint32_t,
        pub ssi_band: ::uint32_t,
        pub ssi_overrun: ::uint32_t,
        pub ssi_trapno: ::uint32_t,
        pub ssi_status: ::int32_t,
        pub ssi_int: ::int32_t,
        pub ssi_ptr: ::c_ulonglong,
        pub ssi_utime: ::c_ulonglong,
        pub ssi_stime: ::c_ulonglong,
        pub ssi_addr: ::c_ulonglong,
        pub ssi_addr_lsb: ::uint16_t,
        _pad: [::uint8_t; 46],
    }

    pub struct ucred {
        pub pid: ::pid_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
    }

    pub struct genlmsghdr {
        pub cmd: u8,
        pub version: u8,
        pub reserved: u16,
    }

    pub struct nlmsghdr {
        pub nlmsg_len: u32,
        pub nlmsg_type: u16,
        pub nlmsg_flags: u16,
        pub nlmsg_seq: u32,
        pub nlmsg_pid: u32,
    }

    pub struct nlmsgerr {
        pub error: ::c_int,
        pub msg: nlmsghdr,
    }

    pub struct nl_pktinfo {
        pub group: u32,
    }

    pub struct nl_mmap_req {
        pub nm_block_size: ::c_uint,
        pub nm_block_nr: ::c_uint,
        pub nm_frame_size: ::c_uint,
        pub nm_frame_nr: ::c_uint,
    }

    pub struct nl_mmap_hdr {
        pub nm_status: ::c_uint,
        pub nm_len: ::c_uint,
        pub nm_group: u32,
        pub nm_pid: u32,
        pub nm_uid: u32,
        pub nm_gid: u32,
    }

    pub struct nlattr {
        pub nla_len: u16,
        pub nla_type: u16,
    }

    pub struct in6_pktinfo {
        pub ipi6_addr: ::in6_addr,
        pub ipi6_ifindex: ::c_int,
    }

    pub struct rtattr {
        rta_len: ::c_ushort,
        rta_type: ::c_ushort,
    }

    pub struct rtmsg {
        rtm_family: ::c_uchar,
        rtm_dst_len: ::c_uchar,
        rtm_src_len: ::c_uchar,
        rtm_tos: ::c_uchar,
        rtm_table: ::c_uchar,
        rtm_protocol: ::c_uchar,
        rtm_scope: ::c_uchar,
        rtm_type: ::c_uchar,
        rtm_flags: ::c_uint,
    }
    
    pub struct rtnexthop {
        rtnh_len: ::c_ushort,
        rtnh_flags: ::c_uchar,
        rtnh_hops: ::c_uchar,
        rtnh_ifindex: ::c_int,
    }

    pub struct rta_cacheinfo {
        rta_clntref: u32,
        rta_lastuse: u32,
        rta_expires: u32,
        rta_error: u32,
        rta_used: u32,
        rta_id: u32,
        rta_ts: u32,
        rta_tsage: u32,
    }

    pub struct rta_mfc_stats {
        mfcs_packets: u64,
        mfcs_bytes: u64,
        mfcs_wrong_if: u64,
    }

    pub struct rtgenmsg {
        rtgen_family: ::c_uchar,
    }

    pub struct ifinfomsg {
        ifi_family: ::c_uchar,
        __ifi_pad: ::c_uchar,
        ifi_type: ::c_ushort,
        ifi_index: ::c_int,
        ifi_flags: ::c_uint,
        ifi_change: ::c_uint,
    }

    pub struct prefixmsg {
        prefix_family: ::c_uchar,
        prefix_pad1: ::c_uchar,
        prefix_pad2: ::c_ushort,
        prefix_ifindex: ::c_int,
        prefix_type: ::c_uchar,
        prefix_len: ::c_uchar,
        prefix_flags: ::c_uchar,
        prefix_pad3: ::c_uchar,
    }

    pub struct prefix_cacheinfo {
        preferred_time: u32,
        valid_time: u32,
    }

    pub struct tcmsg {
        tcm_family: ::c_uchar,
        tcm__pad1: ::c_uchar,
        tcm__pad2: ::c_ushort,
        tcm_ifindex: ::c_int,
        tcm_handle: u32,
        tcm_parent: u32,
        tcm_info: u32,
    }

    pub struct nduseroptmsg {
        nduseropt_family: ::c_uchar,
        nduseropt_pad1: ::c_uchar,
        nduseropt_opts_len: ::c_ushort,
        nduseropt_ifindex: ::c_int,
        nduseropt_icmp_type: u8,
        nduseropt_icmp_code: u8,
        nduseropt_pad2: ::c_ushort,
        nduseropt_pad3: ::c_uint,
    }

    pub struct tcamsg {
        tca_family: ::c_uchar,
        tca_pad1: ::c_uchar,
        tca_pad2: ::c_ushort,
    }
}

pub const O_TRUNC: ::c_int = 512;
pub const O_CLOEXEC: ::c_int = 0x80000;
pub const O_PATH: ::c_int = 0o10000000;
pub const O_NOATIME: ::c_int = 0o1000000;

pub const EBFONT: ::c_int = 59;
pub const ENOSTR: ::c_int = 60;
pub const ENODATA: ::c_int = 61;
pub const ETIME: ::c_int = 62;
pub const ENOSR: ::c_int = 63;
pub const ENONET: ::c_int = 64;
pub const ENOPKG: ::c_int = 65;
pub const EREMOTE: ::c_int = 66;
pub const ENOLINK: ::c_int = 67;
pub const EADV: ::c_int = 68;
pub const ESRMNT: ::c_int = 69;
pub const ECOMM: ::c_int = 70;
pub const EPROTO: ::c_int = 71;
pub const EDOTDOT: ::c_int = 73;

pub const SA_NODEFER: ::c_int = 0x40000000;
pub const SA_RESETHAND: ::c_int = 0x80000000;
pub const SA_RESTART: ::c_int = 0x10000000;
pub const SA_NOCLDSTOP: ::c_int = 0x00000001;

pub const EPOLL_CLOEXEC: ::c_int = 0x80000;
pub const EPOLLONESHOT: ::c_int = 0x40000000;
pub const EPOLLRDHUP: ::c_int = 0x00002000;
pub const EPOLLWAKEUP: ::c_int = 0x20000000;

pub const EFD_CLOEXEC: ::c_int = 0x80000;

pub const USER_PROCESS: ::c_short = 7;

pub const BUFSIZ: ::c_uint = 1024;
pub const FILENAME_MAX: ::c_uint = 1024;
pub const FOPEN_MAX: ::c_uint = 20;
pub const POSIX_FADV_DONTNEED: ::c_int = 4;
pub const POSIX_FADV_NOREUSE: ::c_int = 5;
pub const L_tmpnam: ::c_uint = 1024;
pub const TMP_MAX: ::c_uint = 308915776;
pub const _PC_LINK_MAX: ::c_int = 1;
pub const _PC_MAX_CANON: ::c_int = 2;
pub const _PC_MAX_INPUT: ::c_int = 3;
pub const _PC_NAME_MAX: ::c_int = 4;
pub const _PC_PATH_MAX: ::c_int = 5;
pub const _PC_PIPE_BUF: ::c_int = 6;
pub const _PC_2_SYMLINKS: ::c_int = 7;
pub const _PC_ALLOC_SIZE_MIN: ::c_int = 8;
pub const _PC_REC_INCR_XFER_SIZE: ::c_int = 9;
pub const _PC_REC_MAX_XFER_SIZE: ::c_int = 10;
pub const _PC_REC_MIN_XFER_SIZE: ::c_int = 11;
pub const _PC_REC_XFER_ALIGN: ::c_int = 12;
pub const _PC_SYMLINK_MAX: ::c_int = 13;
pub const _PC_CHOWN_RESTRICTED: ::c_int = 14;
pub const _PC_NO_TRUNC: ::c_int = 15;
pub const _PC_VDISABLE: ::c_int = 16;
pub const _PC_ASYNC_IO: ::c_int = 17;
pub const _PC_PRIO_IO: ::c_int = 18;
pub const _PC_SYNC_IO: ::c_int = 19;

pub const FIONBIO: ::c_int = 0x5421;

pub const _SC_ARG_MAX: ::c_int = 0;
pub const _SC_BC_BASE_MAX: ::c_int = 1;
pub const _SC_BC_DIM_MAX: ::c_int = 2;
pub const _SC_BC_SCALE_MAX: ::c_int = 3;
pub const _SC_BC_STRING_MAX: ::c_int = 4;
pub const _SC_CHILD_MAX: ::c_int = 5;
pub const _SC_CLK_TCK: ::c_int = 6;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 7;
pub const _SC_EXPR_NEST_MAX: ::c_int = 8;
pub const _SC_LINE_MAX: ::c_int = 9;
pub const _SC_NGROUPS_MAX: ::c_int = 10;
pub const _SC_OPEN_MAX: ::c_int = 11;
pub const _SC_PASS_MAX: ::c_int = 12;
pub const _SC_2_C_BIND: ::c_int = 13;
pub const _SC_2_C_DEV: ::c_int = 14;
pub const _SC_2_C_VERSION: ::c_int = 15;
pub const _SC_2_CHAR_TERM: ::c_int = 16;
pub const _SC_2_FORT_DEV: ::c_int = 17;
pub const _SC_2_FORT_RUN: ::c_int = 18;
pub const _SC_2_LOCALEDEF: ::c_int = 19;
pub const _SC_2_SW_DEV: ::c_int = 20;
pub const _SC_2_UPE: ::c_int = 21;
pub const _SC_2_VERSION: ::c_int = 22;
pub const _SC_JOB_CONTROL: ::c_int = 23;
pub const _SC_SAVED_IDS: ::c_int = 24;
pub const _SC_VERSION: ::c_int = 25;
pub const _SC_RE_DUP_MAX: ::c_int = 26;
pub const _SC_STREAM_MAX: ::c_int = 27;
pub const _SC_TZNAME_MAX: ::c_int = 28;
pub const _SC_XOPEN_CRYPT: ::c_int = 29;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 30;
pub const _SC_XOPEN_SHM: ::c_int = 31;
pub const _SC_XOPEN_VERSION: ::c_int = 32;
pub const _SC_XOPEN_XCU_VERSION: ::c_int = 33;
pub const _SC_XOPEN_REALTIME: ::c_int = 34;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 35;
pub const _SC_XOPEN_LEGACY: ::c_int = 36;
pub const _SC_ATEXIT_MAX: ::c_int = 37;
pub const _SC_IOV_MAX: ::c_int = 38;
pub const _SC_PAGESIZE: ::c_int = 39;
pub const _SC_PAGE_SIZE: ::c_int = 40;
pub const _SC_XOPEN_UNIX: ::c_int = 41;
pub const _SC_XBS5_ILP32_OFF32: ::c_int = 42;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_int = 43;
pub const _SC_XBS5_LP64_OFF64: ::c_int = 44;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_int = 45;
pub const _SC_AIO_LISTIO_MAX: ::c_int = 46;
pub const _SC_AIO_MAX: ::c_int = 47;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_int = 48;
pub const _SC_DELAYTIMER_MAX: ::c_int = 49;
pub const _SC_MQ_OPEN_MAX: ::c_int = 50;
pub const _SC_MQ_PRIO_MAX: ::c_int = 51;
pub const _SC_RTSIG_MAX: ::c_int = 52;
pub const _SC_SEM_NSEMS_MAX: ::c_int = 53;
pub const _SC_SEM_VALUE_MAX: ::c_int = 54;
pub const _SC_SIGQUEUE_MAX: ::c_int = 55;
pub const _SC_TIMER_MAX: ::c_int = 56;
pub const _SC_ASYNCHRONOUS_IO: ::c_int = 57;
pub const _SC_FSYNC: ::c_int = 58;
pub const _SC_MAPPED_FILES: ::c_int = 59;
pub const _SC_MEMLOCK: ::c_int = 60;
pub const _SC_MEMLOCK_RANGE: ::c_int = 61;
pub const _SC_MEMORY_PROTECTION: ::c_int = 62;
pub const _SC_MESSAGE_PASSING: ::c_int = 63;
pub const _SC_PRIORITIZED_IO: ::c_int = 64;
pub const _SC_PRIORITY_SCHEDULING: ::c_int = 65;
pub const _SC_REALTIME_SIGNALS: ::c_int = 66;
pub const _SC_SEMAPHORES: ::c_int = 67;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_int = 68;
pub const _SC_SYNCHRONIZED_IO: ::c_int = 69;
pub const _SC_TIMERS: ::c_int = 70;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 71;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 72;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 73;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 74;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 75;
pub const _SC_THREAD_STACK_MIN: ::c_int = 76;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 77;
pub const _SC_TTY_NAME_MAX: ::c_int = 78;
pub const _SC_THREADS: ::c_int = 79;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 80;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 81;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 82;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 83;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 84;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 85;
pub const _SC_NPROCESSORS_CONF: ::c_int = 96;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 97;
pub const _SC_PHYS_PAGES: ::c_int = 98;
pub const _SC_AVPHYS_PAGES: ::c_int = 99;
pub const _SC_MONOTONIC_CLOCK: ::c_int = 100;

pub const _SC_2_PBS: ::c_int = 101;
pub const _SC_2_PBS_ACCOUNTING: ::c_int = 102;
pub const _SC_2_PBS_CHECKPOINT: ::c_int = 103;
pub const _SC_2_PBS_LOCATE: ::c_int = 104;
pub const _SC_2_PBS_MESSAGE: ::c_int = 105;
pub const _SC_2_PBS_TRACK: ::c_int = 106;
pub const _SC_ADVISORY_INFO: ::c_int = 107;
pub const _SC_BARRIERS: ::c_int = 108;
pub const _SC_CLOCK_SELECTION: ::c_int = 109;
pub const _SC_CPUTIME: ::c_int = 110;
pub const _SC_HOST_NAME_MAX: ::c_int = 111;
pub const _SC_IPV6: ::c_int = 112;
pub const _SC_RAW_SOCKETS: ::c_int = 113;
pub const _SC_READER_WRITER_LOCKS: ::c_int = 114;
pub const _SC_REGEXP: ::c_int = 115;
pub const _SC_SHELL: ::c_int = 116;
pub const _SC_SPAWN: ::c_int = 117;
pub const _SC_SPIN_LOCKS: ::c_int = 118;
pub const _SC_SPORADIC_SERVER: ::c_int = 119;
pub const _SC_SS_REPL_MAX: ::c_int = 120;
pub const _SC_SYMLOOP_MAX: ::c_int = 121;
pub const _SC_THREAD_CPUTIME: ::c_int = 122;
pub const _SC_THREAD_PROCESS_SHARED: ::c_int = 123;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: ::c_int = 124;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: ::c_int = 125;
pub const _SC_THREAD_SPORADIC_SERVER: ::c_int = 126;
pub const _SC_TIMEOUTS: ::c_int = 127;
pub const _SC_TRACE: ::c_int = 128;
pub const _SC_TRACE_EVENT_FILTER: ::c_int = 129;
pub const _SC_TRACE_EVENT_NAME_MAX: ::c_int = 130;
pub const _SC_TRACE_INHERIT: ::c_int = 131;
pub const _SC_TRACE_LOG: ::c_int = 132;
pub const _SC_TRACE_NAME_MAX: ::c_int = 133;
pub const _SC_TRACE_SYS_MAX: ::c_int = 134;
pub const _SC_TRACE_USER_EVENT_MAX: ::c_int = 135;
pub const _SC_TYPED_MEMORY_OBJECTS: ::c_int = 136;
pub const _SC_V7_ILP32_OFF32: ::c_int = 137;
pub const _SC_V7_ILP32_OFFBIG: ::c_int = 138;
pub const _SC_V7_LP64_OFF64: ::c_int = 139;
pub const _SC_V7_LPBIG_OFFBIG: ::c_int = 140;
pub const _SC_XOPEN_STREAMS: ::c_int = 141;
pub const _SC_XOPEN_UUCP: ::c_int = 142;

pub const PTHREAD_MUTEX_NORMAL: ::c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = PTHREAD_MUTEX_NORMAL;

pub const FIOCLEX: ::c_int = 0x5451;

pub const SA_ONSTACK: ::c_ulong = 0x08000000;
pub const SA_SIGINFO: ::c_ulong = 0x00000004;
pub const SA_NOCLDWAIT: ::c_ulong = 0x00000002;
pub const SIGCHLD: ::c_int = 17;
pub const SIGBUS: ::c_int = 7;
pub const SIGUSR1: ::c_int = 10;
pub const SIGUSR2: ::c_int = 12;
pub const SIGCONT: ::c_int = 18;
pub const SIGSTOP: ::c_int = 19;
pub const SIGTSTP: ::c_int = 20;
pub const SIGURG: ::c_int = 23;
pub const SIGIO: ::c_int = 29;
pub const SIGSYS: ::c_int = 31;
pub const SIGSTKFLT: ::c_int = 16;
pub const SIGUNUSED: ::c_int = 31;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;
pub const SIGPOLL: ::c_int = 29;
pub const SIGPWR: ::c_int = 30;
pub const SIG_SETMASK: ::c_int = 2;
pub const SIG_BLOCK: ::c_int = 0x000000;
pub const SIG_UNBLOCK: ::c_int = 0x01;

pub const RUSAGE_CHILDREN: ::c_int = -1;

pub const LC_PAPER: ::c_int = 7;
pub const LC_NAME: ::c_int = 8;
pub const LC_ADDRESS: ::c_int = 9;
pub const LC_TELEPHONE: ::c_int = 10;
pub const LC_MEASUREMENT: ::c_int = 11;
pub const LC_IDENTIFICATION: ::c_int = 12;
pub const LC_PAPER_MASK: ::c_int = (1 << LC_PAPER);
pub const LC_NAME_MASK: ::c_int = (1 << LC_NAME);
pub const LC_ADDRESS_MASK: ::c_int = (1 << LC_ADDRESS);
pub const LC_TELEPHONE_MASK: ::c_int = (1 << LC_TELEPHONE);
pub const LC_MEASUREMENT_MASK: ::c_int = (1 << LC_MEASUREMENT);
pub const LC_IDENTIFICATION_MASK: ::c_int = (1 << LC_IDENTIFICATION);
pub const LC_ALL_MASK: ::c_int = ::LC_CTYPE_MASK
                               | ::LC_NUMERIC_MASK
                               | ::LC_TIME_MASK
                               | ::LC_COLLATE_MASK
                               | ::LC_MONETARY_MASK
                               | ::LC_MESSAGES_MASK
                               | LC_PAPER_MASK
                               | LC_NAME_MASK
                               | LC_ADDRESS_MASK
                               | LC_TELEPHONE_MASK
                               | LC_MEASUREMENT_MASK
                               | LC_IDENTIFICATION_MASK;

pub const MAP_ANON: ::c_int = 0x0020;
pub const MAP_ANONYMOUS: ::c_int = 0x0020;
pub const MAP_GROWSDOWN: ::c_int = 0x0100;
pub const MAP_DENYWRITE: ::c_int = 0x0800;
pub const MAP_EXECUTABLE: ::c_int = 0x01000;
pub const MAP_LOCKED: ::c_int = 0x02000;
pub const MAP_NORESERVE: ::c_int = 0x04000;
pub const MAP_POPULATE: ::c_int = 0x08000;
pub const MAP_NONBLOCK: ::c_int = 0x010000;
pub const MAP_STACK: ::c_int = 0x020000;

pub const EDEADLK: ::c_int = 35;
pub const ENAMETOOLONG: ::c_int = 36;
pub const ENOLCK: ::c_int = 37;
pub const ENOSYS: ::c_int = 38;
pub const ENOTEMPTY: ::c_int = 39;
pub const ELOOP: ::c_int = 40;
pub const ENOMSG: ::c_int = 42;
pub const EIDRM: ::c_int = 43;
pub const ECHRNG: ::c_int = 44;
pub const EL2NSYNC: ::c_int = 45;
pub const EL3HLT: ::c_int = 46;
pub const EL3RST: ::c_int = 47;
pub const ELNRNG: ::c_int = 48;
pub const EUNATCH: ::c_int = 49;
pub const ENOCSI: ::c_int = 50;
pub const EL2HLT: ::c_int = 51;
pub const EBADE: ::c_int = 52;
pub const EBADR: ::c_int = 53;
pub const EXFULL: ::c_int = 54;
pub const ENOANO: ::c_int = 55;
pub const EBADRQC: ::c_int = 56;
pub const EBADSLT: ::c_int = 57;

pub const EMULTIHOP: ::c_int = 72;
pub const EBADMSG: ::c_int = 74;
pub const EOVERFLOW: ::c_int = 75;
pub const ENOTUNIQ: ::c_int = 76;
pub const EBADFD: ::c_int = 77;
pub const EREMCHG: ::c_int = 78;
pub const ELIBACC: ::c_int = 79;
pub const ELIBBAD: ::c_int = 80;
pub const ELIBSCN: ::c_int = 81;
pub const ELIBMAX: ::c_int = 82;
pub const ELIBEXEC: ::c_int = 83;
pub const EILSEQ: ::c_int = 84;
pub const ERESTART: ::c_int = 85;
pub const ESTRPIPE: ::c_int = 86;
pub const EUSERS: ::c_int = 87;
pub const ENOTSOCK: ::c_int = 88;
pub const EDESTADDRREQ: ::c_int = 89;
pub const EMSGSIZE: ::c_int = 90;
pub const EPROTOTYPE: ::c_int = 91;
pub const ENOPROTOOPT: ::c_int = 92;
pub const EPROTONOSUPPORT: ::c_int = 93;
pub const ESOCKTNOSUPPORT: ::c_int = 94;
pub const EOPNOTSUPP: ::c_int = 95;
pub const ENOTSUP: ::c_int = EOPNOTSUPP;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const EAFNOSUPPORT: ::c_int = 97;
pub const EADDRINUSE: ::c_int = 98;
pub const EADDRNOTAVAIL: ::c_int = 99;
pub const ENETDOWN: ::c_int = 100;
pub const ENETUNREACH: ::c_int = 101;
pub const ENETRESET: ::c_int = 102;
pub const ECONNABORTED: ::c_int = 103;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EISCONN: ::c_int = 106;
pub const ENOTCONN: ::c_int = 107;
pub const ESHUTDOWN: ::c_int = 108;
pub const ETOOMANYREFS: ::c_int = 109;
pub const ETIMEDOUT: ::c_int = 110;
pub const ECONNREFUSED: ::c_int = 111;
pub const EHOSTDOWN: ::c_int = 112;
pub const EHOSTUNREACH: ::c_int = 113;
pub const EALREADY: ::c_int = 114;
pub const EINPROGRESS: ::c_int = 115;
pub const ESTALE: ::c_int = 116;
pub const EUCLEAN: ::c_int = 117;
pub const ENOTNAM: ::c_int = 118;
pub const ENAVAIL: ::c_int = 119;
pub const EISNAM: ::c_int = 120;
pub const EREMOTEIO: ::c_int = 121;
pub const EDQUOT: ::c_int = 122;
pub const ENOMEDIUM: ::c_int = 123;
pub const EMEDIUMTYPE: ::c_int = 124;
pub const ECANCELED: ::c_int = 125;
pub const ENOKEY: ::c_int = 126;
pub const EKEYEXPIRED: ::c_int = 127;
pub const EKEYREVOKED: ::c_int = 128;
pub const EKEYREJECTED: ::c_int = 129;
pub const EOWNERDEAD: ::c_int = 130;
pub const ENOTRECOVERABLE: ::c_int = 131;

pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_SEQPACKET: ::c_int = 5;
pub const SOCK_DCCP: ::c_int = 6;
pub const SOCK_PACKET: ::c_int = 10;

pub const SOL_SOCKET: ::c_int = 1;
pub const SOL_SCTP: ::c_int = 132;
pub const SOL_IPX: ::c_int = 256;
pub const SOL_AX25: ::c_int = 257;
pub const SOL_ATALK: ::c_int = 258;
pub const SOL_NETROM: ::c_int = 259;
pub const SOL_ROSE: ::c_int = 260;

#[doc(hidden)]
pub const AF_MAX: ::c_int = 43;
#[doc(hidden)]
pub const PF_MAX: ::c_int = AF_MAX;

/* DCCP socket options */
pub const DCCP_SOCKOPT_PACKET_SIZE: ::c_int = 1;
pub const DCCP_SOCKOPT_SERVICE: ::c_int = 2;
pub const DCCP_SOCKOPT_CHANGE_L: ::c_int = 3;
pub const DCCP_SOCKOPT_CHANGE_R: ::c_int = 4;
pub const DCCP_SOCKOPT_GET_CUR_MPS: ::c_int = 5;
pub const DCCP_SOCKOPT_SERVER_TIMEWAIT: ::c_int = 6;
pub const DCCP_SOCKOPT_SEND_CSCOV: ::c_int = 10;
pub const DCCP_SOCKOPT_RECV_CSCOV: ::c_int = 11;
pub const DCCP_SOCKOPT_AVAILABLE_CCIDS: ::c_int = 12;
pub const DCCP_SOCKOPT_CCID: ::c_int = 13;
pub const DCCP_SOCKOPT_TX_CCID: ::c_int = 14;
pub const DCCP_SOCKOPT_RX_CCID: ::c_int = 15;
pub const DCCP_SOCKOPT_QPOLICY_ID: ::c_int = 16;
pub const DCCP_SOCKOPT_QPOLICY_TXQLEN: ::c_int = 17;
pub const DCCP_SOCKOPT_CCID_RX_INFO: ::c_int = 128;
pub const DCCP_SOCKOPT_CCID_TX_INFO: ::c_int = 192;

/// maximum number of services provided on the same listening port
pub const DCCP_SERVICE_LIST_MAX_LEN: ::c_int = 32;

pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_TYPE: ::c_int = 3;
pub const SO_ERROR: ::c_int = 4;
pub const SO_DONTROUTE: ::c_int = 5;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_SNDBUF: ::c_int = 7;
pub const SO_RCVBUF: ::c_int = 8;
pub const SO_KEEPALIVE: ::c_int = 9;
pub const SO_OOBINLINE: ::c_int = 10;
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
pub const SO_BINDTODEVICE: ::c_int = 25;
pub const SO_TIMESTAMP: ::c_int = 29;
pub const SO_ACCEPTCONN: ::c_int = 30;
pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_MARK: ::c_int = 36;
pub const SO_PROTOCOL: ::c_int = 38;
pub const SO_DOMAIN: ::c_int = 39;
pub const SO_RXQ_OVFL: ::c_int = 40;
pub const SO_PEEK_OFF: ::c_int = 42;
pub const SO_BUSY_POLL: ::c_int = 46;

pub const O_ACCMODE: ::c_int = 3;
pub const O_APPEND: ::c_int = 1024;
pub const O_CREAT: ::c_int = 64;
pub const O_EXCL: ::c_int = 128;
pub const O_NOCTTY: ::c_int = 256;
pub const O_NONBLOCK: ::c_int = 2048;
pub const O_SYNC: ::c_int = 0x101000;
pub const O_ASYNC: ::c_int = 0x2000;
pub const O_NDELAY: ::c_int = 0x800;
pub const O_DSYNC: ::c_int = 4096;

pub const NI_MAXHOST: ::size_t = 1025;

pub const NCCS: usize = 19;
pub const TCSBRKP: ::c_int = 0x5425;
pub const TCSANOW: ::c_int = 0;
pub const TCSADRAIN: ::c_int = 0x1;
pub const TCSAFLUSH: ::c_int = 0x2;
pub const VEOF: usize = 4;
pub const VEOL: usize = 11;
pub const VEOL2: usize = 16;
pub const VMIN: usize = 6;
pub const IEXTEN: ::tcflag_t = 0x00008000;
pub const TOSTOP: ::tcflag_t = 0x00000100;
pub const FLUSHO: ::tcflag_t = 0x00001000;
pub const EXTPROC: ::tcflag_t = 0o200000;

pub const ADFS_SUPER_MAGIC: ::c_long = 0x0000adf5;
pub const AFFS_SUPER_MAGIC: ::c_long = 0x0000adff;
pub const CODA_SUPER_MAGIC: ::c_long = 0x73757245;
pub const CRAMFS_MAGIC: ::c_long = 0x28cd3d45;
pub const EFS_SUPER_MAGIC: ::c_long = 0x00414a53;
pub const EXT2_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const EXT3_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const EXT4_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const HPFS_SUPER_MAGIC: ::c_long = 0xf995e849;
pub const HUGETLBFS_MAGIC: ::c_long = 0x958458f6;
pub const ISOFS_SUPER_MAGIC: ::c_long = 0x00009660;
pub const JFFS2_SUPER_MAGIC: ::c_long = 0x000072b6;
pub const MINIX_SUPER_MAGIC: ::c_long = 0x0000137f;
pub const MINIX_SUPER_MAGIC2: ::c_long = 0x0000138f;
pub const MINIX2_SUPER_MAGIC: ::c_long = 0x00002468;
pub const MINIX2_SUPER_MAGIC2: ::c_long = 0x00002478;
pub const MSDOS_SUPER_MAGIC: ::c_long = 0x00004d44;
pub const NCP_SUPER_MAGIC: ::c_long = 0x0000564c;
pub const NFS_SUPER_MAGIC: ::c_long = 0x00006969;
pub const OPENPROM_SUPER_MAGIC: ::c_long = 0x00009fa1;
pub const PROC_SUPER_MAGIC: ::c_long = 0x00009fa0;
pub const QNX4_SUPER_MAGIC: ::c_long = 0x0000002f;
pub const REISERFS_SUPER_MAGIC: ::c_long = 0x52654973;
pub const SMB_SUPER_MAGIC: ::c_long = 0x0000517b;
pub const TMPFS_MAGIC: ::c_long = 0x01021994;
pub const USBDEVICE_SUPER_MAGIC: ::c_long = 0x00009fa2;

pub const MAP_HUGETLB: ::c_int = 0x040000;

pub const PTRACE_TRACEME: ::c_int = 0;
pub const PTRACE_PEEKTEXT: ::c_int = 1;
pub const PTRACE_PEEKDATA: ::c_int = 2;
pub const PTRACE_PEEKUSER: ::c_int = 3;
pub const PTRACE_POKETEXT: ::c_int = 4;
pub const PTRACE_POKEDATA: ::c_int = 5;
pub const PTRACE_POKEUSER: ::c_int = 6;
pub const PTRACE_CONT: ::c_int = 7;
pub const PTRACE_KILL: ::c_int = 8;
pub const PTRACE_SINGLESTEP: ::c_int = 9;
pub const PTRACE_ATTACH: ::c_int = 16;
pub const PTRACE_DETACH: ::c_int = 17;
pub const PTRACE_SYSCALL: ::c_int = 24;
pub const PTRACE_SETOPTIONS: ::c_int = 0x4200;
pub const PTRACE_GETEVENTMSG: ::c_int = 0x4201;
pub const PTRACE_GETSIGINFO: ::c_int = 0x4202;
pub const PTRACE_SETSIGINFO: ::c_int = 0x4203;

pub const EFD_NONBLOCK: ::c_int = 0x800;

pub const F_GETLK: ::c_int = 5;
pub const F_GETOWN: ::c_int = 9;
pub const F_SETOWN: ::c_int = 8;
pub const F_SETLK: ::c_int = 6;
pub const F_SETLKW: ::c_int = 7;

pub const TCGETS: ::c_int = 0x5401;
pub const TCSETS: ::c_int = 0x5402;
pub const TCSETSW: ::c_int = 0x5403;
pub const TCSETSF: ::c_int = 0x5404;
pub const TCGETA: ::c_int = 0x5405;
pub const TCSETA: ::c_int = 0x5406;
pub const TCSETAW: ::c_int = 0x5407;
pub const TCSETAF: ::c_int = 0x5408;
pub const TCSBRK: ::c_int = 0x5409;
pub const TCXONC: ::c_int = 0x540A;
pub const TCFLSH: ::c_int = 0x540B;
pub const TIOCGSOFTCAR: ::c_int = 0x5419;
pub const TIOCSSOFTCAR: ::c_int = 0x541A;
pub const TIOCINQ: ::c_int = 0x541B;
pub const TIOCLINUX: ::c_int = 0x541C;
pub const TIOCGSERIAL: ::c_int = 0x541E;
pub const TIOCEXCL: ::c_int = 0x540C;
pub const TIOCNXCL: ::c_int = 0x540D;
pub const TIOCSCTTY: ::c_int = 0x540E;
pub const TIOCGPGRP: ::c_int = 0x540F;
pub const TIOCSPGRP: ::c_int = 0x5410;
pub const TIOCOUTQ: ::c_int = 0x5411;
pub const TIOCSTI: ::c_int = 0x5412;
pub const TIOCGWINSZ: ::c_int = 0x5413;
pub const TIOCSWINSZ: ::c_int = 0x5414;
pub const TIOCMGET: ::c_int = 0x5415;
pub const TIOCMBIS: ::c_int = 0x5416;
pub const TIOCMBIC: ::c_int = 0x5417;
pub const TIOCMSET: ::c_int = 0x5418;
pub const FIONREAD: ::c_int = 0x541B;
pub const TIOCCONS: ::c_int = 0x541D;

pub const ST_RDONLY: ::c_ulong = 1;
pub const ST_NOSUID: ::c_ulong = 2;
pub const ST_NODEV: ::c_ulong = 4;
pub const ST_NOEXEC: ::c_ulong = 8;
pub const ST_SYNCHRONOUS: ::c_ulong = 16;
pub const ST_MANDLOCK: ::c_ulong = 64;
pub const ST_NOATIME: ::c_ulong = 1024;
pub const ST_NODIRATIME: ::c_ulong = 2048;
pub const ST_RELATIME: ::c_ulong = 4096;

pub const RTLD_NOLOAD: ::c_int = 0x4;

pub const SEM_FAILED: *mut sem_t = 0 as *mut sem_t;

pub const LINUX_REBOOT_MAGIC1: ::c_int = 0xfee1dead;
pub const LINUX_REBOOT_MAGIC2: ::c_int = 672274793;
pub const LINUX_REBOOT_MAGIC2A: ::c_int = 85072278;
pub const LINUX_REBOOT_MAGIC2B: ::c_int = 369367448;
pub const LINUX_REBOOT_MAGIC2C: ::c_int = 537993216;

pub const LINUX_REBOOT_CMD_RESTART: ::c_int = 0x01234567;
pub const LINUX_REBOOT_CMD_HALT: ::c_int = 0xCDEF0123;
pub const LINUX_REBOOT_CMD_CAD_ON: ::c_int = 0x89ABCDEF;
pub const LINUX_REBOOT_CMD_CAD_OFF: ::c_int = 0x00000000;
pub const LINUX_REBOOT_CMD_POWER_OFF: ::c_int = 0x4321FEDC;
pub const LINUX_REBOOT_CMD_RESTART2: ::c_int = 0xA1B2C3D4;
pub const LINUX_REBOOT_CMD_SW_SUSPEND: ::c_int = 0xD000FCE2;
pub const LINUX_REBOOT_CMD_KEXEC: ::c_int = 0x45584543;

pub const MCL_CURRENT: ::c_int = 0x0001;
pub const MCL_FUTURE: ::c_int = 0x0002;

pub const CBAUD: ::tcflag_t = 0o0010017;
pub const TAB1: ::c_int = 0x00000800;
pub const TAB2: ::c_int = 0x00001000;
pub const TAB3: ::c_int = 0x00001800;
pub const CR1: ::c_int  = 0x00000200;
pub const CR2: ::c_int  = 0x00000400;
pub const CR3: ::c_int  = 0x00000600;
pub const FF1: ::c_int  = 0x00008000;
pub const BS1: ::c_int  = 0x00002000;
pub const VT1: ::c_int  = 0x00004000;
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
pub const VSWTC: usize = 7;
pub const OLCUC:  ::tcflag_t = 0o000002;
pub const NLDLY:  ::tcflag_t = 0o000400;
pub const CRDLY:  ::tcflag_t = 0o003000;
pub const TABDLY: ::tcflag_t = 0o014000;
pub const BSDLY:  ::tcflag_t = 0o020000;
pub const FFDLY:  ::tcflag_t = 0o100000;
pub const VTDLY:  ::tcflag_t = 0o040000;
pub const XTABS:  ::tcflag_t = 0o014000;

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

pub const EAI_AGAIN: ::c_int = 2;
pub const EAI_BADFLAGS: ::c_int = 3;
pub const EAI_FAIL: ::c_int = 4;
pub const EAI_FAMILY: ::c_int = 5;
pub const EAI_MEMORY: ::c_int = 6;
pub const EAI_NODATA: ::c_int = 7;
pub const EAI_NONAME: ::c_int = 8;
pub const EAI_SERVICE: ::c_int = 9;
pub const EAI_SOCKTYPE: ::c_int = 10;
pub const EAI_SYSTEM: ::c_int = 11;
pub const EAI_OVERFLOW: ::c_int = 14;

pub const NETLINK_ROUTE: ::c_int = 0;
pub const NETLINK_UNUSED: ::c_int = 1;
pub const NETLINK_USERSOCK: ::c_int = 2;
pub const NETLINK_FIREWALL: ::c_int = 3;
pub const NETLINK_SOCK_DIAG: ::c_int = 4;
pub const NETLINK_NFLOG: ::c_int = 5;
pub const NETLINK_XFRM: ::c_int = 6;
pub const NETLINK_SELINUX: ::c_int = 7;
pub const NETLINK_ISCSI: ::c_int = 8;
pub const NETLINK_AUDIT: ::c_int = 9;
pub const NETLINK_FIB_LOOKUP: ::c_int = 10;
pub const NETLINK_CONNECTOR: ::c_int = 11;
pub const NETLINK_NETFILTER: ::c_int = 12;
pub const NETLINK_IP6_FW: ::c_int = 13;
pub const NETLINK_DNRTMSG: ::c_int = 14;
pub const NETLINK_KOBJECT_UEVENT: ::c_int = 15;
pub const NETLINK_GENERIC: ::c_int = 16;
pub const NETLINK_SCSITRANSPORT: ::c_int = 18;
pub const NETLINK_ECRYPTFS: ::c_int = 19;
pub const NETLINK_RDMA: ::c_int = 20;
pub const NETLINK_CRYPTO: ::c_int = 21;
pub const NETLINK_INET_DIAG: ::c_int = NETLINK_SOCK_DIAG;

pub const MAX_LINKS: ::c_int = 32;

pub const NLM_F_REQUEST: ::c_int = 1;
pub const NLM_F_MULTI: ::c_int = 2;
pub const NLM_F_ACK: ::c_int = 4;
pub const NLM_F_ECHO: ::c_int = 8;
pub const NLM_F_DUMP_INTR: ::c_int = 16;

pub const NLM_F_ROOT: ::c_int = 0x100;
pub const NLM_F_MATCH: ::c_int = 0x200;
pub const NLM_F_ATOMIC: ::c_int = 0x400;
pub const NLM_F_DUMP: ::c_int = NLM_F_ROOT | NLM_F_MATCH;

pub const NLM_F_REPLACE: ::c_int = 0x100;
pub const NLM_F_EXCL: ::c_int = 0x200;
pub const NLM_F_CREATE: ::c_int = 0x400;
pub const NLM_F_APPEND: ::c_int = 0x800;

pub const NLMSG_ALIGNTO: usize = 4;
// FIXME: uncomment when const fn is stable
//pub const NLMSG_HDRLEN: usize = NLMSG_ALIGN(mem::size_of::<nlmsghdr>());
pub const NLMSG_HDRLEN: usize =
    (mem::size_of::<nlmsghdr>() + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1);

pub const NLMSG_NOOP: ::c_int = 0x1;
pub const NLMSG_ERROR: ::c_int = 0x2;
pub const NLMSG_DONE: ::c_int = 0x3;
pub const NLMSG_OVERRUN: ::c_int = 0x4;
pub const NLMSG_MIN_TYPE: ::c_int = 0x10;

pub const GENL_NAMSIZ: ::c_int = 16;

pub const GENL_MIN_ID: ::c_int = NLMSG_MIN_TYPE;
pub const GENL_MAX_ID: ::c_int = 1023;

pub const GENL_ADMIN_PERM: ::c_int = 0x01;
pub const GENL_CMD_CAP_DO: ::c_int = 0x02;
pub const GENL_CMD_CAP_DUMP: ::c_int = 0x04;
pub const GENL_CMD_CAP_HASPOL: ::c_int = 0x08;
pub const GENL_UNS_ADMIN_PERM: ::c_int = 0x10;

pub const GENL_ID_CTRL: ::c_int = NLMSG_MIN_TYPE;
pub const GENL_ID_VFS_DQUOT: ::c_int = NLMSG_MIN_TYPE + 1;
pub const GENL_ID_PMCRAID: ::c_int = NLMSG_MIN_TYPE + 2;

pub const CTRL_CMD_UNSPEC: ::c_int = 0;
pub const CTRL_CMD_NEWFAMILY: ::c_int = 1;
pub const CTRL_CMD_DELFAMILY: ::c_int = 2;
pub const CTRL_CMD_GETFAMILY: ::c_int = 3;
pub const CTRL_CMD_NEWOPS: ::c_int = 4;
pub const CTRL_CMD_DELOPS: ::c_int = 5;
pub const CTRL_CMD_GETOPS: ::c_int = 6;
pub const CTRL_CMD_NEWMCAST_GRP: ::c_int = 7;
pub const CTRL_CMD_DELMCAST_GRP: ::c_int = 8;
pub const CTRL_CMD_GETMCAST_GRP: ::c_int = 9;

pub const CTRL_ATTR_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_FAMILY_ID: ::c_int = 1;
pub const CTRL_ATTR_FAMILY_NAME: ::c_int = 2;
pub const CTRL_ATTR_VERSION: ::c_int = 3;
pub const CTRL_ATTR_HDRSIZE: ::c_int = 4;
pub const CTRL_ATTR_MAXATTR: ::c_int = 5;
pub const CTRL_ATTR_OPS: ::c_int = 6;
pub const CTRL_ATTR_MCAST_GROUPS: ::c_int = 7;

pub const CTRL_ATTR_OP_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_OP_ID: ::c_int = 1;
pub const CTRL_ATTR_OP_FLAGS: ::c_int = 2;

pub const CTRL_ATTR_MCAST_GRP_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_MCAST_GRP_NAME: ::c_int = 1;
pub const CTRL_ATTR_MCAST_GRP_ID: ::c_int = 2;

pub const NETLINK_ADD_MEMBERSHIP: ::c_int = 1;
pub const NETLINK_DROP_MEMBERSHIP: ::c_int = 2;
pub const NETLINK_PKTINFO: ::c_int = 3;
pub const NETLINK_BROADCAST_ERROR: ::c_int = 4;
pub const NETLINK_NO_ENOBUFS: ::c_int = 5;
pub const NETLINK_RX_RING: ::c_int = 6;
pub const NETLINK_TX_RING: ::c_int = 7;

pub const GRND_NONBLOCK: ::c_uint = 0x0001;
pub const GRND_RANDOM: ::c_uint = 0x0002;

pub const SECCOMP_MODE_DISABLED: ::c_uint = 0;
pub const SECCOMP_MODE_STRICT: ::c_uint = 1;
pub const SECCOMP_MODE_FILTER: ::c_uint = 2;

pub const NLA_F_NESTED: ::c_int = 1 << 15;
pub const NLA_F_NET_BYTEORDER: ::c_int = 1 << 14;
pub const NLA_TYPE_MASK: ::c_int = !(NLA_F_NESTED | NLA_F_NET_BYTEORDER);

pub const NLA_ALIGNTO: ::c_int = 4;

pub const SIGEV_THREAD_ID: ::c_int = 4;

pub const CIBAUD: ::tcflag_t = 0o02003600000;
pub const CBAUDEX: ::tcflag_t = 0o010000;

pub const TIOCM_LE: ::c_int = 0x001;
pub const TIOCM_DTR: ::c_int = 0x002;
pub const TIOCM_RTS: ::c_int = 0x004;
pub const TIOCM_ST: ::c_int = 0x008;
pub const TIOCM_SR: ::c_int = 0x010;
pub const TIOCM_CTS: ::c_int = 0x020;
pub const TIOCM_CAR: ::c_int = 0x040;
pub const TIOCM_RNG: ::c_int = 0x080;
pub const TIOCM_DSR: ::c_int = 0x100;
pub const TIOCM_CD: ::c_int = TIOCM_CAR;
pub const TIOCM_RI: ::c_int = TIOCM_RNG;

pub const POLLWRNORM: ::c_short = 0x100;
pub const POLLWRBAND: ::c_short = 0x200;

pub const SFD_CLOEXEC: ::c_int = O_CLOEXEC;
pub const SFD_NONBLOCK: ::c_int = O_NONBLOCK;

pub const SOCK_NONBLOCK: ::c_int = O_NONBLOCK;

pub const SO_ORIGINAL_DST: ::c_int = 80;
pub const IUTF8: ::tcflag_t = 0x00004000;
pub const CMSPAR: ::tcflag_t = 0o10000000000;
pub const O_TMPFILE: ::c_int = 0o20000000 | O_DIRECTORY;

pub const MFD_CLOEXEC: ::c_uint = 0x0001;
pub const MFD_ALLOW_SEALING: ::c_uint = 0x0002;

// linux/netfilter.h
pub const NF_DROP: ::c_int = 0;
pub const NF_ACCEPT: ::c_int =  1;
pub const NF_STOLEN: ::c_int =  2;
pub const NF_QUEUE: ::c_int =  3;
pub const NF_REPEAT: ::c_int =  4;
pub const NF_STOP: ::c_int =  5;
pub const NF_MAX_VERDICT: ::c_int = NF_STOP;

pub const NF_VERDICT_MASK: ::c_int = 0x000000ff;
pub const NF_VERDICT_FLAG_QUEUE_BYPASS: ::c_int = 0x00008000;

pub const NF_VERDICT_QMASK: ::c_int = 0xffff0000;
pub const NF_VERDICT_QBITS: ::c_int = 16;

pub const NF_VERDICT_BITS: ::c_int = 16;

pub const NF_INET_PRE_ROUTING: ::c_int = 0;
pub const NF_INET_LOCAL_IN: ::c_int = 1;
pub const NF_INET_FORWARD: ::c_int = 2;
pub const NF_INET_LOCAL_OUT: ::c_int = 3;
pub const NF_INET_POST_ROUTING: ::c_int = 4;
pub const NF_INET_NUMHOOKS: ::c_int = 5;

pub const NF_NETDEV_INGRESS: ::c_int = 0;
pub const NF_NETDEV_NUMHOOKS: ::c_int = 1;

pub const NFPROTO_UNSPEC: ::c_int = 0;
pub const NFPROTO_INET: ::c_int = 1;
pub const NFPROTO_IPV4: ::c_int = 2;
pub const NFPROTO_ARP: ::c_int = 3;
pub const NFPROTO_NETDEV: ::c_int = 5;
pub const NFPROTO_BRIDGE: ::c_int = 7;
pub const NFPROTO_IPV6: ::c_int = 10;
pub const NFPROTO_DECNET: ::c_int = 12;
pub const NFPROTO_NUMPROTO: ::c_int = 13;

// linux/netfilter_ipv4.h
pub const NF_IP_PRE_ROUTING: ::c_int = 0;
pub const NF_IP_LOCAL_IN: ::c_int = 1;
pub const NF_IP_FORWARD: ::c_int = 2;
pub const NF_IP_LOCAL_OUT: ::c_int = 3;
pub const NF_IP_POST_ROUTING: ::c_int = 4;
pub const NF_IP_NUMHOOKS: ::c_int = 5;

pub const NF_IP_PRI_FIRST: ::c_int = ::INT_MIN;
pub const NF_IP_PRI_CONNTRACK_DEFRAG: ::c_int = -400;
pub const NF_IP_PRI_RAW: ::c_int = -300;
pub const NF_IP_PRI_SELINUX_FIRST: ::c_int = -225;
pub const NF_IP_PRI_CONNTRACK: ::c_int = -200;
pub const NF_IP_PRI_MANGLE: ::c_int = -150;
pub const NF_IP_PRI_NAT_DST: ::c_int = -100;
pub const NF_IP_PRI_FILTER: ::c_int = 0;
pub const NF_IP_PRI_SECURITY: ::c_int = 50;
pub const NF_IP_PRI_NAT_SRC: ::c_int = 100;
pub const NF_IP_PRI_SELINUX_LAST: ::c_int = 225;
pub const NF_IP_PRI_CONNTRACK_HELPER: ::c_int = 300;
pub const NF_IP_PRI_CONNTRACK_CONFIRM: ::c_int = ::INT_MAX;
pub const NF_IP_PRI_LAST: ::c_int = ::INT_MAX;

// linux/netfilter_ipv6.h
pub const NF_IP6_PRE_ROUTING: ::c_int = 0;
pub const NF_IP6_LOCAL_IN: ::c_int = 1;
pub const NF_IP6_FORWARD: ::c_int = 2;
pub const NF_IP6_LOCAL_OUT: ::c_int = 3;
pub const NF_IP6_POST_ROUTING: ::c_int = 4;
pub const NF_IP6_NUMHOOKS: ::c_int = 5;

pub const NF_IP6_PRI_FIRST: ::c_int = ::INT_MIN;
pub const NF_IP6_PRI_CONNTRACK_DEFRAG: ::c_int = -400;
pub const NF_IP6_PRI_RAW: ::c_int = -300;
pub const NF_IP6_PRI_SELINUX_FIRST: ::c_int = -225;
pub const NF_IP6_PRI_CONNTRACK: ::c_int = -200;
pub const NF_IP6_PRI_MANGLE: ::c_int = -150;
pub const NF_IP6_PRI_NAT_DST: ::c_int = -100;
pub const NF_IP6_PRI_FILTER: ::c_int = 0;
pub const NF_IP6_PRI_SECURITY: ::c_int = 50;
pub const NF_IP6_PRI_NAT_SRC: ::c_int = 100;
pub const NF_IP6_PRI_SELINUX_LAST: ::c_int = 225;
pub const NF_IP6_PRI_CONNTRACK_HELPER: ::c_int = 300;
pub const NF_IP6_PRI_LAST: ::c_int = ::INT_MAX;

// linux/netfilter/nf_tables.h
pub const NFT_TABLE_MAXNAMELEN: ::c_int = 32;
pub const NFT_CHAIN_MAXNAMELEN: ::c_int = 32;
pub const NFT_SET_MAXNAMELEN: ::c_int = 32;
pub const NFT_OBJ_MAXNAMELEN: ::c_int = 32;
pub const NFT_USERDATA_MAXLEN: ::c_int = 256;

pub const NFT_REG_VERDICT: ::c_int = 0;
pub const NFT_REG_1: ::c_int = 1;
pub const NFT_REG_2: ::c_int = 2;
pub const NFT_REG_3: ::c_int = 3;
pub const NFT_REG_4: ::c_int = 4;
pub const __NFT_REG_MAX: ::c_int = 5;
pub const NFT_REG32_00: ::c_int = 8;
pub const NFT_REG32_01: ::c_int = 9;
pub const NFT_REG32_02: ::c_int = 10;
pub const NFT_REG32_03: ::c_int = 11;
pub const NFT_REG32_04: ::c_int = 12;
pub const NFT_REG32_05: ::c_int = 13;
pub const NFT_REG32_06: ::c_int = 14;
pub const NFT_REG32_07: ::c_int = 15;
pub const NFT_REG32_08: ::c_int = 16;
pub const NFT_REG32_09: ::c_int = 17;
pub const NFT_REG32_10: ::c_int = 18;
pub const NFT_REG32_11: ::c_int = 19;
pub const NFT_REG32_12: ::c_int = 20;
pub const NFT_REG32_13: ::c_int = 21;
pub const NFT_REG32_14: ::c_int = 22;
pub const NFT_REG32_15: ::c_int = 23;

pub const NFT_REG_SIZE: ::c_int = 16;
pub const NFT_REG32_SIZE: ::c_int = 4;

pub const NFT_CONTINUE: ::c_int = -1;
pub const NFT_BREAK: ::c_int = -2;
pub const NFT_JUMP: ::c_int = -3;
pub const NFT_GOTO: ::c_int = -4;
pub const NFT_RETURN: ::c_int = -5;

pub const NFT_MSG_NEWTABLE: ::c_int = 0;
pub const NFT_MSG_GETTABLE: ::c_int = 1;
pub const NFT_MSG_DELTABLE: ::c_int = 2;
pub const NFT_MSG_NEWCHAIN: ::c_int = 3;
pub const NFT_MSG_GETCHAIN: ::c_int = 4;
pub const NFT_MSG_DELCHAIN: ::c_int = 5;
pub const NFT_MSG_NEWRULE: ::c_int = 6;
pub const NFT_MSG_GETRULE: ::c_int = 7;
pub const NFT_MSG_DELRULE: ::c_int = 8;
pub const NFT_MSG_NEWSET: ::c_int = 9;
pub const NFT_MSG_GETSET: ::c_int = 10;
pub const NFT_MSG_DELSET: ::c_int = 11;
pub const NFT_MSG_NEWSETELEM: ::c_int = 12;
pub const NFT_MSG_GETSETELEM: ::c_int = 13;
pub const NFT_MSG_DELSETELEM: ::c_int = 14;
pub const NFT_MSG_NEWGEN: ::c_int = 15;
pub const NFT_MSG_GETGEN: ::c_int = 16;
pub const NFT_MSG_TRACE: ::c_int = 17;
pub const NFT_MSG_NEWOBJ: ::c_int = 18;
pub const NFT_MSG_GETOBJ: ::c_int = 19;
pub const NFT_MSG_DELOBJ: ::c_int = 20;
pub const NFT_MSG_GETOBJ_RESET: ::c_int = 21;
pub const NFT_MSG_MAX: ::c_int = 22;

pub const NFT_SET_ANONYMOUS: ::c_int = 0x1;
pub const NFT_SET_CONSTANT: ::c_int = 0x2;
pub const NFT_SET_INTERVAL: ::c_int = 0x4;
pub const NFT_SET_MAP: ::c_int = 0x8;
pub const NFT_SET_TIMEOUT: ::c_int = 0x10;
pub const NFT_SET_EVAL: ::c_int = 0x20;

pub const NFT_SET_POL_PERFORMANCE: ::c_int = 0;
pub const NFT_SET_POL_MEMORY: ::c_int = 1;

pub const NFT_SET_ELEM_INTERVAL_END: ::c_int = 0x1;

pub const NFT_DATA_VALUE: ::c_uint = 0;
pub const NFT_DATA_VERDICT: ::c_uint = 0xffffff00;

pub const NFT_DATA_RESERVED_MASK: ::c_uint = 0xffffff00;

pub const NFT_DATA_VALUE_MAXLEN: ::c_int = 64;

pub const NFT_BYTEORDER_NTOH: ::c_int = 0;
pub const NFT_BYTEORDER_HTON: ::c_int = 1;

pub const NFT_CMP_EQ: ::c_int = 0;
pub const NFT_CMP_NEQ: ::c_int = 1;
pub const NFT_CMP_LT: ::c_int = 2;
pub const NFT_CMP_LTE: ::c_int = 3;
pub const NFT_CMP_GT: ::c_int = 4;
pub const NFT_CMP_GTE: ::c_int = 5;

pub const NFT_RANGE_EQ: ::c_int = 0;
pub const NFT_RANGE_NEQ: ::c_int = 1;

pub const NFT_LOOKUP_F_INV: ::c_int = (1 << 0);

pub const NFT_DYNSET_OP_ADD: ::c_int = 0;
pub const NFT_DYNSET_OP_UPDATE: ::c_int = 1;

pub const NFT_DYNSET_F_INV: ::c_int = (1 << 0);

pub const NFT_PAYLOAD_LL_HEADER: ::c_int = 0;
pub const NFT_PAYLOAD_NETWORK_HEADER: ::c_int = 1;
pub const NFT_PAYLOAD_TRANSPORT_HEADER: ::c_int = 2;

pub const NFT_PAYLOAD_CSUM_NONE: ::c_int = 0;
pub const NFT_PAYLOAD_CSUM_INET: ::c_int = 1;

pub const NFT_META_LEN: ::c_int = 0;
pub const NFT_META_PROTOCOL: ::c_int = 1;
pub const NFT_META_PRIORITY: ::c_int = 2;
pub const NFT_META_MARK: ::c_int = 3;
pub const NFT_META_IIF: ::c_int = 4;
pub const NFT_META_OIF: ::c_int = 5;
pub const NFT_META_IIFNAME: ::c_int = 6;
pub const NFT_META_OIFNAME: ::c_int = 7;
pub const NFT_META_IIFTYPE: ::c_int = 8;
pub const NFT_META_OIFTYPE: ::c_int = 9;
pub const NFT_META_SKUID: ::c_int = 10;
pub const NFT_META_SKGID: ::c_int = 11;
pub const NFT_META_NFTRACE: ::c_int = 12;
pub const NFT_META_RTCLASSID: ::c_int = 13;
pub const NFT_META_SECMARK: ::c_int = 14;
pub const NFT_META_NFPROTO: ::c_int = 15;
pub const NFT_META_L4PROTO: ::c_int = 16;
pub const NFT_META_BRI_IIFNAME: ::c_int = 17;
pub const NFT_META_BRI_OIFNAME: ::c_int = 18;
pub const NFT_META_PKTTYPE: ::c_int = 19;
pub const NFT_META_CPU: ::c_int = 20;
pub const NFT_META_IIFGROUP: ::c_int = 21;
pub const NFT_META_OIFGROUP: ::c_int = 22;
pub const NFT_META_CGROUP: ::c_int = 23;
pub const NFT_META_PRANDOM: ::c_int = 24;

pub const NFT_CT_STATE: ::c_int = 0;
pub const NFT_CT_DIRECTION: ::c_int = 1;
pub const NFT_CT_STATUS: ::c_int = 2;
pub const NFT_CT_MARK: ::c_int = 3;
pub const NFT_CT_SECMARK: ::c_int = 4;
pub const NFT_CT_EXPIRATION: ::c_int = 5;
pub const NFT_CT_HELPER: ::c_int = 6;
pub const NFT_CT_L3PROTOCOL: ::c_int = 7;
pub const NFT_CT_SRC: ::c_int = 8;
pub const NFT_CT_DST: ::c_int = 9;
pub const NFT_CT_PROTOCOL: ::c_int = 10;
pub const NFT_CT_PROTO_SRC: ::c_int = 11;
pub const NFT_CT_PROTO_DST: ::c_int = 12;
pub const NFT_CT_LABELS: ::c_int = 13;
pub const NFT_CT_PKTS: ::c_int = 14;
pub const NFT_CT_BYTES: ::c_int = 15;

pub const NFT_LIMIT_PKTS: ::c_int = 0;
pub const NFT_LIMIT_PKT_BYTES: ::c_int = 1;

pub const NFT_LIMIT_F_INV: ::c_int = (1 << 0);

pub const NFT_QUEUE_FLAG_BYPASS: ::c_int = 0x01;
pub const NFT_QUEUE_FLAG_CPU_FANOUT: ::c_int = 0x02;
pub const NFT_QUEUE_FLAG_MASK: ::c_int = 0x03;

pub const NFT_QUOTA_F_INV: ::c_int = (1 << 0);

pub const NFT_REJECT_ICMP_UNREACH: ::c_int = 0;
pub const NFT_REJECT_TCP_RST: ::c_int = 1;
pub const NFT_REJECT_ICMPX_UNREACH: ::c_int = 2;

pub const NFT_REJECT_ICMPX_NO_ROUTE: ::c_int = 0;
pub const NFT_REJECT_ICMPX_PORT_UNREACH: ::c_int = 1;
pub const NFT_REJECT_ICMPX_HOST_UNREACH: ::c_int = 2;
pub const NFT_REJECT_ICMPX_ADMIN_PROHIBITED: ::c_int = 3;

pub const NFT_NAT_SNAT: ::c_int = 0;
pub const NFT_NAT_DNAT: ::c_int = 1;

pub const NFT_TRACETYPE_UNSPEC: ::c_int = 0;
pub const NFT_TRACETYPE_POLICY: ::c_int = 1;
pub const NFT_TRACETYPE_RETURN: ::c_int = 2;
pub const NFT_TRACETYPE_RULE: ::c_int = 3;

pub const NFT_NG_INCREMENTAL: ::c_int = 0;
pub const NFT_NG_RANDOM: ::c_int = 1;

pub const IFF_TUN: ::c_int = 0x0001;
pub const IFF_TAP: ::c_int = 0x0002;
pub const IFF_NO_PI: ::c_int = 0x1000;

// start android/platform/bionic/libc/kernel/uapi/linux/if_ether.h
// from https://android.googlesource.com/
// platform/bionic/+/master/libc/kernel/uapi/linux/if_ether.h
pub const ETH_ALEN: ::c_int = 6;
pub const ETH_HLEN: ::c_int = 14;
pub const ETH_ZLEN: ::c_int = 60;
pub const ETH_DATA_LEN: ::c_int = 1500;
pub const ETH_FRAME_LEN: ::c_int = 1514;
pub const ETH_FCS_LEN: ::c_int = 4;
pub const ETH_MIN_MTU: ::c_int = 68;
pub const ETH_MAX_MTU: ::c_int = 0xFFFF;
pub const ETH_P_LOOP: ::c_int = 0x0060;
pub const ETH_P_PUP: ::c_int = 0x0200;
pub const ETH_P_PUPAT: ::c_int = 0x0201;
pub const ETH_P_TSN: ::c_int = 0x22F0;
pub const ETH_P_IP: ::c_int = 0x0800;
pub const ETH_P_X25: ::c_int = 0x0805;
pub const ETH_P_ARP: ::c_int = 0x0806;
pub const ETH_P_BPQ: ::c_int = 0x08FF;
pub const ETH_P_IEEEPUP: ::c_int = 0x0a00;
pub const ETH_P_IEEEPUPAT: ::c_int = 0x0a01;
pub const ETH_P_BATMAN: ::c_int = 0x4305;
pub const ETH_P_DEC: ::c_int = 0x6000;
pub const ETH_P_DNA_DL: ::c_int = 0x6001;
pub const ETH_P_DNA_RC: ::c_int = 0x6002;
pub const ETH_P_DNA_RT: ::c_int = 0x6003;
pub const ETH_P_LAT: ::c_int = 0x6004;
pub const ETH_P_DIAG: ::c_int = 0x6005;
pub const ETH_P_CUST: ::c_int = 0x6006;
pub const ETH_P_SCA: ::c_int = 0x6007;
pub const ETH_P_TEB: ::c_int = 0x6558;
pub const ETH_P_RARP: ::c_int = 0x8035;
pub const ETH_P_ATALK: ::c_int = 0x809B;
pub const ETH_P_AARP: ::c_int = 0x80F3;
pub const ETH_P_8021Q: ::c_int = 0x8100;
/* see rust-lang/libc#924 pub const ETH_P_ERSPAN: ::c_int = 0x88BE;*/
pub const ETH_P_IPX: ::c_int = 0x8137;
pub const ETH_P_IPV6: ::c_int = 0x86DD;
pub const ETH_P_PAUSE: ::c_int = 0x8808;
pub const ETH_P_SLOW: ::c_int = 0x8809;
pub const ETH_P_WCCP: ::c_int = 0x883E;
pub const ETH_P_MPLS_UC: ::c_int = 0x8847;
pub const ETH_P_MPLS_MC: ::c_int = 0x8848;
pub const ETH_P_ATMMPOA: ::c_int = 0x884c;
pub const ETH_P_PPP_DISC: ::c_int = 0x8863;
pub const ETH_P_PPP_SES: ::c_int = 0x8864;
pub const ETH_P_LINK_CTL: ::c_int = 0x886c;
pub const ETH_P_ATMFATE: ::c_int = 0x8884;
pub const ETH_P_PAE: ::c_int = 0x888E;
pub const ETH_P_AOE: ::c_int = 0x88A2;
pub const ETH_P_8021AD: ::c_int = 0x88A8;
pub const ETH_P_802_EX1: ::c_int = 0x88B5;
pub const ETH_P_TIPC: ::c_int = 0x88CA;
pub const ETH_P_MACSEC: ::c_int = 0x88E5;
pub const ETH_P_8021AH: ::c_int = 0x88E7;
pub const ETH_P_MVRP: ::c_int = 0x88F5;
pub const ETH_P_1588: ::c_int = 0x88F7;
pub const ETH_P_NCSI: ::c_int = 0x88F8;
pub const ETH_P_PRP: ::c_int = 0x88FB;
pub const ETH_P_FCOE: ::c_int = 0x8906;
/* see rust-lang/libc#924 pub const ETH_P_IBOE: ::c_int = 0x8915;*/
pub const ETH_P_TDLS: ::c_int = 0x890D;
pub const ETH_P_FIP: ::c_int = 0x8914;
pub const ETH_P_80221: ::c_int = 0x8917;
pub const ETH_P_HSR: ::c_int = 0x892F;
/* see rust-lang/libc#924 pub const ETH_P_NSH: ::c_int = 0x894F;*/
pub const ETH_P_LOOPBACK: ::c_int = 0x9000;
pub const ETH_P_QINQ1: ::c_int = 0x9100;
pub const ETH_P_QINQ2: ::c_int = 0x9200;
pub const ETH_P_QINQ3: ::c_int = 0x9300;
pub const ETH_P_EDSA: ::c_int = 0xDADA;
/* see rust-lang/libc#924 pub const ETH_P_IFE: ::c_int = 0xED3E;*/
pub const ETH_P_AF_IUCV: ::c_int = 0xFBFB;
pub const ETH_P_802_3_MIN: ::c_int = 0x0600;
pub const ETH_P_802_3: ::c_int = 0x0001;
pub const ETH_P_AX25: ::c_int = 0x0002;
pub const ETH_P_ALL: ::c_int = 0x0003;
pub const ETH_P_802_2: ::c_int = 0x0004;
pub const ETH_P_SNAP: ::c_int = 0x0005;
pub const ETH_P_DDCMP: ::c_int = 0x0006;
pub const ETH_P_WAN_PPP: ::c_int = 0x0007;
pub const ETH_P_PPP_MP: ::c_int = 0x0008;
pub const ETH_P_LOCALTALK: ::c_int = 0x0009;
pub const ETH_P_CAN: ::c_int = 0x000C;
pub const ETH_P_CANFD: ::c_int = 0x000D;
pub const ETH_P_PPPTALK: ::c_int = 0x0010;
pub const ETH_P_TR_802_2: ::c_int = 0x0011;
pub const ETH_P_MOBITEX: ::c_int = 0x0015;
pub const ETH_P_CONTROL: ::c_int = 0x0016;
pub const ETH_P_IRDA: ::c_int = 0x0017;
pub const ETH_P_ECONET: ::c_int = 0x0018;
pub const ETH_P_HDLC: ::c_int = 0x0019;
pub const ETH_P_ARCNET: ::c_int = 0x001A;
pub const ETH_P_DSA: ::c_int = 0x001B;
pub const ETH_P_TRAILER: ::c_int = 0x001C;
pub const ETH_P_PHONET: ::c_int = 0x00F5;
pub const ETH_P_IEEE802154: ::c_int = 0x00F6;
pub const ETH_P_CAIF: ::c_int = 0x00F7;
pub const ETH_P_XDSA: ::c_int = 0x00F8;
/* see rust-lang/libc#924 pub const ETH_P_MAP: ::c_int = 0x00F9;*/
// end android/platform/bionic/libc/kernel/uapi/linux/if_ether.h

pub const SIOCADDRT: ::c_ulong = 0x0000890B;
pub const SIOCDELRT: ::c_ulong = 0x0000890C;
pub const SIOCGIFNAME: ::c_ulong = 0x00008910;
pub const SIOCSIFLINK: ::c_ulong = 0x00008911;
pub const SIOCGIFCONF: ::c_ulong = 0x00008912;
pub const SIOCGIFFLAGS: ::c_ulong = 0x00008913;
pub const SIOCSIFFLAGS: ::c_ulong = 0x00008914;
pub const SIOCGIFADDR: ::c_ulong = 0x00008915;
pub const SIOCSIFADDR: ::c_ulong = 0x00008916;
pub const SIOCGIFDSTADDR: ::c_ulong = 0x00008917;
pub const SIOCSIFDSTADDR: ::c_ulong = 0x00008918;
pub const SIOCGIFBRDADDR: ::c_ulong = 0x00008919;
pub const SIOCSIFBRDADDR: ::c_ulong = 0x0000891A;
pub const SIOCGIFNETMASK: ::c_ulong = 0x0000891B;
pub const SIOCSIFNETMASK: ::c_ulong = 0x0000891C;
pub const SIOCGIFMETRIC: ::c_ulong = 0x0000891D;
pub const SIOCSIFMETRIC: ::c_ulong = 0x0000891E;
pub const SIOCGIFMEM: ::c_ulong = 0x0000891F;
pub const SIOCSIFMEM: ::c_ulong = 0x00008920;
pub const SIOCGIFMTU: ::c_ulong = 0x00008921;
pub const SIOCSIFMTU: ::c_ulong = 0x00008922;
pub const SIOCSIFHWADDR: ::c_ulong = 0x00008924;
pub const SIOCGIFENCAP: ::c_ulong = 0x00008925;
pub const SIOCSIFENCAP: ::c_ulong = 0x00008926;
pub const SIOCGIFHWADDR: ::c_ulong = 0x00008927;
pub const SIOCGIFSLAVE: ::c_ulong = 0x00008929;
pub const SIOCSIFSLAVE: ::c_ulong = 0x00008930;
pub const SIOCADDMULTI: ::c_ulong = 0x00008931;
pub const SIOCDELMULTI: ::c_ulong = 0x00008932;
pub const SIOCDARP: ::c_ulong = 0x00008953;
pub const SIOCGARP: ::c_ulong = 0x00008954;
pub const SIOCSARP: ::c_ulong = 0x00008955;
pub const SIOCDRARP: ::c_ulong = 0x00008960;
pub const SIOCGRARP: ::c_ulong = 0x00008961;
pub const SIOCSRARP: ::c_ulong = 0x00008962;
pub const SIOCGIFMAP: ::c_ulong = 0x00008970;
pub const SIOCSIFMAP: ::c_ulong = 0x00008971;

pub const RTNL_FAMILY_IPMR: u8 = 128;
pub const RTNL_FAMILY_IP6MR: u8 = 129;
pub const RTNL_FAMILY_MAX: u8 = 129;

pub const RTM_BASE: u16 = 16;
pub const RTM_DELLINK: u16 = 17;
pub const RTM_GETLINK: u16 = 18;
pub const RTM_SETLINK: u16 = 19;
pub const RTM_NEWADDR: u16 = 20;
pub const RTM_DELADDR: u16 = 21;
pub const RTM_GETADDR: u16 = 22;
pub const RTM_NEWROUTE: u16 = 24;
pub const RTM_DELROUTE: u16 = 25;
pub const RTM_GETROUTE: u16 = 26;
pub const RTM_NEWNEIGH: u16 = 28;
pub const RTM_DELNEIGH: u16 = 29;
pub const RTM_GETNEIGH: u16 = 30;
pub const RTM_NEWRULE: u16 = 32;
pub const RTM_DELRULE: u16 = 33;
pub const RTM_GETRULE: u16 = 34;
pub const RTM_NEWQDISC: u16 = 36;
pub const RTM_DELQDISC: u16 = 37;
pub const RTM_GETQDISC: u16 = 38;
pub const RTM_NEWTCLASS: u16 = 40;
pub const RTM_DELTCLASS: u16 = 41;
pub const RTM_GETTCLASS: u16 = 42;
pub const RTM_NEWTFILTER: u16 = 44;
pub const RTM_DELTFILTER: u16 = 45;
pub const RTM_GETTFILTER: u16 = 46;
pub const RTM_NEWACTION: u16 = 48;
pub const RTM_DELACTION: u16 = 49;
pub const RTM_GETACTION: u16 = 50;
pub const RTM_NEWPREFIX: u16 = 52;
pub const RTM_GETMULTICAST: u16 = 58;
pub const RTM_GETANYCAST: u16 = 62;
pub const RTM_NEWNEIGHTBL: u16 = 64;
pub const RTM_GETNEIGHTBL: u16 = 66;
pub const RTM_SETNEIGHTBL: u16 = 67;
pub const RTM_NEWNDUSEROPT: u16 = 68;
pub const RTM_NEWADDRLABEL: u16 = 72;
pub const RTM_DELADDRLABEL: u16 = 73;
pub const RTM_GETADDRLABEL: u16 = 74;
pub const RTM_GETDCB: u16 = 78;
pub const RTM_SETDCB: u16 = 79;
pub const RTM_NEWNETCONF: u16 = 80;
pub const RTM_GETNETCONF: u16 = 82;
pub const RTM_NEWMDB: u16 = 84;
pub const RTM_DELMDB: u16 = 85;
pub const RTM_GETMDB: u16 = 86;
pub const RTM_NEWNSID: u16 = 88;
pub const RTM_DELNSID: u16 = 89;
pub const RTM_GETNSID: u16 = 90;
pub const __RTM_MAX: u16 = 91;
pub const RTM_MAX: u16 = (((__RTM_MAX + 3) & !3) - 1);

pub const RTM_NR_MSGTYPES: u16 = RTM_MAX + 1 - RTM_BASE;
pub const RTM_NR_FAMILIES: u16 = RTM_NR_MSGTYPES >> 2;

pub const RTA_ALIGNTO: usize = 4;

pub const RTN_UNSPEC: u8 = 0;
pub const RTN_UNICAST: u8 = 1;
pub const RTN_LOCAL: u8 = 2;
pub const RTN_BROADCAST: u8 = 3;
pub const RTN_ANYCAST: u8 = 4;
pub const RTN_MULTICAST: u8 = 5;
pub const RTN_BLACKHOLE: u8 = 6;
pub const RTN_UNREACHABLE: u8 = 7;
pub const RTN_PROHIBIT: u8 = 8;
pub const RTN_THROW: u8 = 9;
pub const RTN_NAT: u8 = 10;
pub const RTN_XRESOLVE: u8 = 11;
pub const __RTN_MAX: u8 = 12;
pub const RTN_MAX: u8 = __RTN_MAX - 1;

pub const RTPROT_UNSPEC: u8 = 0;
pub const RTPROT_REDIRECT: u8 = 1;
pub const RTPROT_KERNEL: u8 = 2;
pub const RTPROT_BOOT: u8 = 3;
pub const RTPROT_STATIC: u8 = 4;
pub const RTPROT_GATED: u8 = 8;
pub const RTPROT_RA: u8 = 9;
pub const RTPROT_MRT: u8 = 10;
pub const RTPROT_ZEBRA: u8 = 11;
pub const RTPROT_BIRD: u8 = 12;
pub const RTPROT_DNROUTED: u8 = 13;
pub const RTPROT_XORP: u8 = 14;
pub const RTPROT_NTK: u8 = 15;
pub const RTPROT_DHCP: u8 = 16;
pub const RTPROT_MROUTED: u8 = 17;
pub const RTPROT_BABEL: u8 = 42;

pub const RT_SCOPE_UNIVERSE: u8 = 0;
pub const RT_SCOPE_SITE: u8 = 200;
pub const RT_SCOPE_LINK: u8 = 253;
pub const RT_SCOPE_HOST: u8 = 254;
pub const RT_SCOPE_NOWHERE: u8 = 25;

pub const RTM_F_NOTIFY: ::c_uint = 0x100;
pub const RTM_F_CLONED: ::c_uint = 0x200;
pub const RTM_F_EQUALIZE: ::c_uint = 0x400;
pub const RTM_F_PREFIX: ::c_uint = 0x800;
pub const RTM_F_LOOKUP_TABLE: ::c_uint = 0x1000;
pub const RTM_F_FIB_MATCH: ::c_uint = 0x2000;

pub const RT_TABLE_UNSPEC: u32 = 0;
pub const RT_TABLE_COMPAT: u32 = 252;
pub const RT_TABLE_DEFAULT: u32 = 253;
pub const RT_TABLE_MAIN: u32 = 254;
pub const RT_TABLE_LOCAL: u32 = 255;
pub const RT_TABLE_MAX: u32 = 0xFFFFFFF;

pub const RTA_UNSPEC: ::c_ushort = 0;
pub const RTA_DST: ::c_ushort = 1;
pub const RTA_SRC: ::c_ushort = 2;
pub const RTA_IIF: ::c_ushort = 3;
pub const RTA_OIF: ::c_ushort = 4;
pub const RTA_GATEWAY: ::c_ushort = 5;
pub const RTA_PRIORITY: ::c_ushort = 6;
pub const RTA_PREFSRC: ::c_ushort = 7;
pub const RTA_METRICS: ::c_ushort = 8;
pub const RTA_MULTIPATH: ::c_ushort = 9;
pub const RTA_PROTOINFO: ::c_ushort = 10;
pub const RTA_FLOW: ::c_ushort = 11;
pub const RTA_CACHEINFO: ::c_ushort = 12;
pub const RTA_SESSION: ::c_ushort = 13;
pub const RTA_MP_ALGO: ::c_ushort = 14;
pub const RTA_TABLE: ::c_ushort = 15;
pub const RTA_MARK: ::c_ushort = 16;
pub const RTA_MFC_STATS: ::c_ushort = 17;
pub const RTA_VIA: ::c_ushort = 18;
pub const RTA_NEWDST: ::c_ushort = 19;
pub const RTA_PREF: ::c_ushort = 20;
pub const RTA_ENCAP_TYPE: ::c_ushort = 21;
pub const RTA_ENCAP: ::c_ushort = 22;
pub const RTA_EXPIRES: ::c_ushort = 23;
pub const RTA_PAD: ::c_ushort = 24;
pub const RTA_UID: ::c_ushort = 25;
pub const RTA_TTL_PROPAGATE: ::c_ushort = 26;
pub const __RTA_MAX: ::c_ushort = 27;
pub const RTA_MAX: ::c_ushort = __RTA_MAX - 1;

pub const RTNH_F_DEAD: ::c_uchar = 1;
pub const RTNH_F_PERVASIVE: ::c_uchar = 2;
pub const RTNH_F_ONLINK: ::c_uchar = 4;
pub const RTNH_F_OFFLOAD: ::c_uchar = 8;
pub const RTNH_F_LINKDOWN: ::c_uchar = 16;
pub const RTNH_F_UNRESOLVED: ::c_uchar = 32;
pub const RTNH_COMPARE_MASK: ::c_uchar = (RTNH_F_DEAD | RTNH_F_LINKDOWN | RTNH_F_OFFLOAD);
pub const RTNH_ALIGNTO: usize = 4;
pub const RTNETLINK_HAVE_PEERINFO: ::c_int = 1;

pub const RTAX_UNSPEC: ::c_ushort = 0;
pub const RTAX_LOCK: ::c_ushort = 1;
pub const RTAX_MTU: ::c_ushort = 2;
pub const RTAX_WINDOW: ::c_ushort = 3;
pub const RTAX_RTT: ::c_ushort = 4;
pub const RTAX_RTTVAR: ::c_ushort = 5;
pub const RTAX_SSTHRESH: ::c_ushort = 6;
pub const RTAX_CWND: ::c_ushort = 7;
pub const RTAX_ADVMSS: ::c_ushort = 8;
pub const RTAX_REORDERING: ::c_ushort = 9;
pub const RTAX_HOPLIMIT: ::c_ushort = 10;
pub const RTAX_INITCWND: ::c_ushort = 11;
pub const RTAX_FEATURES: ::c_ushort = 12;
pub const RTAX_RTO_MIN: ::c_ushort = 13;
pub const RTAX_INITRWND: ::c_ushort = 14;
pub const RTAX_QUICKACK: ::c_ushort = 15;
pub const RTAX_CC_ALGO: ::c_ushort = 16;
pub const RTAX_FASTOPEN_NO_COOKIE: ::c_ushort = 17;
pub const __RTAX_MAX: ::c_ushort = 18;
pub const RTAX_MAX: ::c_ushort = __RTAX_MAX - 1;

pub const RTAX_FEATURE_ECN: u32 = 1 << 0;
pub const RTAX_FEATURE_SACK: u32 = 1 << 1;
pub const RTAX_FEATURE_TIMESTAMP: u32 = 1 << 2;
pub const RTAX_FEATURE_ALLFRAG: u32 = 1 << 3;
pub const RTAX_FEATURE_MASK: u32 =
    RTAX_FEATURE_ECN | RTAX_FEATURE_SACK | RTAX_FEATURE_TIMESTAMP | RTAX_FEATURE_ALLFRAG;

pub const PREFIX_UNSPEC: ::c_uchar = 0;
pub const PREFIX_ADDRESS: ::c_uchar = 1;
pub const PREFIX_CACHEINFO: ::c_uchar = 2;
pub const __PREFIX_MAX: ::c_uchar = 3;
pub const PREFIX_MAX: ::c_uchar = __PREFIX_MAX - 1;

pub const TCA_UNSPEC: ::c_uchar = 0;
pub const TCA_KIND: ::c_uchar = 1;
pub const TCA_OPTIONS: ::c_uchar = 2;
pub const TCA_STATS: ::c_uchar = 3;
pub const TCA_XSTATS: ::c_uchar = 4;
pub const TCA_RATE: ::c_uchar = 5;
pub const TCA_FCNT: ::c_uchar = 6;
pub const TCA_STATS2: ::c_uchar = 7;
pub const TCA_STAB: ::c_uchar = 8;
pub const TCA_PAD: ::c_uchar = 9;
pub const TCA_DUMP_INVISIBLE: ::c_uchar = 10;
pub const TCA_CHAIN: ::c_uchar = 11;
pub const TCA_HW_OFFLOAD: ::c_uchar = 12;
pub const __TCA_MAX: ::c_uchar = 13;
pub const TCA_MAX: ::c_uchar = __TCA_MAX - 1;

pub const NDUSEROPT_UNSPEC: ::c_uchar = 0;
pub const NDUSEROPT_SRCADDR: ::c_uchar = 1;
pub const __NDUSEROPT_MAX: ::c_uchar = 2;
pub const NDUSEROPT_MAX: ::c_uchar = __NDUSEROPT_MAX - 1;

pub const RTMGRP_LINK: u32 = 1;
pub const RTMGRP_NOTIFY: u32 = 2;
pub const RTMGRP_NEIGH: u32 = 4;
pub const RTMGRP_TC: u32 = 8;
pub const RTMGRP_IPV4_IFADDR: u32 = 0x10;
pub const RTMGRP_IPV4_MROUTE: u32 = 0x20;
pub const RTMGRP_IPV4_ROUTE: u32 = 0x40;
pub const RTMGRP_IPV4_RULE: u32 = 0x80;
pub const RTMGRP_IPV6_IFADDR: u32 = 0x100;
pub const RTMGRP_IPV6_MROUTE: u32 = 0x200;
pub const RTMGRP_IPV6_ROUTE: u32 = 0x400;
pub const RTMGRP_IPV6_IFINFO: u32 = 0x800;
pub const RTMGRP_DECnet_IFADDR: u32 = 0x1000;
pub const RTMGRP_DECnet_ROUTE: u32 = 0x4000;
pub const RTMGRP_IPV6_PREFIX: u32 = 0x20000;

pub const RTNLGRP_NONE: ::c_int = 0;
pub const RTNLGRP_LINK: ::c_int = 1;
pub const RTNLGRP_NOTIFY: ::c_int = 2;
pub const RTNLGRP_NEIGH: ::c_int = 3;
pub const RTNLGRP_TC: ::c_int = 4;
pub const RTNLGRP_IPV4_IFADDR: ::c_int = 5;
pub const RTNLGRP_IPV4_MROUTE: ::c_int = 6;
pub const RTNLGRP_IPV4_ROUTE: ::c_int = 7;
pub const RTNLGRP_IPV4_RULE: ::c_int = 8;
pub const RTNLGRP_IPV6_IFADDR: ::c_int = 9;
pub const RTNLGRP_IPV6_MROUTE: ::c_int = 10;
pub const RTNLGRP_IPV6_ROUTE: ::c_int = 11;
pub const RTNLGRP_IPV6_IFINFO: ::c_int = 12;
pub const RTNLGRP_DECnet_IFADDR: ::c_int = 13;
pub const RTNLGRP_NOP2: ::c_int = 14;
pub const RTNLGRP_DECnet_ROUTE: ::c_int = 15;
pub const RTNLGRP_DECnet_RULE: ::c_int = 16;
pub const RTNLGRP_NOP4: ::c_int = 17;
pub const RTNLGRP_IPV6_PREFIX: ::c_int = 18;
pub const RTNLGRP_IPV6_RULE: ::c_int = 19;
pub const RTNLGRP_ND_USEROPT: ::c_int = 20;
pub const RTNLGRP_PHONET_IFADDR: ::c_int = 21;
pub const RTNLGRP_PHONET_ROUTE: ::c_int = 22;
pub const RTNLGRP_DCB: ::c_int = 23;
pub const RTNLGRP_IPV4_NETCONF: ::c_int = 24;
pub const RTNLGRP_IPV6_NETCONF: ::c_int = 25;
pub const RTNLGRP_MDB: ::c_int = 26;
pub const RTNLGRP_MPLS_ROUTE: ::c_int = 27;
pub const RTNLGRP_NSID: ::c_int = 28;
pub const RTNLGRP_MPLS_NETCONF: ::c_int = 29;
pub const RTNLGRP_IPV4_MROUTE_R: ::c_int = 30;
pub const RTNLGRP_IPV6_MROUTE_R: ::c_int = 31;
pub const __RTNLGRP_MAX: ::c_int = 32;
pub const RTNLGRP_MAX: ::c_int = __RTNLGRP_MAX - 1;

pub const TCA_ROOT_UNSPEC: ::c_int = 0;
pub const TCA_ROOT_TAB: ::c_int = 1;
pub const TCA_ROOT_FLAGS: ::c_int = 2;
pub const TCA_ROOT_COUNT: ::c_int = 3;
pub const TCA_ROOT_TIME_DELTA: ::c_int = 4;
pub const __TCA_ROOT_MAX: ::c_int = 5;
pub const TCA_ROOT_MAX: ::c_int = __TCA_ROOT_MAX - 1;
pub const TCA_ACT_TAB: ::c_int = TCA_ROOT_TAB;
pub const TCAA_MAX: ::c_int = TCA_ROOT_TAB;

pub const TCA_FLAG_LARGE_DUMP_ON: u32 = 1 << 0;

pub const RTEXT_FILTER_VF: u32 = 1 << 0;
pub const RTEXT_FILTER_BRVLAN: u32 = 1 << 1;
pub const RTEXT_FILTER_BRVLAN_COMPRESSED: u32 = 1 << 2;
pub const RTEXT_FILTER_SKIP_STATS: u32 = 1 << 3;

f! {
    pub fn CPU_ZERO(cpuset: &mut cpu_set_t) -> () {
        for slot in cpuset.__bits.iter_mut() {
            *slot = 0;
        }
    }

    pub fn CPU_SET(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in___bits = 8 * mem::size_of_val(&cpuset.__bits[0]);
        let (idx, offset) = (cpu / size_in___bits, cpu % size_in___bits);
        cpuset.__bits[idx] |= 1 << offset;
        ()
    }

    pub fn CPU_CLR(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in___bits = 8 * mem::size_of_val(&cpuset.__bits[0]);
        let (idx, offset) = (cpu / size_in___bits, cpu % size_in___bits);
        cpuset.__bits[idx] &= !(1 << offset);
        ()
    }

    pub fn CPU_ISSET(cpu: usize, cpuset: &cpu_set_t) -> bool {
        let size_in___bits = 8 * mem::size_of_val(&cpuset.__bits[0]);
        let (idx, offset) = (cpu / size_in___bits, cpu % size_in___bits);
        0 != (cpuset.__bits[idx] & (1 << offset))
    }

    pub fn CPU_EQUAL(set1: &cpu_set_t, set2: &cpu_set_t) -> bool {
        set1.__bits == set2.__bits
    }
    pub fn major(dev: ::dev_t) -> ::c_int {
        ((dev >> 8) & 0xfff) as ::c_int
    }
    pub fn minor(dev: ::dev_t) -> ::c_int {
        ((dev & 0xff) | ((dev >> 12) & 0xfff00)) as ::c_int
    }
    pub fn makedev(ma: ::c_int, mi: ::c_int) -> ::dev_t {
        let ma = ma as ::dev_t;
        let mi = mi as ::dev_t;
        ((ma & 0xfff) << 8) | (mi & 0xff) | ((mi & 0xfff00) << 12)
    }

    pub fn NLA_ALIGN(len: ::c_int) -> ::c_int {
        return ((len) + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
    }

    pub fn NLMSG_ALIGN(len: usize) -> usize {
        (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1)
    }

    pub fn NLMSG_LENGTH(len: usize) -> usize {
        len + NLMSG_HDRLEN
    }

    pub fn NLMSG_SPACE(len: usize) -> usize {
        NLMSG_ALIGN(NLMSG_LENGTH(len))
    }

    pub fn NLMSG_DATA(nlh: *const nlmsghdr) -> *const ::c_void {
        let nlh_ptr = nlh as *const u8;
        let nlh_ptr = nlh_ptr.offset(NLMSG_LENGTH(0) as isize);
        nlh_ptr as *const ::c_void
    }

    pub fn NLMSG_NEXT(nlh: &nlmsghdr, len: &mut usize) -> *const nlmsghdr {
        *len -= NLMSG_ALIGN(nlh.nlmsg_len as usize);
        let nlh_ptr = nlh as *const nlmsghdr as *const u8;
        let offset = NLMSG_ALIGN(nlh.nlmsg_len as usize);
        let nlh_ptr = nlh_ptr.offset(offset as isize);
        nlh_ptr as *const nlmsghdr
    }

    pub fn NLMSG_OK(nlh: &nlmsghdr, len: usize) -> bool {
        len >= mem::size_of::<nlmsghdr>() &&
        nlh.nlmsg_len as usize >= mem::size_of::<nlmsghdr>() &&
        nlh.nlmsg_len as usize <= len
    }

    pub fn NLMSG_PAYLOAD(nlh: &nlmsghdr, len: usize) -> usize {
        nlh.nlmsg_len as usize - NLMSG_SPACE(len)
    }

    pub fn RTM_FAM(cmd: u16) -> u16 {
        (cmd - RTM_BASE) >> 2
    }

    pub fn RTA_ALIGN(len: usize) -> usize {
        (len + RTA_ALIGNTO - 1) & !(RTA_ALIGNTO - 1)
    }

    pub fn RTA_OK(rta: &rtattr, len: usize) -> bool {
        len >= mem::size_of::<rtattr>() &&
        (rta.rta_len as usize) >= mem::size_of::<rtattr>() &&
        (rta.rta_len as usize) <= len
    }

    pub fn RTA_NEXT(rta: &rtattr, attrlen: &mut usize) -> *const rtattr {
        *attrlen -= RTA_ALIGN(rta.rta_len as usize);
        let rta_ptr = rta as *const rtattr as *const u8;
        let rta_ptr = rta_ptr.offset(RTA_ALIGN(rta.rta_len as usize) as isize);
        rta_ptr as *const rtattr
    }

    pub fn RTA_LENGTH(len: usize) -> usize {
        RTA_ALIGN(mem::size_of::<rtattr>()) + len
    }

    pub fn RTA_SPACE(len: usize) -> usize {
        RTA_ALIGN(RTA_LENGTH(len))
    }

    pub fn RTA_DATA(rta: *const rtattr) -> *const ::c_void {
        (rta as *const u8).offset(RTA_LENGTH(0) as isize) as *const ::c_void
    }

    pub fn RTA_PAYLOAD(rta: &rtattr) -> usize {
        rta.rta_len as usize - RTA_LENGTH(0)
    }

    pub fn RTM_RTA(r: *const rtmsg) -> *const rtattr {
        let r_ptr = r as *const u8;
        let r_ptr = r_ptr.offset(NLMSG_ALIGN(mem::size_of::<rtmsg>()) as isize);
        r_ptr as *const rtattr
    }

    pub fn RTM_PAYLOAD(n: &nlmsghdr) -> usize {
        NLMSG_PAYLOAD(n, mem::size_of::<rtmsg>())
    }

    pub fn RTNH_ALIGN(len: usize) -> usize {
        (len + RTNH_ALIGNTO - 1) & !(RTNH_ALIGNTO - 1)
    }

    pub fn RTNH_OK(rtnh: &rtnexthop, len: usize) -> bool {
        rtnh.rtnh_len as usize >= mem::size_of::<rtnexthop>() &&
        rtnh.rtnh_len as usize <= len
    }

    pub fn RTNH_NEXT(rtnh: &rtnexthop) -> *const rtnexthop {
        let rtnh_ptr = rtnh as *const rtnexthop as *const u8;
        let rtnh_ptr = rtnh_ptr.offset(RTNH_ALIGN(rtnh.rtnh_len as usize) as isize);
        rtnh_ptr as *const rtnexthop
    }

    pub fn RTNH_LENGTH(len: usize) -> usize {
        RTNH_ALIGN(mem::size_of::<rtnexthop>()) + len
    }

    pub fn RTNH_SPACE(len: usize) -> usize {
        RTNH_ALIGN(RTNH_LENGTH(len))
    }

    pub fn RTNH_DATA(rtnh: *const rtnexthop) -> *const rtattr {
        let rtnh_ptr = rtnh as *const u8;
        let rtnh_ptr = rtnh_ptr.offset(RTNH_LENGTH(0) as isize);
        rtnh_ptr as *const rtattr
    }

    pub fn TCA_RTA(r: *const tcmsg) -> *const rtattr {
        let r_ptr = r as *const u8;
        let r_ptr = r_ptr.offset(NLMSG_ALIGN(mem::size_of::<tcmsg>()) as isize);
        r_ptr as *const rtattr
    }

    pub fn TCA_PAYLOAD(n: &nlmsghdr) -> usize {
        NLMSG_PAYLOAD(n, mem::size_of::<tcmsg>())
    }

    pub fn TA_RTA(r: *const tcamsg) -> *const rtattr {
        let r_ptr = r as *const u8;
        let r_ptr = r_ptr.offset(NLMSG_ALIGN(mem::size_of::<tcamsg>()) as isize);
        r_ptr as *const rtattr
    }

    pub fn TA_PAYLOAD(n: &nlmsghdr) -> usize {
        NLMSG_PAYLOAD(n, mem::size_of::<tcamsg>())
    }
}

extern {
    static mut __progname: *mut ::c_char;
}

extern {
    pub fn madvise(addr: *const ::c_void, len: ::size_t, advice: ::c_int)
                   -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;
    pub fn msync(addr: *const ::c_void, len: ::size_t,
                 flags: ::c_int) -> ::c_int;
    pub fn mprotect(addr: *const ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn recvfrom(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                    flags: ::c_int, addr: *const ::sockaddr,
                    addrlen: *mut ::socklen_t) -> ::ssize_t;
    pub fn getnameinfo(sa: *const ::sockaddr,
                       salen: ::socklen_t,
                       host: *mut ::c_char,
                       hostlen: ::size_t,
                       serv: *mut ::c_char,
                       sevlen: ::size_t,
                       flags: ::c_int) -> ::c_int;
    pub fn ptrace(request: ::c_int, ...) -> ::c_long;
    pub fn getpriority(which: ::c_int, who: ::c_int) -> ::c_int;
    pub fn setpriority(which: ::c_int, who: ::c_int, prio: ::c_int) -> ::c_int;
    pub fn __sched_cpualloc(count: ::size_t) -> *mut ::cpu_set_t;
    pub fn __sched_cpufree(set: *mut ::cpu_set_t);
    pub fn __sched_cpucount(setsize: ::size_t, set: *mut cpu_set_t) -> ::c_int;
    pub fn sched_getcpu() -> ::c_int;

    pub fn utmpname(name: *const ::c_char) -> ::c_int;
    pub fn setutent();
    pub fn getutent() -> *mut utmp;

    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t,
                           len: ::off_t) -> ::c_int;
    pub fn signalfd(fd: ::c_int, mask: *const ::sigset_t, flags: ::c_int)
                    -> ::c_int;
    pub fn syscall(num: ::c_long, ...) -> ::c_long;
    pub fn sched_getaffinity(pid: ::pid_t,
                             cpusetsize: ::size_t,
                             cpuset: *mut cpu_set_t) -> ::c_int;
    pub fn sched_setaffinity(pid: ::pid_t,
                             cpusetsize: ::size_t,
                             cpuset: *const cpu_set_t) -> ::c_int;
    pub fn epoll_create(size: ::c_int) -> ::c_int;
    pub fn epoll_create1(flags: ::c_int) -> ::c_int;
    pub fn epoll_wait(epfd: ::c_int,
                      events: *mut ::epoll_event,
                      maxevents: ::c_int,
                      timeout: ::c_int) -> ::c_int;
    pub fn epoll_ctl(epfd: ::c_int,
                     op: ::c_int,
                     fd: ::c_int,
                     event: *mut ::epoll_event) -> ::c_int;
    pub fn pthread_getschedparam(native: ::pthread_t,
                                 policy: *mut ::c_int,
                                 param: *mut ::sched_param) -> ::c_int;
    pub fn unshare(flags: ::c_int) -> ::c_int;
    pub fn umount(target: *const ::c_char) -> ::c_int;
    pub fn sched_get_priority_max(policy: ::c_int) -> ::c_int;
    pub fn tee(fd_in: ::c_int,
               fd_out: ::c_int,
               len: ::size_t,
               flags: ::c_uint) -> ::ssize_t;
    pub fn settimeofday(tv: *const ::timeval, tz: *const ::timezone) -> ::c_int;
    pub fn splice(fd_in: ::c_int,
                  off_in: *mut ::loff_t,
                  fd_out: ::c_int,
                  off_out: *mut ::loff_t,
                  len: ::size_t,
                  flags: ::c_uint) -> ::ssize_t;
    pub fn eventfd(init: ::c_uint, flags: ::c_int) -> ::c_int;
    pub fn sched_rr_get_interval(pid: ::pid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn sem_timedwait(sem: *mut sem_t,
                         abstime: *const ::timespec) -> ::c_int;
    pub fn sem_getvalue(sem: *mut sem_t,
                        sval: *mut ::c_int) -> ::c_int;
    pub fn sched_setparam(pid: ::pid_t, param: *const ::sched_param) -> ::c_int;
    pub fn setns(fd: ::c_int, nstype: ::c_int) -> ::c_int;
    pub fn swapoff(puath: *const ::c_char) -> ::c_int;
    pub fn vmsplice(fd: ::c_int,
                    iov: *const ::iovec,
                    nr_segs: ::size_t,
                    flags: ::c_uint) -> ::ssize_t;
    pub fn mount(src: *const ::c_char,
                 target: *const ::c_char,
                 fstype: *const ::c_char,
                 flags: ::c_ulong,
                 data: *const ::c_void) -> ::c_int;
    pub fn personality(persona: ::c_ulong) -> ::c_int;
    pub fn prctl(option: ::c_int, ...) -> ::c_int;
    pub fn sched_getparam(pid: ::pid_t, param: *mut ::sched_param) -> ::c_int;
    pub fn ppoll(fds: *mut ::pollfd,
                 nfds: nfds_t,
                 timeout: *const ::timespec,
                 sigmask: *const sigset_t) -> ::c_int;
    pub fn pthread_mutex_timedlock(lock: *mut pthread_mutex_t,
                                   abstime: *const ::timespec) -> ::c_int;
    pub fn clone(cb: extern fn(*mut ::c_void) -> ::c_int,
                 child_stack: *mut ::c_void,
                 flags: ::c_int,
                 arg: *mut ::c_void, ...) -> ::c_int;
    pub fn sched_getscheduler(pid: ::pid_t) -> ::c_int;
    pub fn clock_nanosleep(clk_id: ::clockid_t,
                           flags: ::c_int,
                           rqtp: *const ::timespec,
                           rmtp:  *mut ::timespec) -> ::c_int;
    pub fn pthread_attr_getguardsize(attr: *const ::pthread_attr_t,
                                     guardsize: *mut ::size_t) -> ::c_int;
    pub fn sethostname(name: *const ::c_char, len: ::size_t) -> ::c_int;
    pub fn sched_get_priority_min(policy: ::c_int) -> ::c_int;
    pub fn pthread_condattr_getpshared(attr: *const pthread_condattr_t,
                                       pshared: *mut ::c_int) -> ::c_int;
    pub fn sysinfo(info: *mut ::sysinfo) -> ::c_int;
    pub fn umount2(target: *const ::c_char, flags: ::c_int) -> ::c_int;
    pub fn pthread_setschedparam(native: ::pthread_t,
                                 policy: ::c_int,
                                 param: *const ::sched_param) -> ::c_int;
    pub fn swapon(path: *const ::c_char, swapflags: ::c_int) -> ::c_int;
    pub fn sched_setscheduler(pid: ::pid_t,
                              policy: ::c_int,
                              param: *const ::sched_param) -> ::c_int;
    pub fn sendfile(out_fd: ::c_int,
                    in_fd: ::c_int,
                    offset: *mut off_t,
                    count: ::size_t) -> ::ssize_t;
    pub fn setfsgid(gid: ::gid_t) -> ::c_int;
    pub fn setfsuid(uid: ::uid_t) -> ::c_int;
    pub fn sigsuspend(mask: *const ::sigset_t) -> ::c_int;
    #[cfg_attr(target_os = "solaris", link_name = "__posix_getgrgid_r")]
    pub fn getgrgid_r(uid: ::uid_t,
                      grp: *mut ::group,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut ::group) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "sigaltstack$UNIX2003")]
    #[cfg_attr(target_os = "netbsd", link_name = "__sigaltstack14")]
    pub fn sigaltstack(ss: *const stack_t,
                       oss: *mut stack_t) -> ::c_int;
    pub fn sem_close(sem: *mut sem_t) -> ::c_int;
    pub fn getdtablesize() -> ::c_int;
    #[cfg_attr(target_os = "solaris", link_name = "__posix_getgrnam_r")]
    pub fn getgrnam_r(name: *const ::c_char,
                      grp: *mut ::group,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut ::group) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_sigmask$UNIX2003")]
    pub fn pthread_sigmask(how: ::c_int, set: *const sigset_t,
                           oldset: *mut sigset_t) -> ::c_int;
    pub fn sem_open(name: *const ::c_char, oflag: ::c_int, ...) -> *mut sem_t;
    pub fn getgrnam(name: *const ::c_char) -> *mut ::group;
    pub fn pthread_kill(thread: ::pthread_t, sig: ::c_int) -> ::c_int;
    pub fn sem_unlink(name: *const ::c_char) -> ::c_int;
    pub fn daemon(nochdir: ::c_int, noclose: ::c_int) -> ::c_int;
    #[cfg_attr(target_os = "netbsd", link_name = "__getpwnam_r50")]
    #[cfg_attr(target_os = "solaris", link_name = "__posix_getpwnam_r")]
    pub fn getpwnam_r(name: *const ::c_char,
                      pwd: *mut passwd,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut passwd) -> ::c_int;
    #[cfg_attr(target_os = "netbsd", link_name = "__getpwuid_r50")]
    #[cfg_attr(target_os = "solaris", link_name = "__posix_getpwuid_r")]
    pub fn getpwuid_r(uid: ::uid_t,
                      pwd: *mut passwd,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut passwd) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch ="x86"),
               link_name = "sigwait$UNIX2003")]
    #[cfg_attr(target_os = "solaris", link_name = "__posix_sigwait")]
    pub fn sigwait(set: *const sigset_t,
                   sig: *mut ::c_int) -> ::c_int;
    pub fn pthread_atfork(prepare: Option<unsafe extern fn()>,
                          parent: Option<unsafe extern fn()>,
                          child: Option<unsafe extern fn()>) -> ::c_int;
    pub fn getgrgid(gid: ::gid_t) -> *mut ::group;
    pub fn getgrouplist(user: *const ::c_char,
                        group: ::gid_t,
                        groups: *mut ::gid_t,
                        ngroups: *mut ::c_int) -> ::c_int;
    pub fn initgroups(user: *const ::c_char, group: ::gid_t) -> ::c_int;
    pub fn pthread_mutexattr_getpshared(attr: *const pthread_mutexattr_t,
                                        pshared: *mut ::c_int) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "popen$UNIX2003")]
    pub fn popen(command: *const c_char,
                 mode: *const c_char) -> *mut ::FILE;
    pub fn faccessat(dirfd: ::c_int, pathname: *const ::c_char,
                     mode: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn pthread_create(native: *mut ::pthread_t,
                          attr: *const ::pthread_attr_t,
                          f: extern fn(*mut ::c_void) -> *mut ::c_void,
                          value: *mut ::c_void) -> ::c_int;
    pub fn __errno() -> *mut ::c_int;
}

cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        mod b32;
        pub use self::b32::*;
    } else if #[cfg(target_pointer_width = "64")] {
        mod b64;
        pub use self::b64::*;
    } else {
        // Unknown target_pointer_width
    }
}
