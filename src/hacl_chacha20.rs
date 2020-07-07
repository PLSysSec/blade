use crate::blade_setting::BladeType;
use crate::module::{get_lucet_module, BladeModule};

use lucet_runtime::InstanceHandle;
use std::fmt;

const KEY_BYTES: usize = 32;
const NONCE_BYTES: usize = 12;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Chacha20Key {
    key: [u8; KEY_BYTES],
}

impl Chacha20Key {
    pub fn new(data: [u8; KEY_BYTES]) -> Self {
        Self {
            key: data,
        }
    }
}

impl fmt::Display for Chacha20Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}{:02x}_", self.key[0], self.key[1], self.key[2], self.key[3])?;
        write!(f, "{:02x}{:02x}{:02x}{:02x}_", self.key[4], self.key[5], self.key[6], self.key[7])?;
        write!(f, "{:02x}{:02x}{:02x}{:02x}_", self.key[8], self.key[9], self.key[10], self.key[11])?;
        write!(f, "{:02x}{:02x}{:02x}{:02x}_", self.key[12], self.key[13], self.key[14], self.key[15])?;
        write!(f, "{:02x}{:02x}{:02x}{:02x}_", self.key[16], self.key[17], self.key[18], self.key[19])?;
        write!(f, "{:02x}{:02x}{:02x}{:02x}_", self.key[20], self.key[21], self.key[22], self.key[23])?;
        write!(f, "{:02x}{:02x}{:02x}{:02x}_", self.key[24], self.key[25], self.key[26], self.key[27])?;
        write!(f, "{:02x}{:02x}{:02x}{:02x}_", self.key[28], self.key[29], self.key[30], self.key[31])?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Chacha20Nonce {
    nonce: [u8; NONCE_BYTES],
}

impl Chacha20Nonce {
    pub fn new(data: [u8; NONCE_BYTES]) -> Self {
        Self {
            nonce: data,
        }
    }
}

impl fmt::Display for Chacha20Nonce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}{:02x}_", self.nonce[0], self.nonce[1], self.nonce[2], self.nonce[3])?;
        write!(f, "{:02x}{:02x}{:02x}{:02x}", self.nonce[4], self.nonce[5], self.nonce[6], self.nonce[7])?;
        Ok(())
    }
}

pub struct Chacha20Module {
    so: InstanceHandle,
}

impl BladeModule for Chacha20Module {
    fn new(blade_type: BladeType, blade_v1_1: bool) -> Self {
        Self {
            so: get_lucet_module("wasm_obj/Hacl_Chacha20", blade_type, blade_v1_1),
        }
    }
}

impl Chacha20Module {
    /// Returns the encryption of `msg`. Result will have the same length as `msg`.
    pub fn encrypt(&mut self, key: &Chacha20Key, nonce: &Chacha20Nonce, msg: &[u8]) -> Vec<u8> {
        // allocation
        let mut heap_base = unsafe {
            self.so.globals()[0].i_32 as u32 // seems like global 0 is the heap base?
        };
        let key_ptr = heap_base;
        heap_base += KEY_BYTES as u32;
        let nonce_ptr = heap_base;
        heap_base += NONCE_BYTES as u32;
        let msg_ptr = heap_base;
        heap_base += msg.len() as u32;
        let out_ptr = heap_base;

        // set up inputs
        let heap = self.so.heap_mut();
        let key_heap_idx = key_ptr as usize;
        for i in 0 .. KEY_BYTES {
            heap[key_heap_idx + i] = key.key[i];
        }
        let nonce_heap_idx = nonce_ptr as usize;
        for i in 0 .. NONCE_BYTES {
            heap[nonce_heap_idx + i] = nonce.nonce[i];
        }
        let msg_heap_idx = msg_ptr as usize;
        for i in 0 .. msg.len() {
            heap[msg_heap_idx + i] = msg[i];
        }

        // call wasm
        let _ = self.so.run("Hacl_Chacha20_chacha20_encrypt", &[
            msg.len().into(),
            out_ptr.into(),
            msg_ptr.into(),
            key_ptr.into(),
            nonce_ptr.into(),
            0.into(), // use 0 for the `ctr` argument? There's no docs for this
        ]).unwrap();
        let mut output = vec![];
        let heap = self.so.heap();
        let out_heap_idx = out_ptr as usize;
        for i in 0 .. msg.len() {
            output.push(heap[out_heap_idx + i]);
        }
        output
    }

    /// Returns the decryption of `ciphertext`. Result will have the same length as `ciphertext`.
    pub fn decrypt(&mut self, key: &Chacha20Key, nonce: &Chacha20Nonce, ciphertext: &[u8]) -> Vec<u8> {
        // allocation
        let mut heap_base = unsafe {
            self.so.globals()[0].i_32 as u32 // seems like global 0 is the heap base?
        };
        let key_ptr = heap_base;
        heap_base += KEY_BYTES as u32;
        let nonce_ptr = heap_base;
        heap_base += NONCE_BYTES as u32;
        let ciphertext_ptr = heap_base;
        heap_base += ciphertext.len() as u32;
        let out_ptr = heap_base;

        // set up inputs
        let heap = self.so.heap_mut();
        let key_heap_idx = key_ptr as usize;
        for i in 0 .. KEY_BYTES {
            heap[key_heap_idx + i] = key.key[i];
        }
        let nonce_heap_idx = nonce_ptr as usize;
        for i in 0 .. NONCE_BYTES {
            heap[nonce_heap_idx + i] = nonce.nonce[i];
        }
        let ciphertext_heap_idx = ciphertext_ptr as usize;
        for i in 0 .. ciphertext.len() {
            heap[ciphertext_heap_idx + i] = ciphertext[i];
        }

        // call wasm
        let _ = self.so.run("Hacl_Chacha20_chacha20_decrypt", &[
            ciphertext.len().into(),
            out_ptr.into(),
            ciphertext_ptr.into(),
            key_ptr.into(),
            nonce_ptr.into(),
            0.into(), // use 0 for the `ctr` argument? There's no docs for this
        ]).unwrap();
        let mut output = vec![];
        let heap = self.so.heap();
        let out_heap_idx = out_ptr as usize;
        for i in 0 .. ciphertext.len() {
            output.push(heap[out_heap_idx + i]);
        }
        output
    }
}
