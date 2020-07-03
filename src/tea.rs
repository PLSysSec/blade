use crate::blade_setting::BladeType;

use std::fmt;

use lucet_runtime::{DlModule, InstanceHandle, Limits, MmapRegion, Region};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TeaKey {
    key: [u32; 4],
}

impl TeaKey {
    pub fn new(data: [u32; 4]) -> Self {
        Self {
            key: data,
        }
    }
}

impl fmt::Display for TeaKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}_{:x}_{:x}_{:x}", self.key[0], self.key[1], self.key[2], self.key[3])
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TeaMsg {
    msg: [u32; 2],
}

impl TeaMsg {
    pub fn new(data: [u32; 2]) -> Self {
        Self {
            msg: data,
        }
    }
}

impl fmt::Display for TeaMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}_{:x}", &self.msg[0], &self.msg[1])
    }
}

pub struct TeaModule {
    so: InstanceHandle,
}

impl TeaModule {
    pub fn new(blade_type: BladeType, blade_v1_1: bool) -> Self {
        Self {
            so: {
                let soname = match (blade_type, blade_v1_1) {
                    (BladeType::None, _) => "wasm_obj/tea_ref.so",
                    (BladeType::Baseline, true) => "wasm_obj/tea_baseline_with_v1_1.so",
                    (BladeType::Baseline, false) => "wasm_obj/tea_baseline_no_v1_1.so",
                    (BladeType::Lfence, true) => "wasm_obj/tea_lfence_with_v1_1.so",
                    (BladeType::Lfence, false) => "wasm_obj/tea_lfence_no_v1_1.so",
                    (BladeType::LfencePerBlock, true) => "wasm_obj/tea_lfence_per_block_with_v1_1.so",
                    (BladeType::LfencePerBlock, false) => "wasm_obj/tea_lfence_per_block_no_v1_1.so",
                    (BladeType::SLH, true) => "wasm_obj/tea_slh_with_v1_1.so",
                    (BladeType::SLH, false) => "wasm_obj/tea_slh_no_v1_1.so",
                };
                let module = DlModule::load(soname).unwrap();
                let region = MmapRegion::create(1, &Limits::default()).unwrap();
                region.new_instance(module).unwrap()
            }
        }
    }

    pub fn encrypt(&mut self, msg: &TeaMsg, key: &TeaKey) -> TeaMsg {
        let heap = self.so.heap_u32_mut();
        // the wasm function expects the msg as bytes 0-7 on the wasm heap, and key as bytes 8-23
        heap[0] = msg.msg[0];
        heap[1] = msg.msg[1];
        heap[2] = key.key[0];
        heap[3] = key.key[1];
        heap[4] = key.key[2];
        heap[5] = key.key[3];
        let _ = self.so.run("encrypt", &[]).unwrap();
        TeaMsg {
            // the wasm function leaves the return values in bytes 0-7 of the heap
            msg: {
                let heap = self.so.heap_u32();
                [heap[0], heap[1]]
            }
        }
    }

    pub fn decrypt(&mut self, msg: &TeaMsg, key: &TeaKey) -> TeaMsg {
        let heap = self.so.heap_u32_mut();
        // the wasm function expects the msg as bytes 0-7 on the wasm heap, and key as bytes 8-23
        heap[0] = msg.msg[0];
        heap[1] = msg.msg[1];
        heap[2] = key.key[0];
        heap[3] = key.key[1];
        heap[4] = key.key[2];
        heap[5] = key.key[3];
        let _ = self.so.run("decrypt", &[]).unwrap();
        TeaMsg {
            // the wasm function leaves the return values in bytes 0-7 of the heap
            msg: {
                let heap = self.so.heap_u32();
                [heap[0], heap[1]]
            }
        }
    }
}
