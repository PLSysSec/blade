use blade_benchmarks::{hacl_curve25519_51, hacl_poly1305_32, hacl_chacha20, sha256, tea, blade_setting::BladeType, BladeModule};

fn main() {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module = tea::TeaModule::new(BladeType::None, false);
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);
    let encrypted = module.encrypt(&message, &key);
    let decrypted = module.decrypt(&encrypted, &key);
    println!("Tea encryption of {} with key {} is {}", message, key, encrypted);
    println!("Tea decryption of {} with key {} is {}", encrypted, key, decrypted);

    // example from https://www.cryptopp.com/wiki/ChaCha20#Encryption_and_Decryption
    // so that I could debug the target ciphertext
    let mut module = hacl_chacha20::Chacha20Module::new(BladeType::None, false);
    let message = String::from("My Plaintext!! My Dear plaintext!!").into_bytes();
    let key = hacl_chacha20::Chacha20Key::new([
        0xF2, 0x1C, 0xD8, 0x58, 0x3F, 0x95, 0x18, 0x08,
        0xA0, 0x1C, 0x16, 0x96, 0x3A, 0xC4, 0xAD, 0x23,
        0xFC, 0x35, 0x66, 0x25, 0xD9, 0xBA, 0xCE, 0x17,
        0x82, 0x5F, 0xDE, 0xC3, 0xBB, 0xA1, 0xA9, 0x32,
    ]);
    let nonce = hacl_chacha20::Chacha20Nonce::new([
        0, 0, 0, 0,
        0x7B, 0xAD, 0x60, 0x60, 0x55, 0x18, 0xA6, 0x81, 
    ]);
    let encrypted = module.encrypt(&key, &nonce, &message);
    let decrypted = module.decrypt(&key, &nonce, &encrypted);
    println!("Chacha20 encryption of \"{}\" with key {} and nonce {} is {}", std::str::from_utf8(&message).unwrap(), key, nonce, hex::encode(&encrypted));
    println!("Chacha20 encryption of \"{}\" with key {} and nonce {} is {}", hex::encode(&encrypted), key, nonce, std::str::from_utf8(&decrypted).unwrap());

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

    let mut module = hacl_curve25519_51::Curve25519Module::new(BladeType::None, false);
    let pubkey = hacl_curve25519_51::Curve25519Key::new([
        0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30,
        201, 203, 205, 207, 209, 211, 213, 215, 217, 219, 221, 223, 225, 227, 229, 231,
    ]);
    let privkey = hacl_curve25519_51::Curve25519Key::new([
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
        100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
    ]);
    let out = module.ecdh(&privkey, &pubkey);
    println!("First byte of the Curve25519 output for our keys is {}", out.as_u8_slice()[0]);
}
