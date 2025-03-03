use std::{ffi::c_void, fs};

use anyhow::anyhow;
use goblin::elf::Elf;
use libc::{c_int, dl_iterate_phdr, dl_phdr_info, size_t};
use log::debug;

use crate::player::Player;

const PLAYER1_SYMBOL: &str = "player1";

pub struct Process {
    base_address: u64,
}

impl Process {
    pub fn new() -> anyhow::Result<Self> {
        let base_address = Self::main_exe_base().ok_or(anyhow!("Failed to find main exe base"))?;
        Ok(Self { base_address })
    }

    pub fn get_player1(&self) -> anyhow::Result<&'static Player> {
        let offset = self.get_symbol_offset(PLAYER1_SYMBOL)?;
        let addr = self.base_address + offset;

        Ok(unsafe { &**(addr as *const *const Player) })
    }

    fn get_symbol_offset(&self, symbol: &str) -> anyhow::Result<u64> {
        let bin = fs::read("/proc/self/exe")?;
        let elf = Elf::parse(&bin)?;

        for sym in &elf.syms {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if name == symbol {
                    debug!("Found symbol: {} @ {:#x}", symbol, sym.st_value);
                    return Ok(sym.st_value);
                }
            }
        }

        Err(anyhow!("Failed to find symbol: {}", symbol))
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
