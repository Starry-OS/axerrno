#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

use core::fmt;

mod linux_errno {
    include!(concat!(env!("OUT_DIR"), "/linux_errno.rs"));
}

pub use linux_errno::LinuxError;

/// A specialized [`Result`] type with [`LinuxError`] as the error type.
pub type LinuxResult<T = ()> = Result<T, LinuxError>;

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
        use $crate::LinuxError::*;
        $crate::__priv::warn!("[{:?}]", $err);
        $err
    }};
    ($err: ident, $msg: expr) => {{
        use $crate::LinuxError::*;
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
