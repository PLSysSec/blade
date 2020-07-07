pub mod blade_setting;
pub mod hacl_chacha20;
pub mod hacl_curve25519_51;
pub mod hacl_poly1305_32;
pub mod salsa20;
pub mod sha256;
pub mod tea;

mod module;
pub use module::BladeModule;

#[test]
fn tea() {
    // test round-tripping with tea
    let mut module = tea::TeaModule::new(blade_setting::BladeType::None, false);
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);
    let encrypted = module.encrypt(&message, &key);
    let decrypted = module.decrypt(&encrypted, &key);
    assert_eq!(message, decrypted);
}

#[test]
fn sha256() {
    // test SHA-256 hash of 64 bytes of data
    let mut module = sha256::SHA256Module::new(blade_setting::BladeType::None, false);
    module.init();
    let data = &[
        0xde, 0xad, 0xbe, 0xef, 0xbe, 0xef, 0xf0, 0x0d,
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
        0x0f, 0xed, 0xcb, 0xa9, 0x87, 0x65, 0x43, 0x21,
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
        0x0f, 0xed, 0xcb, 0xa9, 0x87, 0x65, 0x43, 0x21,
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
        0x0f, 0xed, 0xcb, 0xa9, 0x87, 0x65, 0x43, 0x21,
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    ];
    assert_eq!(data.len(), 64);
    module.update(data);
    let hash = module.finalize();
    assert_eq!(&hash.as_u8_slice(), &hmac_sha256::Hash::hash(data));
}

#[test]
fn hacl_chacha20() {
    // test round-tripping with Hacl Chacha20
    let mut module = hacl_chacha20::Chacha20Module::new(blade_setting::BladeType::None, false);
    let key = hacl_chacha20::Chacha20Key::new([
        11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 122, 133, 144, 155, 166, 177,
        188, 199, 211, 222, 233, 244, 255, 0, 10, 20, 30, 40, 50, 60, 70, 80,
    ]);
    let nonce = hacl_chacha20::Chacha20Nonce::new([
        98, 76, 54, 32, 10, 0, 2, 4, 6, 8, 10, 12,
    ]);
    let message = &[
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250,
    ];
    let encrypted = module.encrypt(&key, &nonce, message);
    let decrypted = module.decrypt(&key, &nonce, &encrypted);
    assert_eq!(message, decrypted.as_slice());
}
