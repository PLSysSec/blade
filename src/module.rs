use crate::blade_setting::BladeType;

use lucet_runtime::{DlModule, InstanceHandle, Limits, MmapRegion, Region};
use std::path::{Path, PathBuf};

pub fn get_lucet_module(sodir: impl AsRef<Path>, blade_type: BladeType, blade_v1_1: bool) -> InstanceHandle {
    let sodir = sodir.as_ref();
    let soname: PathBuf = match (blade_type, blade_v1_1) {
        (BladeType::None, _) => sodir.join("ref.so"),
        (BladeType::BaselineFence, true) => sodir.join("baseline_fence_with_v1_1.so"),
        (BladeType::BaselineFence, false) => sodir.join("baseline_fence_no_v1_1.so"),
        (BladeType::BaselineSLH, true) => sodir.join("baseline_slh_with_v1_1.so"),
        (BladeType::BaselineSLH, false) => sodir.join("baseline_slh_no_v1_1.so"),
        (BladeType::Lfence, true) => sodir.join("lfence_with_v1_1.so"),
        (BladeType::Lfence, false) => sodir.join("lfence_no_v1_1.so"),
        (BladeType::LfencePerBlock, true) => sodir.join("lfence_per_block_with_v1_1.so"),
        (BladeType::LfencePerBlock, false) => sodir.join("lfence_per_block_no_v1_1.so"),
        (BladeType::SLH, true) => sodir.join("slh_with_v1_1.so"),
        (BladeType::SLH, false) => sodir.join("slh_no_v1_1.so"),
    };
    let module = DlModule::load(&soname).unwrap();
    let region = MmapRegion::create(1, &Limits::default()).unwrap();
    region.new_instance(module).unwrap()
}

pub trait BladeModule: Sized {
    fn new(blade_type: BladeType, blade_v1_1: bool) -> Self;
}
