#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/tea_bindings.rs"));

use std::fmt;

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

pub fn encrypt(msg: &TeaMsg, key: &TeaKey) -> TeaMsg {
    let mut out = msg.clone();
    unsafe { guest_func_encrypt(out.msg.as_mut_ptr(), key.key.as_ptr()); }
    out
}

pub fn decrypt(msg: &TeaMsg, key: &TeaKey) -> TeaMsg {
    let mut out = msg.clone();
    unsafe { guest_func_decrypt(out.msg.as_mut_ptr(), key.key.as_ptr()); }
    out
}
