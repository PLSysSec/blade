use crate::blade_setting::BladeType;

use std::fmt;

use lucet_runtime::{DlModule, InstanceHandle, Limits, MmapRegion, Region};

pub struct SHA256Module {
    so: InstanceHandle,
}

impl SHA256Module {
    pub fn new(blade_type: BladeType, blade_v1_1: bool) -> Self {
        Self {
            so: {
                let soname = match (blade_type, blade_v1_1) {
                    (BladeType::None, _) => "wasm_obj/sha256_ref.so",
                    (BladeType::Baseline, true) => "wasm_obj/sha256_baseline_with_v1_1.so",
                    (BladeType::Baseline, false) => "wasm_obj/sha256_baseline_no_v1_1.so",
                    (BladeType::Lfence, true) => "wasm_obj/sha256_lfence_with_v1_1.so",
                    (BladeType::Lfence, false) => "wasm_obj/sha256_lfence_no_v1_1.so",
                    (BladeType::LfencePerBlock, true) => "wasm_obj/sha256_lfence_per_block_with_v1_1.so",
                    (BladeType::LfencePerBlock, false) => "wasm_obj/sha256_lfence_per_block_no_v1_1.so",
                    (BladeType::SLH, true) => "wasm_obj/sha256_slh_with_v1_1.so",
                    (BladeType::SLH, false) => "wasm_obj/sha256_slh_no_v1_1.so",
                };
                let module = DlModule::load(soname).unwrap();
                let region = MmapRegion::create(1, &Limits::default()).unwrap();
                region.new_instance(module).unwrap()
            }
        }
    }

    /// Call this before starting, or call this to "refresh" the state to prepare
    /// for a new hash
    pub fn init(&mut self) {
        let _ = self.so.run("init", &[]).unwrap();
    }

    pub fn update(&mut self, data: &[u8]) {
        let heap = self.so.heap_mut();
        // the wasm function expects the input data starting at heap byte 652
        for (idx, &b) in data.iter().enumerate() {
            heap[652 + idx] = b;
        }
        let _ = self.so.run("update", &[data.len().into()]).unwrap();
    }

    pub fn finalize(&mut self) -> Hash {
        let _ = self.so.run("final", &[]).unwrap();
        // the wasm function leaves the hash value in bytes 620-651 of the heap
        let heap = self.so.heap();
        Hash {
            hash: (620 ..= 651).map(|idx| heap[idx]).collect(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Hash {
    /// a Vec with 32 elements (total 32 bytes or 256 bits)
    hash: Vec<u8>,
}

impl Hash {
    pub fn as_u8_slice(&self) -> &[u8] {
        &self.hash
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}{:x}{:x}{:x}_{:x}{:x}{:x}{:x}_{:x}{:x}{:x}{:x}_{:x}{:x}{:x}{:x}_{:x}{:x}{:x}{:x}_{:x}{:x}{:x}{:x}_{:x}{:x}{:x}{:x}_{:x}{:x}{:x}{:x}",
            &self.hash[0], &self.hash[1], &self.hash[2], &self.hash[3],
            &self.hash[4], &self.hash[5], &self.hash[6], &self.hash[7],
            &self.hash[8], &self.hash[9], &self.hash[10], &self.hash[11],
            &self.hash[12], &self.hash[13], &self.hash[14], &self.hash[15],
            &self.hash[16], &self.hash[17], &self.hash[18], &self.hash[19],
            &self.hash[20], &self.hash[21], &self.hash[22], &self.hash[23],
            &self.hash[24], &self.hash[25], &self.hash[26], &self.hash[27],
            &self.hash[28], &self.hash[29], &self.hash[30], &self.hash[31],
        )
    }
}
