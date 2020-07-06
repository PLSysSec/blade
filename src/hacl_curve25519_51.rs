use crate::blade_setting::BladeType;
use crate::module::{get_lucet_module, BladeModule};

use lucet_runtime::InstanceHandle;

const KEY_BYTES: usize = 32;
const OUTPUT_BYTES: usize = 32;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Curve25519Key {
    key: [u8; KEY_BYTES],
}

impl Curve25519Key {
    pub fn new(data: [u8; KEY_BYTES]) -> Self {
        Self {
            key: data,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Curve25519Output {
    out: [u8; OUTPUT_BYTES],
}

impl Curve25519Output {
    pub fn new(data: [u8; OUTPUT_BYTES]) -> Self {
        Self {
            out: data,
        }
    }

    pub fn as_u8_slice(&self) -> &[u8] {
        &self.out
    }
}

pub struct Curve25519Module {
    so: InstanceHandle,
}

impl BladeModule for Curve25519Module {
    fn new(blade_type: BladeType, blade_v1_1: bool) -> Self {
        Self {
            so: get_lucet_module("wasm_obj/Hacl_Curve25519_51", blade_type, blade_v1_1),
        }
    }
}

impl Curve25519Module {
    pub fn ecdh(&mut self, privkey: &Curve25519Key, pubkey: &Curve25519Key) -> Curve25519Output {
        // allocation
        let mut heap_base = unsafe {
            self.so.globals()[0].i_32 as u32 // seems like global 0 is the heap base?
        };
        let privkey_ptr = heap_base;
        heap_base += KEY_BYTES as u32;
        let pubkey_ptr = heap_base;
        heap_base += KEY_BYTES as u32;
        let out_ptr = heap_base;

        // set up inputs
        let heap = self.so.heap_mut();
        let privkey_heap_idx = privkey_ptr as usize;
        for i in 0 .. KEY_BYTES {
            heap[privkey_heap_idx + i] = privkey.key[i];
        }
        let pubkey_heap_idx = pubkey_ptr as usize;
        for i in 0 .. KEY_BYTES {
            heap[pubkey_heap_idx + i] = pubkey.key[i];
        }

        // call wasm
        let _ = self.so.run("Hacl_Curve25519_51_ecdh", &[
            out_ptr.into(),
            privkey_ptr.into(),
            pubkey_ptr.into(),
        ]).unwrap();
        Curve25519Output {
            out: {
                let heap = self.so.heap();
                let out_heap_idx = out_ptr as usize;
                [
                    heap[out_heap_idx],
                    heap[out_heap_idx+1],
                    heap[out_heap_idx+2],
                    heap[out_heap_idx+3],
                    heap[out_heap_idx+4],
                    heap[out_heap_idx+5],
                    heap[out_heap_idx+6],
                    heap[out_heap_idx+7],
                    heap[out_heap_idx+8],
                    heap[out_heap_idx+9],
                    heap[out_heap_idx+10],
                    heap[out_heap_idx+11],
                    heap[out_heap_idx+12],
                    heap[out_heap_idx+13],
                    heap[out_heap_idx+14],
                    heap[out_heap_idx+15],
                    heap[out_heap_idx+16],
                    heap[out_heap_idx+17],
                    heap[out_heap_idx+18],
                    heap[out_heap_idx+19],
                    heap[out_heap_idx+20],
                    heap[out_heap_idx+21],
                    heap[out_heap_idx+22],
                    heap[out_heap_idx+23],
                    heap[out_heap_idx+24],
                    heap[out_heap_idx+25],
                    heap[out_heap_idx+26],
                    heap[out_heap_idx+27],
                    heap[out_heap_idx+28],
                    heap[out_heap_idx+29],
                    heap[out_heap_idx+30],
                    heap[out_heap_idx+31],
                ]
            }
        }
    }
}
