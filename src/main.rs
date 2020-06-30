mod sha256;
mod tea;

fn main() {
    let mut module = tea::TeaModule::new();
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);
    let encrypted = module.encrypt(&message, &key);
    let decrypted = module.decrypt(&encrypted, &key);
    println!("Tea encryption of {} with key {} is {}", message, key, encrypted);
    println!("Tea decryption of {} with key {} is {}", encrypted, key, decrypted);

    let mut module = sha256::SHA256Module::new();
    module.init();
    let data = &[0xde, 0xad, 0xbe, 0xef, 0xbe, 0xef, 0xf0, 0x0d];
    module.update(data);
    let hash = module.finalize();
    assert_eq!(hash.as_u8_slice().len(), 32);
    println!("SHA-256 hash of deadbeef_beeff00d is {}", hash);
}

#[test]
fn tea() {
    // test round-tripping with tea
    let mut module = tea::TeaModule::new();
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);
    let encrypted = module.encrypt(&message, &key);
    let decrypted = module.decrypt(&encrypted, &key);
    assert_eq!(message, decrypted);
}

#[test]
fn sha256() {
    // test SHA-256 hash of 64 bytes of data
    let mut module = sha256::SHA256Module::new();
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
