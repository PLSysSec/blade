use blade_benchmarks::{sha256, tea};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn tea_encrypt(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module = tea::TeaModule::new();
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);

    c.bench_function("tea encrypt", |b| b.iter(|| {
        module.encrypt(black_box(&message), black_box(&key));
    }));
}

pub fn tea_decrypt(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module = tea::TeaModule::new();
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);

    c.bench_function("tea decrypt", |b| b.iter(|| {
        module.decrypt(black_box(&message), black_box(&key));
    }));
}

pub fn sha256_of_64bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module = sha256::SHA256Module::new();
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

    c.bench_function("sha256 of 64 bytes", |b| b.iter(|| {
        module.init();
        module.update(data);
        module.finalize();
    }));
}

pub fn sha256_of_1024bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module = sha256::SHA256Module::new();
    let data = {
        let mut data = vec![];
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
        data
    };

    c.bench_function("sha256 of 1024 bytes", |b| b.iter(|| {
        module.init();
        module.update(&data);
        module.finalize();
    }));
}

criterion_group!(tea, tea_encrypt, tea_decrypt);
criterion_group!(sha256, sha256_of_64bytes, sha256_of_1024bytes);
criterion_main!(tea, sha256);
