#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/sha256_bindings.rs"));

pub struct Context {
    ctx: SHA256_CTX,
}

impl Context {
    pub fn new() -> Self {
        use std::mem::MaybeUninit;
        let ctx = unsafe {
            let mut ctx: MaybeUninit<SHA256_CTX> = MaybeUninit::uninit();
            guest_func_init(ctx.as_mut_ptr());
            ctx.assume_init()
        };
        Self {
            ctx,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe { guest_func_update(&mut self.ctx as *mut SHA256_CTX, data.as_ptr(), data.len() as u64) }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        let mut out: [u8; 32] = [0; 32];
        unsafe { guest_func_final(&mut self.ctx as *mut SHA256_CTX, out.as_mut_ptr()); }
        out
    }
}
