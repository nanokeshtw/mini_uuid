use base64::{DecodeError, DecodeSliceError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
  InvalidBase64,
  InvalidUuid,
  InvalidUuidSize,
}

#[derive(Debug)]
pub enum Error {
  InvalidBase64(DecodeError),
  InvalidUuid(uuid::Error),
  InvalidUuidSize,
}

impl From<DecodeError> for Error {
  fn from(value: DecodeError) -> Self {
    Self::InvalidBase64(value)
  }
}

impl From<DecodeSliceError> for Error {
  fn from(e: DecodeSliceError) -> Self {
    match e {
      DecodeSliceError::DecodeError(err) => Self::InvalidBase64(err),
      _ => Self::InvalidUuidSize,
    }
  }
}

impl From<uuid::Error> for Error {
  fn from(e: uuid::Error) -> Self {
    Self::InvalidUuid(e)
  }
}

impl Into<ErrorKind> for Error {
  fn into(self) -> ErrorKind {
    match self {
      Self::InvalidBase64(_) => ErrorKind::InvalidBase64,
      Self::InvalidUuid(_) => ErrorKind::InvalidUuid,
      Self::InvalidUuidSize => ErrorKind::InvalidUuidSize,
    }
  }
}

pub type Result<T> = std::result::Result<T, Error>;
