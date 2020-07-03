use blade_benchmarks::{salsa20, sha256, tea, blade_setting::BladeType, BladeModule};
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
}

pub fn tea_encrypt(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<tea::TeaModule>::new();
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);

    let mut group = c.benchmark_group("tea encrypt");
    group.bench_function("Ref", |b| b.iter(|| {
        modules.reference.encrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Baseline with v1.1", |b| b.iter(|| {
        modules.baseline_with_v1_1.encrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Baseline no v1.1", |b| b.iter(|| {
        modules.baseline_no_v1_1.encrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Lfence with v1.1", |b| b.iter(|| {
        modules.lfence_with_v1_1.encrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Lfence no v1.1", |b| b.iter(|| {
        modules.lfence_no_v1_1.encrypt(black_box(&message), black_box(&key));
    }));
    /*
    group.bench_function("LfencePerBlock with v1.1", |b| b.iter(|| {
        modules.lfence_per_block_with_v1_1.encrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("LfencePerBlock no v1.1", |b| b.iter(|| {
        modules.lfence_per_block_no_v1_1.encrypt(black_box(&message), black_box(&key));
    }));
    */
    group.bench_function("SLH with 1.1", |b| b.iter(|| {
        modules.slh_with_v1_1.encrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("SLH no 1.1", |b| b.iter(|| {
        modules.slh_no_v1_1.encrypt(black_box(&message), black_box(&key));
    }));
}

pub fn tea_decrypt(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<tea::TeaModule>::new();
    let message = tea::TeaMsg::new([0xdeadbeef, 0xbeeff00d]);
    let key = tea::TeaKey::new([0xd34db33f, 0xb33ff33d, 0xf000ba12, 0xdeadf00d]);

    let mut group = c.benchmark_group("tea decrypt");
    group.bench_function("Ref", |b| b.iter(|| {
        modules.reference.decrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Baseline with v1.1", |b| b.iter(|| {
        modules.baseline_with_v1_1.decrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Baseline no v1.1", |b| b.iter(|| {
        modules.baseline_no_v1_1.decrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Lfence with v1.1", |b| b.iter(|| {
        modules.lfence_with_v1_1.decrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("Lfence no v1.1", |b| b.iter(|| {
        modules.lfence_no_v1_1.decrypt(black_box(&message), black_box(&key));
    }));
    /*
    group.bench_function("LfencePerBlock with v1.1", |b| b.iter(|| {
        modules.lfence_per_block_with_v1_1.decrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("LfencePerBlock no v1.1", |b| b.iter(|| {
        modules.lfence_per_block_no_v1_1.decrypt(black_box(&message), black_box(&key));
    }));
    */
    group.bench_function("SLH with 1.1", |b| b.iter(|| {
        modules.slh_with_v1_1.decrypt(black_box(&message), black_box(&key));
    }));
    group.bench_function("SLH no 1.1", |b| b.iter(|| {
        modules.slh_no_v1_1.decrypt(black_box(&message), black_box(&key));
    }));
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

    let mut group = c.benchmark_group("sha256 of 64 bytes");
    group.bench_function("Ref", |b| b.iter(|| {
        modules.reference.init();
        modules.reference.update(data);
        modules.reference.finalize();
    }));
    group.bench_function("Baseline with v1.1", |b| b.iter(|| {
        modules.baseline_with_v1_1.init();
        modules.baseline_with_v1_1.update(data);
        modules.baseline_with_v1_1.finalize();
    }));
    group.bench_function("Baseline no v1.1", |b| b.iter(|| {
        modules.baseline_no_v1_1.init();
        modules.baseline_no_v1_1.update(data);
        modules.baseline_no_v1_1.finalize();
    }));
    group.bench_function("Lfence with v1.1", |b| b.iter(|| {
        modules.lfence_with_v1_1.init();
        modules.lfence_with_v1_1.update(data);
        modules.lfence_with_v1_1.finalize();
    }));
    group.bench_function("Lfence no v1.1", |b| b.iter(|| {
        modules.lfence_no_v1_1.init();
        modules.lfence_no_v1_1.update(data);
        modules.lfence_no_v1_1.finalize();
    }));
    /*
    group.bench_function("LfencePerBlock with v1.1", |b| b.iter(|| {
        modules.lfence_per_block_with_v1_1.init();
        modules.lfence_per_block_with_v1_1.update(data);
        modules.lfence_per_block_with_v1_1.finalize();
    }));
    group.bench_function("LfencePerBlock no v1.1", |b| b.iter(|| {
        modules.lfence_per_block_no_v1_1.init();
        modules.lfence_per_block_no_v1_1.update(data);
        modules.lfence_per_block_no_v1_1.finalize();
    }));
    */
    group.bench_function("SLH with v1.1", |b| b.iter(|| {
        modules.slh_with_v1_1.init();
        modules.slh_with_v1_1.update(data);
        modules.slh_with_v1_1.finalize();
    }));
    group.bench_function("SLH no v1.1", |b| b.iter(|| {
        modules.slh_no_v1_1.init();
        modules.slh_no_v1_1.update(data);
        modules.slh_no_v1_1.finalize();
    }));
}

pub fn sha256_of_1024bytes(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<sha256::SHA256Module>::new();
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
        modules.reference.init();
        modules.reference.update(&data);
        modules.reference.finalize();
    }));
    group.bench_function("Baseline with v1.1", |b| b.iter(|| {
        modules.baseline_with_v1_1.init();
        modules.baseline_with_v1_1.update(&data);
        modules.baseline_with_v1_1.finalize();
    }));
    group.bench_function("Baseline no v1.1", |b| b.iter(|| {
        modules.baseline_no_v1_1.init();
        modules.baseline_no_v1_1.update(&data);
        modules.baseline_no_v1_1.finalize();
    }));
    group.bench_function("Lfence with v1.1", |b| b.iter(|| {
        modules.lfence_with_v1_1.init();
        modules.lfence_with_v1_1.update(&data);
        modules.lfence_with_v1_1.finalize();
    }));
    group.bench_function("Lfence no v1.1", |b| b.iter(|| {
        modules.lfence_no_v1_1.init();
        modules.lfence_no_v1_1.update(&data);
        modules.lfence_no_v1_1.finalize();
    }));
    /*
    group.bench_function("LfencePerBlock with v1.1", |b| b.iter(|| {
        modules.lfence_per_block_with_v1_1.init();
        modules.lfence_per_block_with_v1_1.update(&data);
        modules.lfence_per_block_with_v1_1.finalize();
    }));
    group.bench_function("LfencePerBlock no v1.1", |b| b.iter(|| {
        modules.lfence_per_block_no_v1_1.init();
        modules.lfence_per_block_no_v1_1.update(&data);
        modules.lfence_per_block_no_v1_1.finalize();
    }));
    */
    group.bench_function("SLH with v1.1", |b| b.iter(|| {
        modules.slh_with_v1_1.init();
        modules.slh_with_v1_1.update(&data);
        modules.slh_with_v1_1.finalize();
    }));
    group.bench_function("SLH no v1.1", |b| b.iter(|| {
        modules.slh_no_v1_1.init();
        modules.slh_no_v1_1.update(&data);
        modules.slh_no_v1_1.finalize();
    }));
}

pub fn salsa20_run(c: &mut Criterion) {
    lucet_runtime::lucet_internal_ensure_linked();

    let mut modules = Modules::<salsa20::Salsa20Module>::new();
    let mut group = c.benchmark_group("salsa20");
    group.bench_function("Ref", |b| b.iter(|| {
        modules.reference.run();
    }));
    group.bench_function("Baseline with v1.1", |b| b.iter(|| {
        modules.baseline_with_v1_1.run();
    }));
    group.bench_function("Baseline no v1.1", |b| b.iter(|| {
        modules.baseline_no_v1_1.run();
    }));
    group.bench_function("Lfence with v1.1", |b| b.iter(|| {
        modules.lfence_with_v1_1.run();
    }));
    group.bench_function("Lfence no v1.1", |b| b.iter(|| {
        modules.lfence_no_v1_1.run();
    }));
    /*
    group.bench_function("LfencePerBlock with v1.1", |b| b.iter(|| {
        modules.lfence_per_block_with_v1_1.run();
    }));
    group.bench_function("LfencePerBlock no v1.1", |b| b.iter(|| {
        modules.lfence_per_block_no_v1_1.run();
    }));
    */
    group.bench_function("SLH with v1.1", |b| b.iter(|| {
        modules.slh_with_v1_1.run();
    }));
    group.bench_function("SLH no v1.1", |b| b.iter(|| {
        modules.slh_no_v1_1.run();
    }));
}

criterion_group!(tea, tea_encrypt, tea_decrypt);
criterion_group!(sha256, sha256_of_64bytes, sha256_of_1024bytes);
criterion_group!(salsa20, salsa20_run);
criterion_main!(tea, sha256, salsa20);
