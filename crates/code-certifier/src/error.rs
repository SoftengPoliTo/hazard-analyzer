//! This module handles errors.

use std::borrow::Cow;

/// All possible error kinds.
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    /// I/O error.
    Io,
    /// Git error.
    Git,
    /// Toml error.
    Toml,
    /// Rustdoc error.
    Rustdoc,
    /// Concurrent error.
    Concurrent,
    /// Json error.
    Json,
}

impl ErrorKind {
    pub(crate) const fn description(self) -> &'static str {
        match self {
            ErrorKind::Io => "I/O error",
            ErrorKind::Git => "Git error",
            ErrorKind::Toml => "TOML error",
            ErrorKind::Rustdoc => "Rustdoc error",
            ErrorKind::Concurrent => "Concurrent error",
            ErrorKind::Json => "JSON error",
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.description().fmt(f)
    }
}

/// Library error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    info: Cow<'static, str>,
}

impl Error {
    /// Creates a new [`Error`] instance.
    pub fn new(kind: ErrorKind, info: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind,
            info: info.into(),
        }
    }

    pub(crate) fn error(&self) -> String {
        format!("{}: {}", self.kind, self.info)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error().fmt(f)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::new(ErrorKind::Io, e.to_string())
    }
}

impl<T> From<crossbeam::channel::SendError<T>> for Error {
    fn from(_: crossbeam::channel::SendError<T>) -> Self {
        Self::new(ErrorKind::Concurrent, "Sender error")
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::new(ErrorKind::Json, e.to_string())
    }
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        Self::new(ErrorKind::Git, e.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::new(ErrorKind::Toml, e.to_string())
    }
}

impl From<rustdoc_json::BuildError> for Error {
    fn from(e: rustdoc_json::BuildError) -> Self {
        Self::new(ErrorKind::Rustdoc, e.to_string())
    }
}

/// A specialized `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
