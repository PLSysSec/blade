use blade_benchmarks::{hacl_poly1305_32, sha256, tea, blade_setting::BladeType, BladeModule};

fn main() {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module = tea::TeaModule::new(BladeType::None, false);
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);
    let encrypted = module.encrypt(&message, &key);
    let decrypted = module.decrypt(&encrypted, &key);
    println!("Tea encryption of {} with key {} is {}", message, key, encrypted);
    println!("Tea decryption of {} with key {} is {}", encrypted, key, decrypted);

    let mut module = sha256::SHA256Module::new(BladeType::None, false);
    module.init();
    let data = &[0xde, 0xad, 0xbe, 0xef, 0xbe, 0xef, 0xf0, 0x0d];
    module.update(data);
    let hash = module.finalize();
    assert_eq!(hash.as_u8_slice().len(), 32);
    println!("SHA-256 hash of deadbeef_beeff00d is {}", hash);

    let mut module = hacl_poly1305_32::Poly1305Module::new(BladeType::None, false);
    let message = &[55, 54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36];
    let key = hacl_poly1305_32::Poly1305Key::new([
        15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    ]);
    let tag = module.mac(&key, message);
    println!("First byte of the Poly1305 tag of our message is {}", tag.as_u8_slice()[0]);
}
