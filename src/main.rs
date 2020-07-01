use blade_benchmarks::{sha256, tea};

fn main() {
    lucet_runtime::lucet_internal_ensure_linked();

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
