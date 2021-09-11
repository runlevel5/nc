// Code generated by mkerrno_freebsd.py; DO NOT EDIT.

use crate::syscalls::Errno;

/// Operation not permitted
pub const EPERM: Errno = 1;
/// No such file or directory
pub const ENOENT: Errno = 2;
/// No such process
pub const ESRCH: Errno = 3;
/// Interrupted system call
pub const EINTR: Errno = 4;
/// Input/output error
pub const EIO: Errno = 5;
/// Device not configured
pub const ENXIO: Errno = 6;
/// Argument list too long
pub const E2BIG: Errno = 7;
/// Exec format error
pub const ENOEXEC: Errno = 8;
/// Bad file descriptor
pub const EBADF: Errno = 9;
/// No child processes
pub const ECHILD: Errno = 10;
/// Resource deadlock avoided
pub const EDEADLK: Errno = 11;
/// Cannot allocate memory
pub const ENOMEM: Errno = 12;
/// Permission denied
pub const EACCES: Errno = 13;
/// Bad address
pub const EFAULT: Errno = 14;
/// Block device required
pub const ENOTBLK: Errno = 15;
/// Device busy
pub const EBUSY: Errno = 16;
/// File exists
pub const EEXIST: Errno = 17;
/// Cross-device link
pub const EXDEV: Errno = 18;
/// Operation not supported by device
pub const ENODEV: Errno = 19;
/// Not a directory
pub const ENOTDIR: Errno = 20;
/// Is a directory
pub const EISDIR: Errno = 21;
/// Invalid argument
pub const EINVAL: Errno = 22;
/// Too many open files in system
pub const ENFILE: Errno = 23;
/// Too many open files
pub const EMFILE: Errno = 24;
/// Inappropriate ioctl for device
pub const ENOTTY: Errno = 25;
/// Text file busy
pub const ETXTBSY: Errno = 26;
/// File too large
pub const EFBIG: Errno = 27;
/// No space left on device
pub const ENOSPC: Errno = 28;
/// Illegal seek
pub const ESPIPE: Errno = 29;
/// Read-only filesystem
pub const EROFS: Errno = 30;
/// Too many links
pub const EMLINK: Errno = 31;
/// Broken pipe
pub const EPIPE: Errno = 32;
/// Numerical argument out of domain
pub const EDOM: Errno = 33;
/// Result too large
pub const ERANGE: Errno = 34;
/// Resource temporarily unavailable
pub const EAGAIN: Errno = 35;
pub const EWOULDBLOCK: Errno = EAGAIN;
/// Operation now in progress
pub const EINPROGRESS: Errno = 36;
/// Operation already in progress
pub const EALREADY: Errno = 37;
/// Socket operation on non-socket
pub const ENOTSOCK: Errno = 38;
/// Destination address required
pub const EDESTADDRREQ: Errno = 39;
/// Message too long
pub const EMSGSIZE: Errno = 40;
/// Protocol wrong type for socket
pub const EPROTOTYPE: Errno = 41;
/// Protocol not available
pub const ENOPROTOOPT: Errno = 42;
/// Protocol not supported
pub const EPROTONOSUPPORT: Errno = 43;
/// Socket type not supported
pub const ESOCKTNOSUPPORT: Errno = 44;
/// Operation not supported
pub const EOPNOTSUPP: Errno = 45;
pub const ENOTSUP: Errno = EOPNOTSUPP;
/// Protocol family not supported
pub const EPFNOSUPPORT: Errno = 46;
/// Address family not supported by protocol family
pub const EAFNOSUPPORT: Errno = 47;
/// Address already in use
pub const EADDRINUSE: Errno = 48;
/// Can't assign requested address
pub const EADDRNOTAVAIL: Errno = 49;
/// Network is down
pub const ENETDOWN: Errno = 50;
/// Network is unreachable
pub const ENETUNREACH: Errno = 51;
/// Network dropped connection on reset
pub const ENETRESET: Errno = 52;
/// Software caused connection abort
pub const ECONNABORTED: Errno = 53;
/// Connection reset by peer
pub const ECONNRESET: Errno = 54;
/// No buffer space available
pub const ENOBUFS: Errno = 55;
/// Socket is already connected
pub const EISCONN: Errno = 56;
/// Socket is not connected
pub const ENOTCONN: Errno = 57;
/// Can't send after socket shutdown
pub const ESHUTDOWN: Errno = 58;
/// Too many references: can't splice
pub const ETOOMANYREFS: Errno = 59;
/// Operation timed out
pub const ETIMEDOUT: Errno = 60;
/// Connection refused
pub const ECONNREFUSED: Errno = 61;
/// Too many levels of symbolic links
pub const ELOOP: Errno = 62;
/// File name too long
pub const ENAMETOOLONG: Errno = 63;
/// Host is down
pub const EHOSTDOWN: Errno = 64;
/// No route to host
pub const EHOSTUNREACH: Errno = 65;
/// Directory not empty
pub const ENOTEMPTY: Errno = 66;
/// Too many processes
pub const EPROCLIM: Errno = 67;
/// Too many users
pub const EUSERS: Errno = 68;
/// Disc quota exceeded
pub const EDQUOT: Errno = 69;
/// Stale NFS file handle
pub const ESTALE: Errno = 70;
/// Too many levels of remote in path
pub const EREMOTE: Errno = 71;
/// RPC struct is bad
pub const EBADRPC: Errno = 72;
/// RPC version wrong
pub const ERPCMISMATCH: Errno = 73;
/// RPC prog. not avail
pub const EPROGUNAVAIL: Errno = 74;
/// Program version wrong
pub const EPROGMISMATCH: Errno = 75;
/// Bad procedure for program
pub const EPROCUNAVAIL: Errno = 76;
/// No locks available
pub const ENOLCK: Errno = 77;
/// Function not implemented
pub const ENOSYS: Errno = 78;
/// Inappropriate file type or format
pub const EFTYPE: Errno = 79;
/// Authentication error
pub const EAUTH: Errno = 80;
/// Need authenticator
pub const ENEEDAUTH: Errno = 81;
/// Identifier removed
pub const EIDRM: Errno = 82;
/// No message of desired type
pub const ENOMSG: Errno = 83;
/// Value too large to be stored in data type
pub const EOVERFLOW: Errno = 84;
/// Operation canceled
pub const ECANCELED: Errno = 85;
/// Illegal byte sequence
pub const EILSEQ: Errno = 86;
/// Attribute not found
pub const ENOATTR: Errno = 87;
/// Programming error
pub const EDOOFUS: Errno = 88;
/// Bad message
pub const EBADMSG: Errno = 89;
/// Multihop attempted
pub const EMULTIHOP: Errno = 90;
/// Link has been severed
pub const ENOLINK: Errno = 91;
/// Protocol error
pub const EPROTO: Errno = 92;
/// Capabilities insufficient
pub const ENOTCAPABLE: Errno = 93;
/// Not permitted in capability mode
pub const ECAPMODE: Errno = 94;
/// State not recoverable
pub const ENOTRECOVERABLE: Errno = 95;
/// Previous owner died
pub const EOWNERDEAD: Errno = 96;
/// Integrity check failed
pub const EINTEGRITY: Errno = 97;

/// Get errno description.
pub fn strerror(errno: Errno) -> &'static str {
    match errno {
        EPERM => "Operation not permitted",
        ENOENT => "No such file or directory",
        ESRCH => "No such process",
        EINTR => "Interrupted system call",
        EIO => "Input/output error",
        ENXIO => "Device not configured",
        E2BIG => "Argument list too long",
        ENOEXEC => "Exec format error",
        EBADF => "Bad file descriptor",
        ECHILD => "No child processes",
        EDEADLK => "Resource deadlock avoided",
        ENOMEM => "Cannot allocate memory",
        EACCES => "Permission denied",
        EFAULT => "Bad address",
        ENOTBLK => "Block device required",
        EBUSY => "Device busy",
        EEXIST => "File exists",
        EXDEV => "Cross-device link",
        ENODEV => "Operation not supported by device",
        ENOTDIR => "Not a directory",
        EISDIR => "Is a directory",
        EINVAL => "Invalid argument",
        ENFILE => "Too many open files in system",
        EMFILE => "Too many open files",
        ENOTTY => "Inappropriate ioctl for device",
        ETXTBSY => "Text file busy",
        EFBIG => "File too large",
        ENOSPC => "No space left on device",
        ESPIPE => "Illegal seek",
        EROFS => "Read-only filesystem",
        EMLINK => "Too many links",
        EPIPE => "Broken pipe",
        EDOM => "Numerical argument out of domain",
        ERANGE => "Result too large",
        EAGAIN => "Resource temporarily unavailable",
        EINPROGRESS => "Operation now in progress",
        EALREADY => "Operation already in progress",
        ENOTSOCK => "Socket operation on non-socket",
        EDESTADDRREQ => "Destination address required",
        EMSGSIZE => "Message too long",
        EPROTOTYPE => "Protocol wrong type for socket",
        ENOPROTOOPT => "Protocol not available",
        EPROTONOSUPPORT => "Protocol not supported",
        ESOCKTNOSUPPORT => "Socket type not supported",
        EOPNOTSUPP => "Operation not supported",
        EPFNOSUPPORT => "Protocol family not supported",
        EAFNOSUPPORT => "Address family not supported by protocol family",
        EADDRINUSE => "Address already in use",
        EADDRNOTAVAIL => "Can't assign requested address",
        ENETDOWN => "Network is down",
        ENETUNREACH => "Network is unreachable",
        ENETRESET => "Network dropped connection on reset",
        ECONNABORTED => "Software caused connection abort",
        ECONNRESET => "Connection reset by peer",
        ENOBUFS => "No buffer space available",
        EISCONN => "Socket is already connected",
        ENOTCONN => "Socket is not connected",
        ESHUTDOWN => "Can't send after socket shutdown",
        ETOOMANYREFS => "Too many references: can't splice",
        ETIMEDOUT => "Operation timed out",
        ECONNREFUSED => "Connection refused",
        ELOOP => "Too many levels of symbolic links",
        ENAMETOOLONG => "File name too long",
        EHOSTDOWN => "Host is down",
        EHOSTUNREACH => "No route to host",
        ENOTEMPTY => "Directory not empty",
        EPROCLIM => "Too many processes",
        EUSERS => "Too many users",
        EDQUOT => "Disc quota exceeded",
        ESTALE => "Stale NFS file handle",
        EREMOTE => "Too many levels of remote in path",
        EBADRPC => "RPC struct is bad",
        ERPCMISMATCH => "RPC version wrong",
        EPROGUNAVAIL => "RPC prog. not avail",
        EPROGMISMATCH => "Program version wrong",
        EPROCUNAVAIL => "Bad procedure for program",
        ENOLCK => "No locks available",
        ENOSYS => "Function not implemented",
        EFTYPE => "Inappropriate file type or format",
        EAUTH => "Authentication error",
        ENEEDAUTH => "Need authenticator",
        EIDRM => "Identifier removed",
        ENOMSG => "No message of desired type",
        EOVERFLOW => "Value too large to be stored in data type",
        ECANCELED => "Operation canceled",
        EILSEQ => "Illegal byte sequence",
        ENOATTR => "Attribute not found",
        EDOOFUS => "Programming error",
        EBADMSG => "Bad message",
        EMULTIHOP => "Multihop attempted",
        ENOLINK => "Link has been severed",
        EPROTO => "Protocol error",
        ENOTCAPABLE => "Capabilities insufficient",
        ECAPMODE => "Not permitted in capability mode",
        ENOTRECOVERABLE => "State not recoverable",
        EOWNERDEAD => "Previous owner died",
        EINTEGRITY => "Integrity check failed",

        _ => "Unknown errno!",
    }
}
