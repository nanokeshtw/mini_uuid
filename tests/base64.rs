use mini_uuid::{error, MiniUuid};

#[test]
fn test_from_uuid_padded() {
    let uuid = uuid::Uuid::parse_str("fc3ceb1c-5ffd-4596-806a-595ee34acd1f").unwrap();
    let mini_uuid = MiniUuid::from(uuid);
    assert_eq!(mini_uuid.to_base64(), "/DzrHF/9RZaAalle40rNHw==");
}

#[test]
fn test_from_uuid_no_pad() {
    let uuid = uuid::Uuid::parse_str("fc3ceb1c-5ffd-4596-806a-595ee34acd1f").unwrap();
    let mini_uuid = MiniUuid::from(uuid);
    assert_eq!(mini_uuid.to_base64_no_pad(), "/DzrHF/9RZaAalle40rNHw");
}

#[test]
fn test_from_base64_padded() {
    let mini_uuid = MiniUuid::from_base64("/DzrHF/9RZaAalle40rNHw==").unwrap();
    assert_eq!(mini_uuid.to_base64(), "/DzrHF/9RZaAalle40rNHw==");
}

#[test]
fn test_from_base64_no_pad() {
    let mini_uuid = MiniUuid::from_base64_no_pad("/DzrHF/9RZaAalle40rNHw").unwrap();
    assert_eq!(mini_uuid.to_base64_no_pad(), "/DzrHF/9RZaAalle40rNHw");
}

#[test]
fn test_from_url_base64_padded() {
    let mini_uuid = MiniUuid::from_base64("_DzrHF_9RZaAalle40rNHw==").unwrap_err();
    assert_eq!(error::ErrorKind::InvalidBase64, mini_uuid.into())
}

#[test]
fn test_from_url_base64_no_pad() {
    let mini_uuid = MiniUuid::from_base64_no_pad("_DzrHF_9RZaAalle40rNHw").unwrap_err();
    assert_eq!(error::ErrorKind::InvalidBase64, mini_uuid.into())
}
