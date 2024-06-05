// Copyright 2024 Gavin Noktes
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # :mouse2: mini_uuid
//!
//! **mini_uuid** is a Rust crate that converts UUIDs to a compact base64 representation, making them shorter and more efficient for storage and transmission!
//!
//! **P.S, I made this in my free time for fun when I was bored one day, not sure how much I will iterate this, please don't use in production unless you know the code you are using**
//!
//! ## Features
//!
//! You can use this crate with `v1`, `v4`, or `v7` uuids. Padding is optional, with the `padding` feature and you can enable url safety with the `url_safe` feature.
//!
//! ## Usage
//!
//! ```rust
//!
//! use mini_uuid::MiniUuid;
//!
//! fn main() {
//!   // Generate a UUID
//!   let uuid = uuid::Uuid::new_v4();
//!
//!   // Convert to mini UUID (base64)
//!   let mini = MiniUuid::from_uuid(&uuid);
//!   // or just create it without the `uuid` crate
//!   let mini = MiniUuid::new();
//!   println!("Mini UUID: {}", mini.to_string());
//!
//!   // Convert back to UUID
//!   let original_uuid = mini.to_uuid().unwrap();
//!   println!("Original UUID: {}", original_uuid);
//!
//! }
//! ```
//!
//! ## Methods
//!
//! ```rust
//! // Creates a `MiniUuid` from a `uuid::Uuid`
//! let mini = MiniUuid::from_uuid(uuid::Uuid::new_v4());
//!
//! // Creates a `MiniUuid` from a string representing `uuid::Uuid`
//! let mini = MiniUuid::from_uuid_str("67e55044-10b1-426f-9247-bb680e5fe0c8");
//!
//! // Creates a `MiniUuid` from a string by checking the `padding` and `url_safe` features
//! let mini = MiniUuid::from_string("/*INSERT MINI UUID*/");
//!
//! // Creates a `MiniUuid` from a url safe base64 string with padding
//! let mini = MiniUuid::from_url_base64("Z+VQRBCxQm+SR7toDl/gyA==");
//!
//! // Creates a `MiniUuid` from a url safe base64 string without padding
//! let mini = MiniUuid::from_url_base64_no_pad("Z+VQRBCxQm+SR7toDl/gyA");
//!
//! // Creates a `MiniUuid` from a base64 string with padding
//! let mini = MiniUuid::from_base64("Z-VQRBCxQm-SR7toDl_gyA==");
//!
//! // Creates a `MiniUuid` from a base64 string without padding
//! let mini = MiniUuid::from_base64_no_pad("Z-VQRBCxQm-SR7toDl_gyA");
//!
//! // Converts `MiniUuid` to `uuid::Uuid`
//! let uuid = mini.to_uuid();
//!
//! // Converts `MiniUuid` to a string by checking the `padding` and `url_safe` features
//! let uuid = mini.to_string();
//!
//! // Converts `MiniUuid` to url safe base64 string with padding
//! let uuid = mini.to_url_base64();
//!
//! // Converts `MiniUuid` to url safe base64 string without padding
//! let uuid = mini.to_url_base64_no_pad();
//!
//! // Converts `MiniUuid` to base64 string with padding
//! let uuid = mini.to_base64();
//!
//! // Converts `MiniUuid` to base64 string without padding
//! let uuid = mini.to_base64_no_pad();
//! ```

pub mod error;

use base64::{engine::general_purpose, Engine};

type Bytes = [u8; 16];
// Base64 implementation of UUID
#[derive(Debug, Clone, Copy)]
pub struct MiniUuid(Bytes);

impl MiniUuid {
  pub fn new() -> Self {
    #[cfg(feature = "v4")]
    Self(uuid::Uuid::new_v4().into_bytes())
  }

  #[inline]
  pub const fn into_bytes(self) -> Bytes {
    self.0
  }

  pub fn from_uuid(uuid: uuid::Uuid) -> Self {
    Self(uuid.into_bytes())
  }

  pub fn from_uuid_str(uuid: &str) -> Result<Self, uuid::Error> {
    let uuid = uuid::Uuid::parse_str(uuid)?;
    Ok(Self(uuid.into_bytes()))
  }

  fn decode(bytes: Bytes) -> error::Result<Self> {
    let string = hex::encode(bytes);
    let uuid = uuid::Uuid::parse_str(&string)?;

    Ok(Self(uuid.into_bytes()))
  }

  pub fn from_url_base64(uuid: &str) -> error::Result<Self> {
    let mut bytes: Bytes = [0; 16];
    general_purpose::URL_SAFE.decode_slice(uuid.as_bytes(), &mut bytes)?;

    Self::decode(bytes)
  }

  pub fn from_url_base64_no_pad(uuid: &str) -> error::Result<Self> {
    let mut bytes: Bytes = [0; 16];
    general_purpose::URL_SAFE_NO_PAD.decode_slice(uuid.as_bytes(), &mut bytes)?;

    Self::decode(bytes)
  }

  pub fn from_base64(uuid: &str) -> error::Result<Self> {
    let mut bytes: Bytes = [0; 16];
    general_purpose::STANDARD.decode_slice(uuid.as_bytes(), &mut bytes)?;

    Self::decode(bytes)
  }

  pub fn from_base64_no_pad(uuid: &str) -> error::Result<Self> {
    let mut bytes: Bytes = [0; 16];
    general_purpose::STANDARD_NO_PAD.decode_slice(uuid.as_bytes(), &mut bytes)?;

    Self::decode(bytes)
  }

  pub fn from_string(uuid: &str) -> error::Result<Self> {
    #[cfg(feature = "url_safe")]
    #[cfg(feature = "padded")]
    return Self::from_url_base64(uuid);

    #[cfg(feature = "url_safe")]
    #[cfg(not(feature = "padded"))]
    return Self::from_url_base64_no_pad(uuid);

    #[cfg(not(feature = "url_safe"))]
    #[cfg(feature = "padded")]
    return Self::from_base64(uuid);

    #[cfg(not(feature = "url_safe"))]
    #[cfg(not(feature = "padded"))]
    return Self::from_base64_no_pad(uuid);
  }

  pub fn to_uuid(&self) -> uuid::Uuid {
    uuid::Uuid::from_bytes(self.0)
  }

  pub fn to_url_base64(&self) -> String {
    general_purpose::URL_SAFE.encode(&self.0)
  }

  pub fn to_url_base64_no_pad(&self) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(&self.0)
  }

  pub fn to_base64(&self) -> String {
    general_purpose::STANDARD.encode(&self.0)
  }

  pub fn to_base64_no_pad(&self) -> String {
    general_purpose::STANDARD_NO_PAD.encode(&self.0)
  }
}

impl ToString for MiniUuid {
  fn to_string(&self) -> String {
    #[cfg(feature = "url_safe")]
    #[cfg(feature = "padded")]
    return self.to_url_base64();

    #[cfg(feature = "url_safe")]
    #[cfg(not(feature = "padded"))]
    return self.to_url_base64_no_pad();

    #[cfg(not(feature = "url_safe"))]
    #[cfg(feature = "padded")]
    return self.to_base64();

    #[cfg(not(feature = "url_safe"))]
    #[cfg(not(feature = "padded"))]
    return self.to_base64_no_pad();
  }
}

impl From<uuid::Uuid> for MiniUuid {
  fn from(uuid: uuid::Uuid) -> Self {
    Self::from_uuid(uuid)
  }
}

impl Into<uuid::Uuid> for MiniUuid {
  fn into(self) -> uuid::Uuid {
    uuid::Uuid::from_bytes(self.0)
  }
}
