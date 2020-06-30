mod tea;

fn main() {
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);
    let encrypted = tea::tea_encrypt(&message, &key);
    let decrypted = tea::tea_decrypt(&encrypted, &key);
    println!("Encryption of {} with key {} is {}", message, key, encrypted);
    println!("Decryption of {} with key {} is {}", encrypted, key, decrypted);
}
