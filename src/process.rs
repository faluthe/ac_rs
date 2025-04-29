use std::{ffi::c_void, fs, mem, ptr};

use anyhow::anyhow;
use goblin::elf::Elf;
use libc::{c_int, dl_iterate_phdr, dl_phdr_info, size_t};
use log::debug;

use crate::player::{Player, WorldPosition};

const PLAYER1_SYMBOL: &str = "player1";
const PLAYERS_SYMBOL: &str = "players";
const IS_VISIBLE_SYMBOL: &str = "_Z9IsVisible3vecS_P6dynentb";

#[derive(Copy, Clone)]
pub struct Process {
    base_address: u64,
}

// This is shit but I don't want to use nightly for OnceLock::get_or_try_init
macro_rules! static_symbol_address {
    ($self:ident, $symbol:ident) => {{
        static mut OFFSET: u64 = 0;
        unsafe {
            if OFFSET == 0 {
                OFFSET = $self.get_symbol_offset($symbol)?;
            }
            $self.base_address + OFFSET
        }
    }};
}

impl Process {
    pub unsafe fn new() -> anyhow::Result<Self> {
        let base_address = Self::main_exe_base().ok_or(anyhow!("Failed to find main exe base"))?;
        debug!("Process base address found @ {:#X}", base_address);
        Ok(Self { base_address })
    }

    pub unsafe fn get_player1(&self) -> anyhow::Result<&'static mut Player> {
        let addr = static_symbol_address!(self, PLAYER1_SYMBOL);
        Ok(&mut **(addr as *const *mut Player))
    }

    pub unsafe fn get_players(&self) -> anyhow::Result<Vec<&'static Player>> {
        let addr = static_symbol_address!(self, PLAYERS_SYMBOL);
        // AssaultCube::vector stores a capacity and a length, we need the length
        let length = &*((addr + 0xc) as *const u32);
        // The symbol points to a pointer to a list of Player pointers
        let list = *(addr as *const *const *const Player);
        // Localplayer is first, skip it
        Ok((1..*length).map(|i| &**list.offset(i as isize)).collect())
    }

    pub unsafe fn is_visible(&self, player: &Player, other: &Player) -> anyhow::Result<bool> {
        let addr = static_symbol_address!(self, IS_VISIBLE_SYMBOL);
        let is_visible: extern "C" fn(WorldPosition, WorldPosition, *const Player, bool) -> bool =
            mem::transmute(addr);

        // lol
        let from = WorldPosition::new(player.pos.v.x, player.pos.v.y, player.pos.v.z + 3.0);
        let to = WorldPosition::new(other.pos.v.x, other.pos.v.y, other.pos.v.z + 3.0);

        Ok(is_visible(from, to, ptr::null(), false))
    }

    fn get_symbol_offset(&self, symbol: &str) -> anyhow::Result<u64> {
        let bin = fs::read("/proc/self/exe")?;
        let elf = Elf::parse(&bin)?;

        for sym in &elf.syms {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if name == symbol {
                    debug!("Found symbol {} @ {:#X}", symbol, sym.st_value);
                    return Ok(sym.st_value);
                }
            }
        }

        Err(anyhow!("Failed to find symbol: {}", symbol))
    }

    unsafe fn main_exe_base() -> Option<u64> {
        unsafe extern "C" fn callback(
            info: *mut dl_phdr_info,
            _size: size_t,
            data: *mut c_void,
        ) -> c_int {
            let info = &*info;
            let base_address = data as *mut u64;
            // The main executable has a null name
            if info.dlpi_name.is_null() || *info.dlpi_name == 0 {
                *base_address = info.dlpi_addr;
                1 // Stop iterating
            } else {
                0
            }
        }

        let mut base_address: u64 = 0;
        // The main executable is the first entry, so no real need to iterate :shrug:
        dl_iterate_phdr(Some(callback), &mut base_address as *mut u64 as *mut c_void);
        match base_address {
            0 => None,
            _ => Some(base_address),
        }
    }
}
