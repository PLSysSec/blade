use crate::blade_setting::BladeSetting;

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
    pub fn new(blade_setting: BladeSetting) -> Self {
        Self {
            so: {
                let soname = match blade_setting {
                    BladeSetting::None => "wasm_obj/tea_ref.so",
                    BladeSetting::Lfence => "wasm_obj/tea_lfence.so",
                    BladeSetting::LfencePerBlock => "wasm_obj/tea_lfence_per_block.so",
                    BladeSetting::SLH => "wasm_obj/tea_slh.so",
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
