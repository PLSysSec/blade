use crate::blade_setting::BladeType;

use lucet_runtime::{DlModule, InstanceHandle, Limits, MmapRegion, Region};

pub struct Salsa20Module {
    so: InstanceHandle,
}

impl Salsa20Module {
    pub fn new(blade_type: BladeType, blade_v1_1: bool) -> Self {
        Self {
            so: {
                let soname = match (blade_type, blade_v1_1) {
                    (BladeType::None, _) => "wasm_obj/salsa20_ref.so",
                    (BladeType::Baseline, true) => "wasm_obj/salsa20_baseline_with_v1_1.so",
                    (BladeType::Baseline, false) => "wasm_obj/salsa20_baseline_no_v1_1.so",
                    (BladeType::Lfence, true) => "wasm_obj/salsa20_lfence_with_v1_1.so",
                    (BladeType::Lfence, false) => "wasm_obj/salsa20_lfence_no_v1_1.so",
                    (BladeType::LfencePerBlock, true) => "wasm_obj/salsa20_lfence_per_block_with_v1_1.so",
                    (BladeType::LfencePerBlock, false) => "wasm_obj/salsa20_lfence_per_block_no_v1_1.so",
                    (BladeType::SLH, true) => "wasm_obj/salsa20_slh_with_v1_1.so",
                    (BladeType::SLH, false) => "wasm_obj/salsa20_slh_no_v1_1.so",
                };
                let module = DlModule::load(soname).unwrap();
                let region = MmapRegion::create(1, &Limits::default()).unwrap();
                region.new_instance(module).unwrap()
            }
        }
    }

    pub fn run(&mut self) {
        let _ = self.so.run("salsa20", &[]).unwrap();
    }
}
