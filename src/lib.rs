#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

use core::fmt;

mod linux_errno {
    include!(concat!(env!("OUT_DIR"), "/linux_errno.rs"));
}

pub use linux_errno::LinuxError;

/// A specialized [`Result`] type with [`LinuxError`] as the error type.
pub type LinuxResult<T = ()> = Result<T, LinuxError>;

/// A specialized [`Result`] type with [`AxError`] as the error type.
pub type AxResult<T = ()> = Result<T, AxError>;

/// The error type used by ArceOS.
///
/// Similar to [`std::io::ErrorKind`].
///
/// [`std::io::ErrorKind`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html
#[repr(i32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AxError {
    /// A socket address could not be bound because the address is already in use elsewhere.
    AddressInUse = 1,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// Bad address.
    BadAddress,
    /// Bad internal state.
    BadState,
    /// The connection was refused by the remote server,
    ConnectionRefused,
    /// The connection was reset by the remote server.
    ConnectionReset,
    /// A non-empty directory was specified where an empty directory was expected.
    DirectoryNotEmpty,
    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: AxError::InvalidInput
    InvalidData,
    /// Invalid parameter/argument.
    InvalidInput,
    /// Input/output error.
    Io,
    /// The filesystem object is, unexpectedly, a directory.
    IsADirectory,
    /// Not enough space/cannot allocate memory.
    NoMemory,
    /// A filesystem object is, unexpectedly, not a directory.
    NotADirectory,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// The requested entity is not found.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// Device or resource is busy.
    ResourceBusy,
    /// The underlying storage (typically, a filesystem) is full.
    StorageFull,
    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    UnexpectedEof,
    /// This operation is unsupported or unimplemented.
    Unsupported,
    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,
    /// An error returned when an operation could not be completed because a
    /// call to `write()` returned [`Ok(0)`](Ok).
    WriteZero,
    /// Argument list is too long.
    TooBig,
    /// Cross-device or cross-filesystem (hard) link or rename.
    CrossesDevices,
    /// Inappropriate ioctl for device.
    BadIoctl,
    /// Filename is too long.
    NameTooLong,
    /// Bad file descriptor.
    BadFileDescriptor,
    /// Loop in the filesystem or IO subsystem; often, too many levels of
    /// symbolic links.
    FilesystemLoop,
    /// Operation not supported.
    OperationNotSupported,
    /// The operation was partially successful and needs to be checked later on
    /// due to not blocking.
    InProgress,
    /// The socket is already connected.
    AlreadyConnected,
    /// The operation was interrupted by a signal.
    Interrupted,
    /// The I/O operation’s timeout expired, causing it to be canceled.
    TimedOut,
    /// The specified entity is not a socket.
    NotASocket,
    /// Broken pipe
    BrokenPipe,
    /// The process has too many files open.
    TooManyOpenFiles,
    /// No such process.
    NoSuchProcess,
    /// Illegal byte sequence.
    IllegalBytes,
    /// Operation not permitted.
    OperationNotPermitted,
    /// Result out of range.
    OutOfRange,
    /// Invalid executable format.
    InvalidExecutable,
    /// No such device.
    NoSuchDevice,
    /// The filesystem is read-only.
    ReadOnlyFilesystem,
    /// Other error with the given Linux errno code.
    Other(LinuxError),
}

impl AxError {
    /// Returns the error description.
    pub fn as_str(&self) -> &'static str {
        use AxError::*;
        match *self {
            AddressInUse => "Address in use",
            BadAddress => "Bad address",
            BadState => "Bad internal state",
            AlreadyExists => "Entity already exists",
            ConnectionRefused => "Connection refused",
            ConnectionReset => "Connection reset",
            DirectoryNotEmpty => "Directory not empty",
            InvalidData => "Invalid data",
            InvalidInput => "Invalid input parameter",
            Io => "I/O error",
            IsADirectory => "Is a directory",
            NoMemory => "Out of memory",
            NotADirectory => "Not a directory",
            NotConnected => "Not connected",
            NotFound => "Entity not found",
            PermissionDenied => "Permission denied",
            ResourceBusy => "Resource busy",
            StorageFull => "No storage space",
            UnexpectedEof => "Unexpected end of file",
            Unsupported => "Operation not supported",
            WouldBlock => "Operation would block",
            WriteZero => "Write zero",
            TooBig => "Argument list too long",
            CrossesDevices => "Cross-device link or rename",
            BadIoctl => "Inappropriate ioctl for device",
            NameTooLong => "Filename too long",
            BadFileDescriptor => "Bad file descriptor",
            FilesystemLoop => "Filesystem loop or indirection limit",
            OperationNotSupported => "Operation not supported",
            InProgress => "Operation in progress",
            AlreadyConnected => "Already connected",
            Interrupted => "Operation interrupted",
            TimedOut => "Timed out",
            NotASocket => "Not a socket",
            BrokenPipe => "Broken pipe",
            TooManyOpenFiles => "Too many open files",
            NoSuchProcess => "No such process",
            IllegalBytes => "Illegal byte sequence",
            OperationNotPermitted => "Operation not permitted",
            OutOfRange => "Result out of range",
            InvalidExecutable => "Invalid executable format",
            NoSuchDevice => "No such device",
            ReadOnlyFilesystem => "Read-only filesystem",
            Other(errno) => errno.as_str(),
        }
    }
}

impl fmt::Display for AxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<AxError> for LinuxError {
    fn from(e: AxError) -> Self {
        use AxError::*;
        match e {
            AddressInUse => LinuxError::EADDRINUSE,
            AlreadyExists => LinuxError::EEXIST,
            BadAddress | BadState => LinuxError::EFAULT,
            ConnectionRefused => LinuxError::ECONNREFUSED,
            ConnectionReset => LinuxError::ECONNRESET,
            DirectoryNotEmpty => LinuxError::ENOTEMPTY,
            InvalidInput | InvalidData => LinuxError::EINVAL,
            Io => LinuxError::EIO,
            IsADirectory => LinuxError::EISDIR,
            NoMemory => LinuxError::ENOMEM,
            NotADirectory => LinuxError::ENOTDIR,
            NotConnected => LinuxError::ENOTCONN,
            NotFound => LinuxError::ENOENT,
            PermissionDenied => LinuxError::EACCES,
            ResourceBusy => LinuxError::EBUSY,
            StorageFull => LinuxError::ENOSPC,
            Unsupported => LinuxError::ENOSYS,
            UnexpectedEof | WriteZero => LinuxError::EIO,
            WouldBlock => LinuxError::EAGAIN,
            TooBig => LinuxError::E2BIG,
            CrossesDevices => LinuxError::EXDEV,
            BadIoctl => LinuxError::ENOTTY,
            NameTooLong => LinuxError::ENAMETOOLONG,
            BadFileDescriptor => LinuxError::EBADF,
            FilesystemLoop => LinuxError::ELOOP,
            OperationNotSupported => LinuxError::EOPNOTSUPP,
            InProgress => LinuxError::EINPROGRESS,
            AlreadyConnected => LinuxError::EISCONN,
            Interrupted => LinuxError::EINTR,
            TimedOut => LinuxError::ETIMEDOUT,
            NotASocket => LinuxError::ENOTSOCK,
            BrokenPipe => LinuxError::EPIPE,
            TooManyOpenFiles => LinuxError::EMFILE,
            NoSuchProcess => LinuxError::ESRCH,
            IllegalBytes => LinuxError::EILSEQ,
            OperationNotPermitted => LinuxError::EPERM,
            OutOfRange => LinuxError::ERANGE,
            InvalidExecutable => LinuxError::ENOEXEC,
            NoSuchDevice => LinuxError::ENODEV,
            ReadOnlyFilesystem => LinuxError::EROFS,
            Other(errno) => errno,
        }
    }
}

impl TryFrom<LinuxError> for AxError {
    type Error = LinuxError;

    fn try_from(e: LinuxError) -> Result<Self, Self::Error> {
        use AxError::*;
        use LinuxError::*;
        Ok(match e {
            EADDRINUSE => AddressInUse,
            EEXIST => AlreadyExists,
            EFAULT => BadAddress,
            ECONNREFUSED => ConnectionRefused,
            ECONNRESET => ConnectionReset,
            ENOTEMPTY => DirectoryNotEmpty,
            EINVAL => InvalidInput,
            EIO => Io,
            EISDIR => IsADirectory,
            ENOMEM => NoMemory,
            ENOTDIR => NotADirectory,
            ENOTCONN => NotConnected,
            ENOENT => NotFound,
            EACCES => PermissionDenied,
            EBUSY => ResourceBusy,
            ENOSPC => StorageFull,
            ENOSYS => Unsupported,
            EAGAIN => WouldBlock,
            E2BIG => TooBig,
            EXDEV => CrossesDevices,
            ENOTTY => BadIoctl,
            ENAMETOOLONG => NameTooLong,
            EBADF => BadFileDescriptor,
            ELOOP => FilesystemLoop,
            EOPNOTSUPP => OperationNotSupported,
            EINPROGRESS => InProgress,
            EISCONN => AlreadyConnected,
            EINTR => Interrupted,
            ETIMEDOUT => TimedOut,
            ENOTSOCK => NotASocket,
            EPIPE => BrokenPipe,
            EMFILE => TooManyOpenFiles,
            ESRCH => NoSuchProcess,
            EILSEQ => IllegalBytes,
            EPERM => OperationNotPermitted,
            ERANGE => OutOfRange,
            ENOEXEC => InvalidExecutable,
            ENODEV => NoSuchDevice,
            EROFS => ReadOnlyFilesystem,
            _ => {
                return Err(e);
            }
        })
    }
}

/// Convenient method to construct an [`LinuxError`] type while printing a
/// warning message.
///
/// # Examples
///
/// ```
/// # use axerrno::{ax_err, LinuxError};
/// #
/// // Also print "[ENOMEM]" if the `log` crate is enabled.
/// assert_eq!(
///     ax_err!(ENOMEM),
///     LinuxError::ENOMEM,
/// );
///
/// // Also print "[EFAULT] the address is 0!" if the `log` crate
/// // is enabled.
/// assert_eq!(
///     ax_err!(EFAULT, "the address is 0!"),
///     LinuxError::EFAULT,
/// );
/// ```
#[macro_export]
macro_rules! ax_err {
    ($err: ident) => {{
        use $crate::AxError::*;
        $crate::__priv::warn!("[{:?}]", $err);
        $err
    }};
    ($err: ident, $msg: expr) => {{
        use $crate::AxError::*;
        $crate::__priv::warn!("[{:?}] {}", $err, $msg);
        $err
    }};
}

/// Throws an error of type [`LinuxError`] with the given error code, optionally
/// with a message.
#[macro_export]
macro_rules! bail {
    ($($t:tt)*) => {
        return Err($crate::ax_err!($($t)*));
    };
}

impl fmt::Display for LinuxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[doc(hidden)]
pub mod __priv {
    pub use log::warn;
}
