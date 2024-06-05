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
