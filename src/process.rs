use std::ffi::c_void;

use anyhow::anyhow;
use libc::{c_int, dl_iterate_phdr, dl_phdr_info, size_t};

pub struct Process {
    pub base_address: u64,
}

impl Process {
    pub fn new() -> anyhow::Result<Self> {
        let base_address = Self::main_exe_base().ok_or(anyhow!("Failed to find main exe base"))?;
        Ok(Self { base_address })
    }

    fn main_exe_base() -> Option<u64> {
        extern "C" fn callback(info: *mut dl_phdr_info, _size: size_t, data: *mut c_void) -> c_int {
            unsafe {
                let info = &*info;
                let base_address = data as *mut u64;
                if info.dlpi_name.is_null() || *info.dlpi_name == 0 {
                    *base_address = info.dlpi_addr;
                    1 // Stop iterating
                } else {
                    0
                }
            }
        }

        let mut base_address: u64 = 0;
        unsafe {
            dl_iterate_phdr(Some(callback), &mut base_address as *mut u64 as *mut c_void);
        }
        match base_address {
            0 => None,
            _ => Some(base_address),
        }
    }
}
