use blade_benchmarks::{sha256, tea, blade_setting::BladeSetting};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn tea_encrypt(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module_ref = tea::TeaModule::new(BladeSetting::None);
    let mut module_lfence = tea::TeaModule::new(BladeSetting::Lfence);
    //let mut module_lfence_per_block = tea::TeaModule::new(BladeSetting::LfencePerBlock);
    let mut module_slh_with_1_1 = tea::TeaModule::new(BladeSetting::SLHWith11);
    let mut module_slh_no_1_1 = tea::TeaModule::new(BladeSetting::SLHNo11);
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);

    let mut group = c.benchmark_group("tea encrypt");
    group.bench_function("Ref", |b| b.iter(|| {
        module_ref.encrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Lfence", |b| b.iter(|| {
        module_lfence.encrypt(black_box(&message), black_box(&key));
    }));
    /*
    group.bench_function("LfencePerBlock", |b| b.iter(|| {
        module_lfence_per_block.encrypt(black_box(&message), black_box(&key));
    }));
    */
    group.bench_function("SLH with 1.1", |b| b.iter(|| {
        module_slh_with_1_1.encrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("SLH no 1.1", |b| b.iter(|| {
        module_slh_no_1_1.encrypt(black_box(&message), black_box(&key));
    }));
}

pub fn tea_decrypt(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module_ref = tea::TeaModule::new(BladeSetting::None);
    let mut module_lfence = tea::TeaModule::new(BladeSetting::Lfence);
    //let mut module_lfence_per_block = tea::TeaModule::new(BladeSetting::LfencePerBlock);
    let mut module_slh_with_1_1 = tea::TeaModule::new(BladeSetting::SLHWith11);
    let mut module_slh_no_1_1 = tea::TeaModule::new(BladeSetting::SLHNo11);
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);

    let mut group = c.benchmark_group("tea decrypt");
    group.bench_function("Ref", |b| b.iter(|| {
        module_ref.decrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Lfence", |b| b.iter(|| {
        module_lfence.decrypt(black_box(&message), black_box(&key));
    }));
    /*
    group.bench_function("LfencePerBlock", |b| b.iter(|| {
        module_lfence_per_block.decrypt(black_box(&message), black_box(&key));
    }));
    */
    group.bench_function("SLH with 1.1", |b| b.iter(|| {
        module_slh_with_1_1.encrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("SLH no 1.1", |b| b.iter(|| {
        module_slh_no_1_1.encrypt(black_box(&message), black_box(&key));
    }));
}

pub fn sha256_of_64bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module_ref = sha256::SHA256Module::new(BladeSetting::None);
    let mut module_lfence = sha256::SHA256Module::new(BladeSetting::Lfence);
    //let mut module_lfence_per_block = sha256::SHA256Module::new(BladeSetting::LfencePerBlock);
    let mut module_slh_with_1_1 = sha256::SHA256Module::new(BladeSetting::SLHWith11);
    let mut module_slh_no_1_1 = sha256::SHA256Module::new(BladeSetting::SLHNo11);
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

    let mut group = c.benchmark_group("sha256 of 64 bytes");
    group.bench_function("Ref", |b| b.iter(|| {
        module_ref.init();
        module_ref.update(data);
        module_ref.finalize();
    }));
    group.bench_function("Lfence", |b| b.iter(|| {
        module_lfence.init();
        module_lfence.update(data);
        module_lfence.finalize();
    }));
    /*
    group.bench_function("LfencePerBlock", |b| b.iter(|| {
        module_lfence_per_block.init();
        module_lfence_per_block.update(data);
        module_lfence_per_block.finalize();
    }));
    */
    group.bench_function("SLH with 1.1", |b| b.iter(|| {
        module_slh_with_1_1.init();
        module_slh_with_1_1.update(data);
        module_slh_with_1_1.finalize();
    }));
    group.bench_function("SLH no 1.1", |b| b.iter(|| {
        module_slh_no_1_1.init();
        module_slh_no_1_1.update(data);
        module_slh_no_1_1.finalize();
    }));
}

pub fn sha256_of_1024bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut module_ref = sha256::SHA256Module::new(BladeSetting::None);
    let mut module_lfence = sha256::SHA256Module::new(BladeSetting::Lfence);
    //let mut module_lfence_per_block = sha256::SHA256Module::new(BladeSetting::LfencePerBlock);
    let mut module_slh_with_1_1 = sha256::SHA256Module::new(BladeSetting::SLHWith11);
    let mut module_slh_no_1_1 = sha256::SHA256Module::new(BladeSetting::SLHNo11);
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

    let mut group = c.benchmark_group("sha256 of 1024 bytes");
    group.bench_function("Ref", |b| b.iter(|| {
        module_ref.init();
        module_ref.update(&data);
        module_ref.finalize();
    }));
    group.bench_function("Lfence", |b| b.iter(|| {
        module_lfence.init();
        module_lfence.update(&data);
        module_lfence.finalize();
    }));
    /*
    group.bench_function("LfencePerBlock", |b| b.iter(|| {
        module_lfence_per_block.init();
        module_lfence_per_block.update(&data);
        module_lfence_per_block.finalize();
    }));
    */
    group.bench_function("SLH with 1.1", |b| b.iter(|| {
        module_slh_with_1_1.init();
        module_slh_with_1_1.update(&data);
        module_slh_with_1_1.finalize();
    }));
    group.bench_function("SLH no 1.1", |b| b.iter(|| {
        module_slh_no_1_1.init();
        module_slh_no_1_1.update(&data);
        module_slh_no_1_1.finalize();
    }));
}

criterion_group!(tea, tea_encrypt, tea_decrypt);
criterion_group!(sha256, sha256_of_64bytes, sha256_of_1024bytes);
criterion_main!(tea, sha256);
