use crate::blade_setting::BladeType;
use crate::module::get_lucet_module;

use lucet_runtime::InstanceHandle;

pub struct Salsa20Module {
    so: InstanceHandle,
}

impl Salsa20Module {
    pub fn new(blade_type: BladeType, blade_v1_1: bool) -> Self {
        Self {
            so: get_lucet_module("wasm_obj/salsa20", blade_type, blade_v1_1),
        }
    }

    pub fn run(&mut self) {
        let _ = self.so.run("salsa20", &[]).unwrap();
    }
}
