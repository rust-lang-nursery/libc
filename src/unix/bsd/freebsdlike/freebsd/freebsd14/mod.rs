// APIs in FreeBSD 13 that have changed since 11.

pub type kpaddr_t = u64;
pub type kssize_t = i64;
pub type domainset_t = __c_anonymous_domainset;

s! {
    pub struct kvm_page {
        pub kp_version: ::u_int,
        pub kp_paddr: ::kpaddr_t,
        pub kp_kmap_vaddr: ::kvaddr_t,
        pub kp_dmap_vaddr: ::kvaddr_t,
        pub kp_prot: ::vm_prot_t,
        pub kp_offset: ::off_t,
        pub kp_len: ::size_t,
    }

    pub struct __c_anonymous_domainset {
        _priv: [::uintptr_t; 4],
    }

    pub struct kinfo_proc {
        /// Size of this structure.
        pub ki_structsize: ::c_int,
        /// Reserved: layout identifier.
        pub ki_layout: ::c_int,
        /// Address of command arguments.
        pub ki_args: *mut ::pargs,
        // This is normally "struct proc".
        /// Address of proc.
        pub ki_paddr: *mut ::c_void,
        // This is normally "struct user".
        /// Kernel virtual address of u-area.
        pub ki_addr: *mut ::c_void,
        // This is normally "struct vnode".
        /// Pointer to trace file.
        pub ki_tracep: *mut ::c_void,
        // This is normally "struct vnode".
        /// Pointer to executable file.
        pub ki_textvp: *mut ::c_void,
        // This is normally "struct filedesc".
        /// Pointer to open file info.
        pub ki_fd: *mut ::c_void,
        // This is normally "struct vmspace".
        /// Pointer to kernel vmspace struct.
        pub ki_vmspace: *mut ::c_void,
        /// Sleep address.
        pub ki_wchan: *const ::c_void,
        /// Process identifier.
        pub ki_pid: ::pid_t,
        /// Parent process ID.
        pub ki_ppid: ::pid_t,
        /// Process group ID.
        pub ki_pgid: ::pid_t,
        /// tty process group ID.
        pub ki_tpgid: ::pid_t,
        /// Process session ID.
        pub ki_sid: ::pid_t,
        /// Terminal session ID.
        pub ki_tsid: ::pid_t,
        /// Job control counter.
        pub ki_jobc: ::c_short,
        /// Unused (just here for alignment).
        pub ki_spare_short1: ::c_short,
        /// Controlling tty dev.
        pub ki_tdev_freebsd11: u32,
        /// Signals arrived but not delivered.
        pub ki_siglist: ::sigset_t,
        /// Current signal mask.
        pub ki_sigmask: ::sigset_t,
        /// Signals being ignored.
        pub ki_sigignore: ::sigset_t,
        /// Signals being caught by user.
        pub ki_sigcatch: ::sigset_t,
        /// Effective user ID.
        pub ki_uid: ::uid_t,
        /// Real user ID.
        pub ki_ruid: ::uid_t,
        /// Saved effective user ID.
        pub ki_svuid: ::uid_t,
        /// Real group ID.
        pub ki_rgid: ::gid_t,
        /// Saved effective group ID.
        pub ki_svgid: ::gid_t,
        /// Number of groups.
        pub ki_ngroups: ::c_short,
        /// Unused (just here for alignment).
        pub ki_spare_short2: ::c_short,
        /// Groups.
        pub ki_groups: [::gid_t; ::KI_NGROUPS],
        /// Virtual size.
        pub ki_size: ::vm_size_t,
        /// Current resident set size in pages.
        pub ki_rssize: ::segsz_t,
        /// Resident set size before last swap.
        pub ki_swrss: ::segsz_t,
        /// Text size (pages) XXX.
        pub ki_tsize: ::segsz_t,
        /// Data size (pages) XXX.
        pub ki_dsize: ::segsz_t,
        /// Stack size (pages).
        pub ki_ssize: ::segsz_t,
        /// Exit status for wait & stop signal.
        pub ki_xstat: ::u_short,
        /// Accounting flags.
        pub ki_acflag: ::u_short,
        /// %cpu for process during `ki_swtime`.
        pub ki_pctcpu: ::fixpt_t,
        /// Time averaged value of `ki_cpticks`.
        pub ki_estcpu: ::u_int,
        /// Time since last blocked.
        pub ki_slptime: ::u_int,
        /// Time swapped in or out.
        pub ki_swtime: ::u_int,
        /// Number of copy-on-write faults.
        pub ki_cow: ::u_int,
        /// Real time in microsec.
        pub ki_runtime: u64,
        /// Starting time.
        pub ki_start: ::timeval,
        /// Time used by process children.
        pub ki_childtime: ::timeval,
        /// P_* flags.
        pub ki_flag: ::c_long,
        /// KI_* flags (below).
        pub ki_kiflag: ::c_long,
        /// Kernel trace points.
        pub ki_traceflag: ::c_int,
        /// S* process status.
        pub ki_stat: ::c_char,
        /// Process "nice" value.
        pub ki_nice: i8, // signed char
        /// Process lock (prevent swap) count.
        pub ki_lock: ::c_char,
        /// Run queue index.
        pub ki_rqindex: ::c_char,
        /// Which cpu we are on.
        pub ki_oncpu_old: ::c_uchar,
        /// Last cpu we were on.
        pub ki_lastcpu_old: ::c_uchar,
        /// Thread name.
        pub ki_tdname: [::c_char; ::TDNAMLEN + 1],
        /// Wchan message.
        pub ki_wmesg: [::c_char; ::WMESGLEN + 1],
        /// Setlogin name.
        pub ki_login: [::c_char; ::LOGNAMELEN + 1],
        /// Lock name.
        pub ki_lockname: [::c_char; ::LOCKNAMELEN + 1],
        /// Command name.
        pub ki_comm: [::c_char; ::COMMLEN + 1],
        /// Emulation name.
        pub ki_emul: [::c_char; ::KI_EMULNAMELEN + 1],
        /// Login class.
        pub ki_loginclass: [::c_char; ::LOGINCLASSLEN + 1],
        /// More thread name.
        pub ki_moretdname: [::c_char; ::MAXCOMLEN - ::TDNAMLEN + 1],
        /// Spare string space.
        pub ki_sparestrings: [[::c_char; 23]; 2], // little hack to allow PartialEq
        /// Spare room for growth.
        pub ki_spareints: [::c_int; ::KI_NSPARE_INT],
        /// Controlling tty dev.
        pub ki_tdev: u64,
        /// Which cpu we are on.
        pub ki_oncpu: ::c_int,
        /// Last cpu we were on.
        pub ki_lastcpu: ::c_int,
        /// PID of tracing process.
        pub ki_tracer: ::c_int,
        /// P2_* flags.
        pub ki_flag2: ::c_int,
        /// Default FIB number.
        pub ki_fibnum: ::c_int,
        /// Credential flags.
        pub ki_cr_flags: ::u_int,
        /// Process jail ID.
        pub ki_jid: ::c_int,
        /// Number of threads in total.
        pub ki_numthreads: ::c_int,
        /// Thread ID.
        pub ki_tid: ::lwpid_t,
        /// Process priority.
        pub ki_pri: ::priority,
        /// Process rusage statistics.
        pub ki_rusage: ::rusage,
        /// rusage of children processes.
        pub ki_rusage_ch: ::rusage,
        // This is normally "struct pcb".
        /// Kernel virtual addr of pcb.
        pub ki_pcb: *mut ::c_void,
        /// Kernel virtual addr of stack.
        pub ki_kstack: *mut ::c_void,
        /// User convenience pointer.
        pub ki_udata: *mut ::c_void,
        // This is normally "struct thread".
        pub ki_tdaddr: *mut ::c_void,
        // This is normally "struct pwddesc".
        /// Pointer to process paths info.
        pub ki_pd: *mut ::c_void,
        pub ki_spareptrs: [*mut ::c_void; ::KI_NSPARE_PTR],
        pub ki_sparelongs: [::c_long; ::KI_NSPARE_LONG],
        /// PS_* flags.
        pub ki_sflag: ::c_long,
        /// kthread flag.
        pub ki_tdflags: ::c_long,
    }
}

s_no_extra_traits! {
    pub struct vnstat {
        pub vn_fileid: u64,
        pub vn_size: u64,
        pub vn_dev: u64,
        pub vn_fsid: u64,
        pub vn_mntdir: *mut ::c_char,
        pub vn_type: ::c_int,
        pub vn_mode: u16,
        pub vn_devname: [::c_char; ::SPECNAMELEN as usize + 1],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for vnstat {
            fn eq(&self, other: &vnstat) -> bool {
                let self_vn_devname: &[::c_char] = &self.vn_devname;
                let other_vn_devname: &[::c_char] = &other.vn_devname;

                self.vn_fileid == other.vn_fileid &&
                self.vn_size == other.vn_size &&
                self.vn_dev == other.vn_dev &&
                self.vn_fsid == other.vn_fsid &&
                self.vn_mntdir == other.vn_mntdir &&
                self.vn_type == other.vn_type &&
                self.vn_mode == other.vn_mode &&
                self_vn_devname == other_vn_devname
            }
        }
        impl Eq for vnstat {}
        impl ::fmt::Debug for vnstat {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                let self_vn_devname: &[::c_char] = &self.vn_devname;

                f.debug_struct("vnstat")
                    .field("vn_fileid", &self.vn_fileid)
                    .field("vn_size", &self.vn_size)
                    .field("vn_dev", &self.vn_dev)
                    .field("vn_fsid", &self.vn_fsid)
                    .field("vn_mntdir", &self.vn_mntdir)
                    .field("vn_type", &self.vn_type)
                    .field("vn_mode", &self.vn_mode)
                    .field("vn_devname", &self_vn_devname)
                    .finish()
            }
        }
        impl ::hash::Hash for vnstat {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                let self_vn_devname: &[::c_char] = &self.vn_devname;

                self.vn_fileid.hash(state);
                self.vn_size.hash(state);
                self.vn_dev.hash(state);
                self.vn_fsid.hash(state);
                self.vn_mntdir.hash(state);
                self.vn_type.hash(state);
                self.vn_mode.hash(state);
                self_vn_devname.hash(state);
            }
        }
    }
}

pub const RAND_MAX: ::c_int = 0x7fff_ffff;
pub const ELAST: ::c_int = 97;

pub const KF_TYPE_EVENTFD: ::c_int = 13;

/// max length of devicename
pub const SPECNAMELEN: ::c_int = 255;
pub const KI_NSPARE_PTR: usize = 5;

/// domainset policies
pub const DOMAINSET_POLICY_INVALID: ::c_int = 0;
pub const DOMAINSET_POLICY_ROUNDROBIN: ::c_int = 1;
pub const DOMAINSET_POLICY_FIRSTTOUCH: ::c_int = 2;
pub const DOMAINSET_POLICY_PREFER: ::c_int = 3;
pub const DOMAINSET_POLICY_INTERLEAVE: ::c_int = 4;

pub const MINCORE_SUPER: ::c_int = 0x60;

safe_f! {
    pub {const} fn makedev(major: ::c_uint, minor: ::c_uint) -> ::dev_t {
        let major = major as ::dev_t;
        let minor = minor as ::dev_t;
        let mut dev = 0;
        dev |= ((major & 0xffffff00) as ::dev_t) << 32;
        dev |= ((major & 0x000000ff) as ::dev_t) << 8;
        dev |= ((minor & 0x0000ff00) as ::dev_t) << 24;
        dev |= ((minor & 0xffff00ff) as ::dev_t) << 0;
        dev
    }
}

f! {
    pub fn major(dev: ::dev_t) -> ::c_int {
        (((dev >> 32) & 0xffffff00) | ((dev >> 8) & 0xff)) as ::c_int
    }

    pub fn minor(dev: ::dev_t) -> ::c_int {
        (((dev >> 24) & 0xff00) | (dev & 0xffff00ff)) as ::c_int
    }
}

extern "C" {
    pub fn setgrent();
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int;
    pub fn freelocale(loc: ::locale_t);
    pub fn msgrcv(
        msqid: ::c_int,
        msgp: *mut ::c_void,
        msgsz: ::size_t,
        msgtyp: ::c_long,
        msgflg: ::c_int,
    ) -> ::ssize_t;

    pub fn cpuset_getdomain(
        level: ::cpulevel_t,
        which: ::cpuwhich_t,
        id: ::id_t,
        setsize: ::size_t,
        mask: *mut ::domainset_t,
        policy: *mut ::c_int,
    ) -> ::c_int;
    pub fn cpuset_setdomain(
        level: ::cpulevel_t,
        which: ::cpuwhich_t,
        id: ::id_t,
        setsize: ::size_t,
        mask: *const ::domainset_t,
        policy: ::c_int,
    ) -> ::c_int;

    pub fn dirname(path: *mut ::c_char) -> *mut ::c_char;
    pub fn basename(path: *mut ::c_char) -> *mut ::c_char;
}

#[link(name = "kvm")]
extern "C" {
    pub fn kvm_kerndisp(kd: *mut ::kvm_t) -> ::kssize_t;
}

cfg_if! {
    if #[cfg(any(target_arch = "x86_64",
                 target_arch = "aarch64",
                 target_arch = "riscv64"))] {
        mod b64;
        pub use self::b64::*;
    }
}

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    }
}
