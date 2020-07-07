use blade_benchmarks::{hacl_chacha20, hacl_curve25519_51, hacl_poly1305_32, salsa20, sha256, tea, blade_setting::BladeType, BladeModule};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

struct Modules<T> {
    reference: T,
    baseline_with_v1_1: T,
    baseline_no_v1_1: T,
    lfence_with_v1_1: T,
    lfence_no_v1_1: T,
    //lfence_per_block_with_v1_1: T,
    //lfence_per_block_no_v1_1: T,
    slh_with_v1_1: T,
    slh_no_v1_1: T,
}

impl<T: BladeModule> Modules<T> {
    fn new() -> Self {
        Self {
            reference: T::new(BladeType::None, false),
            baseline_with_v1_1: T::new(BladeType::Baseline, true),
            baseline_no_v1_1: T::new(BladeType::Baseline, false),
            lfence_with_v1_1: T::new(BladeType::Lfence, true),
            lfence_no_v1_1: T::new(BladeType::Lfence, false),
            //lfence_per_block_with_v1_1: T::new(BladeType::LfencePerBlock, true),
            //lfence_per_block_no_v1_1: T::new(BladeType::LfencePerBlock, false),
            slh_with_v1_1: T::new(BladeType::SLH, true),
            slh_no_v1_1: T::new(BladeType::SLH, false),
        }
    }

    // Bench all the modules in this `Modules` with the given closure
    fn bench_all(&mut self, c: &mut Criterion, group_name: &str, f: impl Fn(&mut T)) {
        let mut group = c.benchmark_group(group_name);
        group.bench_function("Ref", |b| b.iter(|| {
            f(&mut self.reference);
        }));
        group.bench_function("Baseline with v1.1", |b| b.iter(|| {
            f(&mut self.baseline_with_v1_1);
        }));
        group.bench_function("Baseline no v1.1", |b| b.iter(|| {
            f(&mut self.baseline_no_v1_1);
        }));
        group.bench_function("Lfence with v1.1", |b| b.iter(|| {
            f(&mut self.lfence_with_v1_1);
        }));
        group.bench_function("Lfence no v1.1", |b| b.iter(|| {
            f(&mut self.lfence_no_v1_1);
        }));
        /*
        group.bench_function("LfencePerBlock with v1.1", |b| b.iter(|| {
            f(&mut self.lfence_per_block_with_v1_1);
        }));
        group.bench_function("LfencePerBlock no v1.1", |b| b.iter(|| {
            f(&mut self.lfence_per_block_no_v1_1);
        }));
        */
        group.bench_function("SLH with v1.1", |b| b.iter(|| {
            f(&mut self.slh_with_v1_1);
        }));
        group.bench_function("SLH no v1.1", |b| b.iter(|| {
            f(&mut self.slh_no_v1_1);
        }));
    }
}

pub fn tea_encrypt(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<tea::TeaModule>::new();
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);

    modules.bench_all(c, "tea encrypt", |m| {
        m.encrypt(black_box(&message), black_box(&key));
    });
}

pub fn tea_decrypt(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<tea::TeaModule>::new();
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);

    modules.bench_all(c, "tea decrypt", |m| {
        m.decrypt(black_box(&message), black_box(&key));
    });
}

pub fn sha256_of_64bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<sha256::SHA256Module>::new();
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

    modules.bench_all(c, "sha256 of 64 bytes", |m| {
        m.init();
        m.update(data);
        m.finalize();
    });
}

pub fn sha256_of_1024bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<sha256::SHA256Module>::new();
    let data = get_some_bytes(1024);

    modules.bench_all(c, "sha256 of 1024 bytes", |m| {
        m.init();
        m.update(&data);
        m.finalize();
    });
}

pub fn salsa20_run(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<salsa20::Salsa20Module>::new();
    modules.bench_all(c, "salsa20", |m| {
        m.run();
    });
}

pub fn chacha20_encrypt_8192_bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<hacl_chacha20::Chacha20Module>::new();
    let key = hacl_chacha20::Chacha20Key::new([
        11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 122, 133, 144, 155, 166, 177,
        188, 199, 211, 222, 233, 244, 255, 0, 10, 20, 30, 40, 50, 60, 70, 80,
    ]);
    let nonce = hacl_chacha20::Chacha20Nonce::new([
        98, 76, 54, 32, 10, 0, 2, 4,
    ]);
    let msg = get_some_bytes(8192);
    modules.bench_all(c, "chacha20 of 8192 bytes", |m| {
        m.encrypt(&key, &nonce, &msg);
    })
}

pub fn poly1305_mac_of_1024bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<hacl_poly1305_32::Poly1305Module>::new();
    let key = hacl_poly1305_32::Poly1305Key::new(
        [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    );
    let msg = get_some_bytes(1024);
    modules.bench_all(c, "poly1305 of 1024 bytes", |m| {
        m.mac(&key, &msg);
    })
}

pub fn poly1305_mac_of_8192bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<hacl_poly1305_32::Poly1305Module>::new();
    let key = hacl_poly1305_32::Poly1305Key::new(
        [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    );
    let msg = get_some_bytes(8192);
    modules.bench_all(c, "poly1305 of 8192 bytes", |m| {
        m.mac(&key, &msg);
    })
}

pub fn curve25519_51_ecdh(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<hacl_curve25519_51::Curve25519Module>::new();
    let pubkey = hacl_curve25519_51::Curve25519Key::new([
        0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30,
        201, 203, 205, 207, 209, 211, 213, 215, 217, 219, 221, 223, 225, 227, 229, 231,
    ]);
    let privkey = hacl_curve25519_51::Curve25519Key::new([
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
        100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
    ]);
    modules.bench_all(c, "curve25519_51", |m| {
        m.ecdh(&pubkey, &privkey);
    })
}

pub fn get_some_bytes(howmany: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(howmany);
    assert_eq!(howmany % 8, 0, "this function expects a multiple of 8 bytes");
    let full_1024s = howmany / 1024;
    let leftover_bytes = howmany % 1024;
    for _ in 0 .. full_1024s {
        for i in 0 .. 128 {
            data.push(0xfa - i);
            data.push(0xce - i);
            data.push(0x1f + i);
            data.push(0x31 + i);
            data.push(0x78 + (i/2));
            data.push(0x04 + (3*i/2));
            data.push(0xaa - (i/2));
            data.push(0x32 + i);
        }
    }
    for i in 0 .. (leftover_bytes/8) as u8 {
        data.push(0xfa - i);
        data.push(0xce - i);
        data.push(0x1f + i);
        data.push(0x31 + i);
        data.push(0x78 + (i/2));
        data.push(0x04 + (3*i/2));
        data.push(0xaa - (i/2));
        data.push(0x32 + i);
    }
    data
}

criterion_group!(tea, tea_encrypt, tea_decrypt);
criterion_group!(sha256, sha256_of_64bytes, sha256_of_1024bytes);
criterion_group!(salsa20, salsa20_run);
criterion_group!(chacha20, chacha20_encrypt_8192_bytes);
criterion_group!(poly1305, poly1305_mac_of_1024bytes, poly1305_mac_of_8192bytes);
criterion_group!(curve25519_51, curve25519_51_ecdh);
criterion_main!(tea, sha256, salsa20, chacha20, poly1305, curve25519_51);
