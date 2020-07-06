use crate::blade_setting::BladeType;
use crate::module::{get_lucet_module, BladeModule};

use lucet_runtime::InstanceHandle;

const KEY_BYTES: usize = 16;
const TAG_BYTES: usize = 16;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Poly1305Key {
    key: [u8; KEY_BYTES],
}

impl Poly1305Key {
    pub fn new(data: [u8; KEY_BYTES]) -> Self {
        Self {
            key: data,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Poly1305Tag {
    tag: [u8; TAG_BYTES],
}

impl Poly1305Tag {
    pub fn new(data: [u8; TAG_BYTES]) -> Self {
        Self {
            tag: data,
        }
    }

    pub fn as_u8_slice(&self) -> &[u8] {
        &self.tag
    }
}

pub struct Poly1305Module {
    so: InstanceHandle,
}

impl BladeModule for Poly1305Module {
    fn new(blade_type: BladeType, blade_v1_1: bool) -> Self {
        Self {
            so: get_lucet_module("wasm_obj/Hacl_Poly1305_32", blade_type, blade_v1_1),
        }
    }
}

impl Poly1305Module {
    pub fn mac(&mut self, key: &Poly1305Key, msg: &[u8]) -> Poly1305Tag {
        // allocation
        let mut heap_base = unsafe {
            self.so.globals()[0].i_32 as u32 // seems like global 0 is the heap base?
        };
        let key_ptr = heap_base;
        heap_base += KEY_BYTES as u32;
        let tag_ptr = heap_base;
        heap_base += TAG_BYTES as u32;
        let msg_ptr = heap_base;

        // set up inputs
        let heap = self.so.heap_mut();
        let key_heap_idx = key_ptr as usize;
        for i in 0 .. KEY_BYTES {
            heap[key_heap_idx + i] = key.key[i];
        }
        let msg_heap_idx = msg_ptr as usize;
        for i in 0 .. msg.len() {
            heap[msg_heap_idx + i] = msg[i];
        }

        // call wasm
        let _ = self.so.run("Hacl_Poly1305_32_poly1305_mac", &[
            tag_ptr.into(),
            msg.len().into(),
            msg_ptr.into(),
            key_ptr.into(),
        ]).unwrap();
        Poly1305Tag {
            tag: {
                let heap = self.so.heap();
                let tag_heap_idx = tag_ptr as usize;
                [
                    heap[tag_heap_idx],
                    heap[tag_heap_idx+1],
                    heap[tag_heap_idx+2],
                    heap[tag_heap_idx+3],
                    heap[tag_heap_idx+4],
                    heap[tag_heap_idx+5],
                    heap[tag_heap_idx+6],
                    heap[tag_heap_idx+7],
                    heap[tag_heap_idx+8],
                    heap[tag_heap_idx+9],
                    heap[tag_heap_idx+10],
                    heap[tag_heap_idx+11],
                    heap[tag_heap_idx+12],
                    heap[tag_heap_idx+13],
                    heap[tag_heap_idx+14],
                    heap[tag_heap_idx+15],
                ]
            }
        }
    }
}
