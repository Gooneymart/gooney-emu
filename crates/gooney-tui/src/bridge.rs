// crates/gooney-tui/src/bridge.rs
use std::os::raw::{c_char, c_int};
#[allow(dead_code)]
mod ffi {
    use super::*;
    extern "C" {
        pub fn xzxt_sim_init(firmware_path: *const c_char);
        pub fn xzxt_sim_destroy();
        pub fn xzxt_sim_step(branch_taken: c_int, target_pc: u32, stall: c_int, flush: c_int);

        pub fn xzxt_get_pc() -> u64;
        pub fn xzxt_get_wb_result() -> u64;
        pub fn xzxt_get_wb_rd() -> u8;
        pub fn xzxt_get_wb_commit() -> c_int;
        pub fn xzxt_get_eflags() -> u64;

        pub fn xzxt_get_mem_ren() -> c_int;
        pub fn xzxt_get_mem_wen() -> c_int;
        pub fn xzxt_get_mem_addr() -> u32;
        pub fn xzxt_get_mem_wdata() -> u64;
        pub fn xzxt_get_mem_rdata() -> u64;
        pub fn xzxt_get_inst() -> u32;
        pub fn xzxt_get_custom_active() -> c_int;
        pub fn xzxt_get_sim_done() -> c_int;
    }
}
#[allow(dead_code)]
pub trait SimCore {
    unsafe fn init(&self, firmware_path: &str);
    unsafe fn destroy(&self);
    unsafe fn step(&self, branch_taken: bool, target_pc: u32, stall: bool, flush: bool);
    unsafe fn get_pc(&self) -> u64;
    unsafe fn get_wb_result(&self) -> u64;
    unsafe fn get_wb_rd(&self) -> u8;
    unsafe fn get_wb_commit(&self) -> bool;
    unsafe fn get_eflags(&self) -> u64;
    unsafe fn get_mem_ren(&self) -> bool;
    unsafe fn get_mem_wen(&self) -> bool;
    unsafe fn get_mem_addr(&self) -> u32;
    unsafe fn get_mem_wdata(&self) -> u64;
    unsafe fn get_mem_rdata(&self) -> u64;
    unsafe fn get_inst(&self) -> u32;
    unsafe fn get_custom_active(&self) -> bool;
    unsafe fn get_sim_done(&self) -> bool;
}

pub struct ActiveSimCore;

impl SimCore for ActiveSimCore {
    unsafe fn init(&self, firmware_path: &str) {
        let c_path = std::ffi::CString::new(firmware_path).unwrap();
        ffi::xzxt_sim_init(c_path.as_ptr());
    }
    unsafe fn destroy(&self) { ffi::xzxt_sim_destroy(); }
    unsafe fn step(&self, branch_taken: bool, target_pc: u32, stall: bool, flush: bool) {
        ffi::xzxt_sim_step(branch_taken as c_int, target_pc, stall as c_int, flush as c_int);
    }
    unsafe fn get_pc(&self) -> u64 { ffi::xzxt_get_pc() }
    unsafe fn get_wb_result(&self) -> u64 { ffi::xzxt_get_wb_result() }
    unsafe fn get_wb_rd(&self) -> u8 { ffi::xzxt_get_wb_rd() }
    unsafe fn get_wb_commit(&self) -> bool { ffi::xzxt_get_wb_commit() != 0 }
    unsafe fn get_eflags(&self) -> u64 { ffi::xzxt_get_eflags() }
    unsafe fn get_mem_ren(&self) -> bool { ffi::xzxt_get_mem_ren() != 0 }
    unsafe fn get_mem_wen(&self) -> bool { ffi::xzxt_get_mem_wen() != 0 }
    unsafe fn get_mem_addr(&self) -> u32 { ffi::xzxt_get_mem_addr() }
    unsafe fn get_mem_wdata(&self) -> u64 { ffi::xzxt_get_mem_wdata() }
    unsafe fn get_mem_rdata(&self) -> u64 { ffi::xzxt_get_mem_rdata() }
    unsafe fn get_inst(&self) -> u32 { ffi::xzxt_get_inst() }
    unsafe fn get_custom_active(&self) -> bool { ffi::xzxt_get_custom_active() != 0 }
    unsafe fn get_sim_done(&self) -> bool { ffi::xzxt_get_sim_done() != 0 }
}
