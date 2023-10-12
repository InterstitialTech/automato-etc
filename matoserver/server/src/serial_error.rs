use elm_rs::{Elm, ElmJson};
use serde_derive::{Deserialize, Serialize};
use serialport;

#[derive(Serialize, Deserialize, Debug, Clone, Elm, ElmJson)]
pub enum ErrorKind {
    /// The device is not available.
    ///
    /// This could indicate that the device is in use by another process or was
    /// disconnected while performing I/O.
    NoDevice,

    /// A parameter was incorrect.
    InvalidInput,

    /// An unknown error occurred.
    Unknown,

    /// An I/O error occurred.
    ///
    /// The type of I/O error is determined by the inner `io::ErrorKind`.
    Io(Result<IOErrorKind, String>),
}

impl From<serialport::ErrorKind> for ErrorKind {
    fn from(kind: serialport::ErrorKind) -> ErrorKind {
        match kind {
            serialport::ErrorKind::NoDevice => ErrorKind::NoDevice,
            serialport::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
            serialport::ErrorKind::Unknown => ErrorKind::Unknown,
            serialport::ErrorKind::Io(e) => ErrorKind::Io(ioe_from(e)),
        }
    }
}

/// An error type for serial port operations
#[derive(Serialize, Deserialize, Debug, Clone, Elm, ElmJson)]
pub struct Error {
    /// The kind of error this is
    pub kind: ErrorKind,
    /// A description of the error suitable for end-users
    pub description: String,
}

impl From<serialport::Error> for Error {
    fn from(e: serialport::Error) -> Error {
        Error {
            kind: ErrorKind::from(e.kind),
            description: e.description.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Elm, ElmJson)]
pub enum IOErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    HostUnreachable,
    NetworkUnreachable,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    NetworkDown,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    NotADirectory,
    IsADirectory,
    DirectoryNotEmpty,
    ReadOnlyFilesystem,
    FilesystemLoop,
    StaleNetworkFileHandle,
    InvalidInputX, // must be different from the ErrorKind::InvalidInput because elm
    InvalidData,
    TimedOut,
    WriteZero,
    StorageFull,
    NotSeekable,
    FilesystemQuotaExceeded,
    FileTooLarge,
    ResourceBusy,
    ExecutableFileBusy,
    Deadlock,
    CrossesDevices,
    TooManyLinks,
    InvalidFilename,
    ArgumentListTooLong,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
    Uncategorized,
}

/// fragile conversion function.
pub fn ioe_from(kind: std::io::ErrorKind) -> Result<IOErrorKind, String> {
    // Err(format!("unsupported std::io::ErrorKind: {}", kind))

    // unstable errors are commented.
    match kind {
        std::io::ErrorKind::NotFound => Ok(IOErrorKind::NotFound),
        std::io::ErrorKind::PermissionDenied => Ok(IOErrorKind::PermissionDenied),
        std::io::ErrorKind::ConnectionRefused => Ok(IOErrorKind::ConnectionRefused),
        std::io::ErrorKind::ConnectionReset => Ok(IOErrorKind::ConnectionReset),
        // std::io::ErrorKind::HostUnreachable => Ok(IOErrorKind::HostUnreachable),
        // std::io::ErrorKind::NetworkUnreachable => Ok(IOErrorKind::NetworkUnreachable),
        std::io::ErrorKind::ConnectionAborted => Ok(IOErrorKind::ConnectionAborted),
        std::io::ErrorKind::NotConnected => Ok(IOErrorKind::NotConnected),
        std::io::ErrorKind::AddrInUse => Ok(IOErrorKind::AddrInUse),
        std::io::ErrorKind::AddrNotAvailable => Ok(IOErrorKind::AddrNotAvailable),
        // std::io::ErrorKind::NetworkDown => Ok(IOErrorKind::NetworkDown),
        std::io::ErrorKind::BrokenPipe => Ok(IOErrorKind::BrokenPipe),
        std::io::ErrorKind::AlreadyExists => Ok(IOErrorKind::AlreadyExists),
        std::io::ErrorKind::WouldBlock => Ok(IOErrorKind::WouldBlock),
        // std::io::ErrorKind::NotADirectory => Ok(IOErrorKind::NotADirectory),
        // std::io::ErrorKind::IsADirectory => Ok(IOErrorKind::IsADirectory),
        // std::io::ErrorKind::DirectoryNotEmpty => Ok(IOErrorKind::DirectoryNotEmpty),
        // std::io::ErrorKind::ReadOnlyFilesystem => Ok(IOErrorKind::ReadOnlyFilesystem),
        // std::io::ErrorKind::FilesystemLoop => Ok(IOErrorKind::FilesystemLoop),
        // std::io::ErrorKind::StaleNetworkFileHandle => Ok(IOErrorKind::StaleNetworkFileHandle),
        std::io::ErrorKind::InvalidInput => Ok(IOErrorKind::InvalidInputX),
        std::io::ErrorKind::InvalidData => Ok(IOErrorKind::InvalidData),
        std::io::ErrorKind::TimedOut => Ok(IOErrorKind::TimedOut),
        std::io::ErrorKind::WriteZero => Ok(IOErrorKind::WriteZero),
        // std::io::ErrorKind::StorageFull => Ok(IOErrorKind::StorageFull),
        // std::io::ErrorKind::NotSeekable => Ok(IOErrorKind::NotSeekable),
        // std::io::ErrorKind::FilesystemQuotaExceeded => Ok(IOErrorKind::FilesystemQuotaExceeded),
        // std::io::ErrorKind::FileTooLarge => Ok(IOErrorKind::FileTooLarge),
        // std::io::ErrorKind::ResourceBusy => Ok(IOErrorKind::ResourceBusy),
        // std::io::ErrorKind::ExecutableFileBusy => Ok(IOErrorKind::ExecutableFileBusy),
        // std::io::ErrorKind::Deadlock => Ok(IOErrorKind::Deadlock),
        // std::io::ErrorKind::CrossesDevices => Ok(IOErrorKind::CrossesDevices),
        // std::io::ErrorKind::TooManyLinks => Ok(IOErrorKind::TooManyLinks),
        // std::io::ErrorKind::InvalidFilename => Ok(IOErrorKind::InvalidFilename),
        // std::io::ErrorKind::ArgumentListTooLong => Ok(IOErrorKind::ArgumentListTooLong),
        std::io::ErrorKind::Interrupted => Ok(IOErrorKind::Interrupted),
        std::io::ErrorKind::Unsupported => Ok(IOErrorKind::Unsupported),
        std::io::ErrorKind::UnexpectedEof => Ok(IOErrorKind::UnexpectedEof),
        std::io::ErrorKind::OutOfMemory => Ok(IOErrorKind::OutOfMemory),
        std::io::ErrorKind::Other => Ok(IOErrorKind::Other),
        // std::io::ErrorKind::Uncategorized => Ok(IOErrorKind::Uncategorized),
        _ => Err(format!("unsupported std::io::ErrorKind: {}", kind)),
    }
}
