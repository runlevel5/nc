// Code generated by mksysnum_freebsd.py; DO NOT EDIT.

use crate::syscalls::Sysno;

pub const SYS_SYSCALL: Sysno = 0;
pub const SYS_EXIT: Sysno = 1;
pub const SYS_FORK: Sysno = 2;
pub const SYS_READ: Sysno = 3;
pub const SYS_WRITE: Sysno = 4;
pub const SYS_OPEN: Sysno = 5;
pub const SYS_CLOSE: Sysno = 6;
pub const SYS_WAIT4: Sysno = 7;
pub const SYS_LINK: Sysno = 9;
pub const SYS_UNLINK: Sysno = 10;
pub const SYS_CHDIR: Sysno = 12;
pub const SYS_FCHDIR: Sysno = 13;
pub const SYS_FREEBSD11_MKNOD: Sysno = 14;
pub const SYS_CHMOD: Sysno = 15;
pub const SYS_CHOWN: Sysno = 16;
pub const SYS_BREAK: Sysno = 17;
pub const SYS_GETPID: Sysno = 20;
pub const SYS_MOUNT: Sysno = 21;
pub const SYS_UNMOUNT: Sysno = 22;
pub const SYS_SETUID: Sysno = 23;
pub const SYS_GETUID: Sysno = 24;
pub const SYS_GETEUID: Sysno = 25;
pub const SYS_PTRACE: Sysno = 26;
pub const SYS_RECVMSG: Sysno = 27;
pub const SYS_SENDMSG: Sysno = 28;
pub const SYS_RECVFROM: Sysno = 29;
pub const SYS_ACCEPT: Sysno = 30;
pub const SYS_GETPEERNAME: Sysno = 31;
pub const SYS_GETSOCKNAME: Sysno = 32;
pub const SYS_ACCESS: Sysno = 33;
pub const SYS_CHFLAGS: Sysno = 34;
pub const SYS_FCHFLAGS: Sysno = 35;
pub const SYS_SYNC: Sysno = 36;
pub const SYS_KILL: Sysno = 37;
pub const SYS_GETPPID: Sysno = 39;
pub const SYS_DUP: Sysno = 41;
pub const SYS_FREEBSD10_PIPE: Sysno = 42;
pub const SYS_GETEGID: Sysno = 43;
pub const SYS_PROFIL: Sysno = 44;
pub const SYS_KTRACE: Sysno = 45;
pub const SYS_GETGID: Sysno = 47;
pub const SYS_GETLOGIN: Sysno = 49;
pub const SYS_SETLOGIN: Sysno = 50;
pub const SYS_ACCT: Sysno = 51;
pub const SYS_SIGALTSTACK: Sysno = 53;
pub const SYS_IOCTL: Sysno = 54;
pub const SYS_REBOOT: Sysno = 55;
pub const SYS_REVOKE: Sysno = 56;
pub const SYS_SYMLINK: Sysno = 57;
pub const SYS_READLINK: Sysno = 58;
pub const SYS_EXECVE: Sysno = 59;
pub const SYS_UMASK: Sysno = 60;
pub const SYS_CHROOT: Sysno = 61;
pub const SYS_MSYNC: Sysno = 65;
pub const SYS_VFORK: Sysno = 66;
pub const SYS_SBRK: Sysno = 69;
pub const SYS_SSTK: Sysno = 70;
pub const SYS_FREEBSD11_VADVISE: Sysno = 72;
pub const SYS_MUNMAP: Sysno = 73;
pub const SYS_MPROTECT: Sysno = 74;
pub const SYS_MADVISE: Sysno = 75;
pub const SYS_MINCORE: Sysno = 78;
pub const SYS_GETGROUPS: Sysno = 79;
pub const SYS_SETGROUPS: Sysno = 80;
pub const SYS_GETPGRP: Sysno = 81;
pub const SYS_SETPGID: Sysno = 82;
pub const SYS_SETITIMER: Sysno = 83;
pub const SYS_SWAPON: Sysno = 85;
pub const SYS_GETITIMER: Sysno = 86;
pub const SYS_GETDTABLESIZE: Sysno = 89;
pub const SYS_DUP2: Sysno = 90;
pub const SYS_FCNTL: Sysno = 92;
pub const SYS_SELECT: Sysno = 93;
pub const SYS_FSYNC: Sysno = 95;
pub const SYS_SETPRIORITY: Sysno = 96;
pub const SYS_SOCKET: Sysno = 97;
pub const SYS_CONNECT: Sysno = 98;
pub const SYS_GETPRIORITY: Sysno = 100;
pub const SYS_BIND: Sysno = 104;
pub const SYS_SETSOCKOPT: Sysno = 105;
pub const SYS_LISTEN: Sysno = 106;
pub const SYS_GETTIMEOFDAY: Sysno = 116;
pub const SYS_GETRUSAGE: Sysno = 117;
pub const SYS_GETSOCKOPT: Sysno = 118;
pub const SYS_READV: Sysno = 120;
pub const SYS_WRITEV: Sysno = 121;
pub const SYS_SETTIMEOFDAY: Sysno = 122;
pub const SYS_FCHOWN: Sysno = 123;
pub const SYS_FCHMOD: Sysno = 124;
pub const SYS_SETREUID: Sysno = 126;
pub const SYS_SETREGID: Sysno = 127;
pub const SYS_RENAME: Sysno = 128;
pub const SYS_FLOCK: Sysno = 131;
pub const SYS_MKFIFO: Sysno = 132;
pub const SYS_SENDTO: Sysno = 133;
pub const SYS_SHUTDOWN: Sysno = 134;
pub const SYS_SOCKETPAIR: Sysno = 135;
pub const SYS_MKDIR: Sysno = 136;
pub const SYS_RMDIR: Sysno = 137;
pub const SYS_UTIMES: Sysno = 138;
pub const SYS_ADJTIME: Sysno = 140;
pub const SYS_SETSID: Sysno = 147;
pub const SYS_QUOTACTL: Sysno = 148;
pub const SYS_NLM_SYSCALL: Sysno = 154;
pub const SYS_NFSSVC: Sysno = 155;
pub const SYS_LGETFH: Sysno = 160;
pub const SYS_GETFH: Sysno = 161;
pub const SYS_SYSARCH: Sysno = 165;
pub const SYS_RTPRIO: Sysno = 166;
pub const SYS_SEMSYS: Sysno = 169;
pub const SYS_MSGSYS: Sysno = 170;
pub const SYS_SHMSYS: Sysno = 171;
pub const SYS_SETFIB: Sysno = 175;
pub const SYS_NTP_ADJTIME: Sysno = 176;
pub const SYS_SETGID: Sysno = 181;
pub const SYS_SETEGID: Sysno = 182;
pub const SYS_SETEUID: Sysno = 183;
pub const SYS_FREEBSD11_STAT: Sysno = 188;
pub const SYS_FREEBSD11_FSTAT: Sysno = 189;
pub const SYS_FREEBSD11_LSTAT: Sysno = 190;
pub const SYS_PATHCONF: Sysno = 191;
pub const SYS_FPATHCONF: Sysno = 192;
pub const SYS_GETRLIMIT: Sysno = 194;
pub const SYS_SETRLIMIT: Sysno = 195;
pub const SYS_FREEBSD11_GETDIRENTRIES: Sysno = 196;
pub const SYS___SYSCALL: Sysno = 198;
pub const SYS___SYSCTL: Sysno = 202;
pub const SYS_MLOCK: Sysno = 203;
pub const SYS_MUNLOCK: Sysno = 204;
pub const SYS_UNDELETE: Sysno = 205;
pub const SYS_FUTIMES: Sysno = 206;
pub const SYS_GETPGID: Sysno = 207;
pub const SYS_POLL: Sysno = 209;
pub const SYS_FREEBSD7___SEMCTL: Sysno = 220;
pub const SYS_SEMGET: Sysno = 221;
pub const SYS_SEMOP: Sysno = 222;
pub const SYS_FREEBSD7_MSGCTL: Sysno = 224;
pub const SYS_MSGGET: Sysno = 225;
pub const SYS_MSGSND: Sysno = 226;
pub const SYS_MSGRCV: Sysno = 227;
pub const SYS_SHMAT: Sysno = 228;
pub const SYS_FREEBSD7_SHMCTL: Sysno = 229;
pub const SYS_SHMDT: Sysno = 230;
pub const SYS_SHMGET: Sysno = 231;
pub const SYS_CLOCK_GETTIME: Sysno = 232;
pub const SYS_CLOCK_SETTIME: Sysno = 233;
pub const SYS_CLOCK_GETRES: Sysno = 234;
pub const SYS_KTIMER_CREATE: Sysno = 235;
pub const SYS_KTIMER_DELETE: Sysno = 236;
pub const SYS_KTIMER_SETTIME: Sysno = 237;
pub const SYS_KTIMER_GETTIME: Sysno = 238;
pub const SYS_KTIMER_GETOVERRUN: Sysno = 239;
pub const SYS_NANOSLEEP: Sysno = 240;
pub const SYS_FFCLOCK_GETCOUNTER: Sysno = 241;
pub const SYS_FFCLOCK_SETESTIMATE: Sysno = 242;
pub const SYS_FFCLOCK_GETESTIMATE: Sysno = 243;
pub const SYS_CLOCK_NANOSLEEP: Sysno = 244;
pub const SYS_CLOCK_GETCPUCLOCKID2: Sysno = 247;
pub const SYS_NTP_GETTIME: Sysno = 248;
pub const SYS_MINHERIT: Sysno = 250;
pub const SYS_RFORK: Sysno = 251;
pub const SYS_ISSETUGID: Sysno = 253;
pub const SYS_LCHOWN: Sysno = 254;
pub const SYS_AIO_READ: Sysno = 255;
pub const SYS_AIO_WRITE: Sysno = 256;
pub const SYS_LIO_LISTIO: Sysno = 257;
pub const SYS_FREEBSD11_GETDENTS: Sysno = 272;
pub const SYS_LCHMOD: Sysno = 274;
pub const SYS_LUTIMES: Sysno = 276;
pub const SYS_FREEBSD11_NSTAT: Sysno = 278;
pub const SYS_FREEBSD11_NFSTAT: Sysno = 279;
pub const SYS_FREEBSD11_NLSTAT: Sysno = 280;
pub const SYS_PREADV: Sysno = 289;
pub const SYS_PWRITEV: Sysno = 290;
pub const SYS_FHOPEN: Sysno = 298;
pub const SYS_FREEBSD11_FHSTAT: Sysno = 299;
pub const SYS_MODNEXT: Sysno = 300;
pub const SYS_MODSTAT: Sysno = 301;
pub const SYS_MODFNEXT: Sysno = 302;
pub const SYS_MODFIND: Sysno = 303;
pub const SYS_KLDLOAD: Sysno = 304;
pub const SYS_KLDUNLOAD: Sysno = 305;
pub const SYS_KLDFIND: Sysno = 306;
pub const SYS_KLDNEXT: Sysno = 307;
pub const SYS_KLDSTAT: Sysno = 308;
pub const SYS_KLDFIRSTMOD: Sysno = 309;
pub const SYS_GETSID: Sysno = 310;
pub const SYS_SETRESUID: Sysno = 311;
pub const SYS_SETRESGID: Sysno = 312;
pub const SYS_AIO_RETURN: Sysno = 314;
pub const SYS_AIO_SUSPEND: Sysno = 315;
pub const SYS_AIO_CANCEL: Sysno = 316;
pub const SYS_AIO_ERROR: Sysno = 317;
pub const SYS_YIELD: Sysno = 321;
pub const SYS_MLOCKALL: Sysno = 324;
pub const SYS_MUNLOCKALL: Sysno = 325;
pub const SYS___GETCWD: Sysno = 326;
pub const SYS_SCHED_SETPARAM: Sysno = 327;
pub const SYS_SCHED_GETPARAM: Sysno = 328;
pub const SYS_SCHED_SETSCHEDULER: Sysno = 329;
pub const SYS_SCHED_GETSCHEDULER: Sysno = 330;
pub const SYS_SCHED_YIELD: Sysno = 331;
pub const SYS_SCHED_GET_PRIORITY_MAX: Sysno = 332;
pub const SYS_SCHED_GET_PRIORITY_MIN: Sysno = 333;
pub const SYS_SCHED_RR_GET_INTERVAL: Sysno = 334;
pub const SYS_UTRACE: Sysno = 335;
pub const SYS_KLDSYM: Sysno = 337;
pub const SYS_JAIL: Sysno = 338;
pub const SYS_NNPFS_SYSCALL: Sysno = 339;
pub const SYS_SIGPROCMASK: Sysno = 340;
pub const SYS_SIGSUSPEND: Sysno = 341;
pub const SYS_SIGPENDING: Sysno = 343;
pub const SYS_SIGTIMEDWAIT: Sysno = 345;
pub const SYS_SIGWAITINFO: Sysno = 346;
pub const SYS___ACL_GET_FILE: Sysno = 347;
pub const SYS___ACL_SET_FILE: Sysno = 348;
pub const SYS___ACL_GET_FD: Sysno = 349;
pub const SYS___ACL_SET_FD: Sysno = 350;
pub const SYS___ACL_DELETE_FILE: Sysno = 351;
pub const SYS___ACL_DELETE_FD: Sysno = 352;
pub const SYS___ACL_ACLCHECK_FILE: Sysno = 353;
pub const SYS___ACL_ACLCHECK_FD: Sysno = 354;
pub const SYS_EXTATTRCTL: Sysno = 355;
pub const SYS_EXTATTR_SET_FILE: Sysno = 356;
pub const SYS_EXTATTR_GET_FILE: Sysno = 357;
pub const SYS_EXTATTR_DELETE_FILE: Sysno = 358;
pub const SYS_AIO_WAITCOMPLETE: Sysno = 359;
pub const SYS_GETRESUID: Sysno = 360;
pub const SYS_GETRESGID: Sysno = 361;
pub const SYS_KQUEUE: Sysno = 362;
pub const SYS_FREEBSD11_KEVENT: Sysno = 363;
pub const SYS_EXTATTR_SET_FD: Sysno = 371;
pub const SYS_EXTATTR_GET_FD: Sysno = 372;
pub const SYS_EXTATTR_DELETE_FD: Sysno = 373;
pub const SYS___SETUGID: Sysno = 374;
pub const SYS_EACCESS: Sysno = 376;
pub const SYS_AFS3_SYSCALL: Sysno = 377;
pub const SYS_NMOUNT: Sysno = 378;
pub const SYS___MAC_GET_PROC: Sysno = 384;
pub const SYS___MAC_SET_PROC: Sysno = 385;
pub const SYS___MAC_GET_FD: Sysno = 386;
pub const SYS___MAC_GET_FILE: Sysno = 387;
pub const SYS___MAC_SET_FD: Sysno = 388;
pub const SYS___MAC_SET_FILE: Sysno = 389;
pub const SYS_KENV: Sysno = 390;
pub const SYS_LCHFLAGS: Sysno = 391;
pub const SYS_UUIDGEN: Sysno = 392;
pub const SYS_SENDFILE: Sysno = 393;
pub const SYS_MAC_SYSCALL: Sysno = 394;
pub const SYS_FREEBSD11_GETFSSTAT: Sysno = 395;
pub const SYS_FREEBSD11_STATFS: Sysno = 396;
pub const SYS_FREEBSD11_FSTATFS: Sysno = 397;
pub const SYS_FREEBSD11_FHSTATFS: Sysno = 398;
pub const SYS_KSEM_CLOSE: Sysno = 400;
pub const SYS_KSEM_POST: Sysno = 401;
pub const SYS_KSEM_WAIT: Sysno = 402;
pub const SYS_KSEM_TRYWAIT: Sysno = 403;
pub const SYS_KSEM_INIT: Sysno = 404;
pub const SYS_KSEM_OPEN: Sysno = 405;
pub const SYS_KSEM_UNLINK: Sysno = 406;
pub const SYS_KSEM_GETVALUE: Sysno = 407;
pub const SYS_KSEM_DESTROY: Sysno = 408;
pub const SYS___MAC_GET_PID: Sysno = 409;
pub const SYS___MAC_GET_LINK: Sysno = 410;
pub const SYS___MAC_SET_LINK: Sysno = 411;
pub const SYS_EXTATTR_SET_LINK: Sysno = 412;
pub const SYS_EXTATTR_GET_LINK: Sysno = 413;
pub const SYS_EXTATTR_DELETE_LINK: Sysno = 414;
pub const SYS___MAC_EXECVE: Sysno = 415;
pub const SYS_SIGACTION: Sysno = 416;
pub const SYS_SIGRETURN: Sysno = 417;
pub const SYS_GETCONTEXT: Sysno = 421;
pub const SYS_SETCONTEXT: Sysno = 422;
pub const SYS_SWAPCONTEXT: Sysno = 423;
pub const SYS_FREEBSD13_SWAPOFF: Sysno = 424;
pub const SYS___ACL_GET_LINK: Sysno = 425;
pub const SYS___ACL_SET_LINK: Sysno = 426;
pub const SYS___ACL_DELETE_LINK: Sysno = 427;
pub const SYS___ACL_ACLCHECK_LINK: Sysno = 428;
pub const SYS_SIGWAIT: Sysno = 429;
pub const SYS_THR_CREATE: Sysno = 430;
pub const SYS_THR_EXIT: Sysno = 431;
pub const SYS_THR_SELF: Sysno = 432;
pub const SYS_THR_KILL: Sysno = 433;
pub const SYS_FREEBSD10__UMTX_LOCK: Sysno = 434;
pub const SYS_FREEBSD10__UMTX_UNLOCK: Sysno = 435;
pub const SYS_JAIL_ATTACH: Sysno = 436;
pub const SYS_EXTATTR_LIST_FD: Sysno = 437;
pub const SYS_EXTATTR_LIST_FILE: Sysno = 438;
pub const SYS_EXTATTR_LIST_LINK: Sysno = 439;
pub const SYS_KSEM_TIMEDWAIT: Sysno = 441;
pub const SYS_THR_SUSPEND: Sysno = 442;
pub const SYS_THR_WAKE: Sysno = 443;
pub const SYS_KLDUNLOADF: Sysno = 444;
pub const SYS_AUDIT: Sysno = 445;
pub const SYS_AUDITON: Sysno = 446;
pub const SYS_GETAUID: Sysno = 447;
pub const SYS_SETAUID: Sysno = 448;
pub const SYS_GETAUDIT: Sysno = 449;
pub const SYS_SETAUDIT: Sysno = 450;
pub const SYS_GETAUDIT_ADDR: Sysno = 451;
pub const SYS_SETAUDIT_ADDR: Sysno = 452;
pub const SYS_AUDITCTL: Sysno = 453;
pub const SYS__UMTX_OP: Sysno = 454;
pub const SYS_THR_NEW: Sysno = 455;
pub const SYS_SIGQUEUE: Sysno = 456;
pub const SYS_KMQ_OPEN: Sysno = 457;
pub const SYS_KMQ_SETATTR: Sysno = 458;
pub const SYS_KMQ_TIMEDRECEIVE: Sysno = 459;
pub const SYS_KMQ_TIMEDSEND: Sysno = 460;
pub const SYS_KMQ_NOTIFY: Sysno = 461;
pub const SYS_KMQ_UNLINK: Sysno = 462;
pub const SYS_ABORT2: Sysno = 463;
pub const SYS_THR_SET_NAME: Sysno = 464;
pub const SYS_AIO_FSYNC: Sysno = 465;
pub const SYS_RTPRIO_THREAD: Sysno = 466;
pub const SYS_SCTP_PEELOFF: Sysno = 471;
pub const SYS_SCTP_GENERIC_SENDMSG: Sysno = 472;
pub const SYS_SCTP_GENERIC_SENDMSG_IOV: Sysno = 473;
pub const SYS_SCTP_GENERIC_RECVMSG: Sysno = 474;
pub const SYS_PREAD: Sysno = 475;
pub const SYS_PWRITE: Sysno = 476;
pub const SYS_MMAP: Sysno = 477;
pub const SYS_LSEEK: Sysno = 478;
pub const SYS_TRUNCATE: Sysno = 479;
pub const SYS_FTRUNCATE: Sysno = 480;
pub const SYS_THR_KILL2: Sysno = 481;
pub const SYS_FREEBSD12_SHM_OPEN: Sysno = 482;
pub const SYS_SHM_UNLINK: Sysno = 483;
pub const SYS_CPUSET: Sysno = 484;
pub const SYS_CPUSET_SETID: Sysno = 485;
pub const SYS_CPUSET_GETID: Sysno = 486;
pub const SYS_CPUSET_GETAFFINITY: Sysno = 487;
pub const SYS_CPUSET_SETAFFINITY: Sysno = 488;
pub const SYS_FACCESSAT: Sysno = 489;
pub const SYS_FCHMODAT: Sysno = 490;
pub const SYS_FCHOWNAT: Sysno = 491;
pub const SYS_FEXECVE: Sysno = 492;
pub const SYS_FREEBSD11_FSTATAT: Sysno = 493;
pub const SYS_FUTIMESAT: Sysno = 494;
pub const SYS_LINKAT: Sysno = 495;
pub const SYS_MKDIRAT: Sysno = 496;
pub const SYS_MKFIFOAT: Sysno = 497;
pub const SYS_FREEBSD11_MKNODAT: Sysno = 498;
pub const SYS_OPENAT: Sysno = 499;
pub const SYS_READLINKAT: Sysno = 500;
pub const SYS_RENAMEAT: Sysno = 501;
pub const SYS_SYMLINKAT: Sysno = 502;
pub const SYS_UNLINKAT: Sysno = 503;
pub const SYS_POSIX_OPENPT: Sysno = 504;
pub const SYS_GSSD_SYSCALL: Sysno = 505;
pub const SYS_JAIL_GET: Sysno = 506;
pub const SYS_JAIL_SET: Sysno = 507;
pub const SYS_JAIL_REMOVE: Sysno = 508;
pub const SYS_FREEBSD12_CLOSEFROM: Sysno = 509;
pub const SYS___SEMCTL: Sysno = 510;
pub const SYS_MSGCTL: Sysno = 511;
pub const SYS_SHMCTL: Sysno = 512;
pub const SYS_LPATHCONF: Sysno = 513;
pub const SYS___CAP_RIGHTS_GET: Sysno = 515;
pub const SYS_CAP_ENTER: Sysno = 516;
pub const SYS_CAP_GETMODE: Sysno = 517;
pub const SYS_PDFORK: Sysno = 518;
pub const SYS_PDKILL: Sysno = 519;
pub const SYS_PDGETPID: Sysno = 520;
pub const SYS_PSELECT: Sysno = 522;
pub const SYS_GETLOGINCLASS: Sysno = 523;
pub const SYS_SETLOGINCLASS: Sysno = 524;
pub const SYS_RCTL_GET_RACCT: Sysno = 525;
pub const SYS_RCTL_GET_RULES: Sysno = 526;
pub const SYS_RCTL_GET_LIMITS: Sysno = 527;
pub const SYS_RCTL_ADD_RULE: Sysno = 528;
pub const SYS_RCTL_REMOVE_RULE: Sysno = 529;
pub const SYS_POSIX_FALLOCATE: Sysno = 530;
pub const SYS_POSIX_FADVISE: Sysno = 531;
pub const SYS_WAIT6: Sysno = 532;
pub const SYS_CAP_RIGHTS_LIMIT: Sysno = 533;
pub const SYS_CAP_IOCTLS_LIMIT: Sysno = 534;
pub const SYS_CAP_IOCTLS_GET: Sysno = 535;
pub const SYS_CAP_FCNTLS_LIMIT: Sysno = 536;
pub const SYS_CAP_FCNTLS_GET: Sysno = 537;
pub const SYS_BINDAT: Sysno = 538;
pub const SYS_CONNECTAT: Sysno = 539;
pub const SYS_CHFLAGSAT: Sysno = 540;
pub const SYS_ACCEPT4: Sysno = 541;
pub const SYS_PIPE2: Sysno = 542;
pub const SYS_AIO_MLOCK: Sysno = 543;
pub const SYS_PROCCTL: Sysno = 544;
pub const SYS_PPOLL: Sysno = 545;
pub const SYS_FUTIMENS: Sysno = 546;
pub const SYS_UTIMENSAT: Sysno = 547;
pub const SYS_FDATASYNC: Sysno = 550;
pub const SYS_FSTAT: Sysno = 551;
pub const SYS_FSTATAT: Sysno = 552;
pub const SYS_FHSTAT: Sysno = 553;
pub const SYS_GETDIRENTRIES: Sysno = 554;
pub const SYS_STATFS: Sysno = 555;
pub const SYS_FSTATFS: Sysno = 556;
pub const SYS_GETFSSTAT: Sysno = 557;
pub const SYS_FHSTATFS: Sysno = 558;
pub const SYS_MKNODAT: Sysno = 559;
pub const SYS_KEVENT: Sysno = 560;
pub const SYS_CPUSET_GETDOMAIN: Sysno = 561;
pub const SYS_CPUSET_SETDOMAIN: Sysno = 562;
pub const SYS_GETRANDOM: Sysno = 563;
pub const SYS_GETFHAT: Sysno = 564;
pub const SYS_FHLINK: Sysno = 565;
pub const SYS_FHLINKAT: Sysno = 566;
pub const SYS_FHREADLINK: Sysno = 567;
pub const SYS_FUNLINKAT: Sysno = 568;
pub const SYS_COPY_FILE_RANGE: Sysno = 569;
pub const SYS___SYSCTLBYNAME: Sysno = 570;
pub const SYS_SHM_OPEN2: Sysno = 571;
pub const SYS_SHM_RENAME: Sysno = 572;
pub const SYS_SIGFASTBLOCK: Sysno = 573;
pub const SYS___REALPATHAT: Sysno = 574;
pub const SYS_CLOSE_RANGE: Sysno = 575;
pub const SYS_RPCTLS_SYSCALL: Sysno = 576;
pub const SYS___SPECIALFD: Sysno = 577;
pub const SYS_AIO_WRITEV: Sysno = 578;
pub const SYS_AIO_READV: Sysno = 579;
pub const SYS_SCHED_GETCPU: Sysno = 581;
pub const SYS_SWAPOFF: Sysno = 582;
