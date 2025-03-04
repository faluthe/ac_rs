use std::{
    ffi::{c_void, CString},
    ptr,
};

use anyhow::anyhow;
use libc::{dlopen, dlsym, RTLD_LAZY, RTLD_NOLOAD};
use log::{debug, warn};

use crate::process::Process;

const SDL2_LIB: &str = "/usr/lib/x86_64-linux-gnu/libSDL2-2.0.so.0.3000.0";

static mut PROCESS: Option<Process> = None;
static mut OG_SWAP_WINDOW: Option<unsafe extern "C" fn(*mut c_void)> = None;

pub unsafe fn init() -> anyhow::Result<()> {
    // The process needs to be initialized before any hooks
    PROCESS = Some(Process::new()?);
    hook_swap_window()?;
    Ok(())
}

unsafe extern "C" fn hk_swap_window(window: *mut c_void) {
    unsafe fn hk(window: *mut c_void) -> anyhow::Result<()> {
        let og = OG_SWAP_WINDOW.ok_or(anyhow!("SwapWindow original function not initialized"))?;
        let process = PROCESS.ok_or(anyhow!("Process not initialized"))?;

        og(window);

        let player1 = process.get_player1()?;
        debug!("Player 1 health: {}", player1.health);

        Ok(())
    }

    if let Err(e) = hk(window) {
        warn!("Error in hk_swap_window: {}", e);
    }
}

unsafe fn hook_swap_window() -> anyhow::Result<()> {
    let lib = CString::new(SDL2_LIB)?;
    let h_sdl = dlopen(lib.as_ptr(), RTLD_NOLOAD | RTLD_LAZY);
    if h_sdl.is_null() {
        return Err(anyhow!("Failed to load SDL2 @ {}", SDL2_LIB));
    }
    debug!("SDL2 handle: {:#X}", h_sdl as u64);

    // The symbol resolves to a wrapper around the actual function
    let fn_name = CString::new("SDL_GL_SwapWindow")?;
    let sym_addr = {
        let sym = dlsym(h_sdl, fn_name.as_ptr());
        if sym.is_null() {
            return Err(anyhow!("Failed to find SDL_GL_SwapWindow"));
        }
        sym as u64
    };
    debug!("SDL_GL_SwapWindow: {:#X}", sym_addr);

    /* SDL_GL_SwapWindow:
     * endbr64            ; (4 bytes)
     * jmp *(ip + offset) ; (2 byte jmp, 4 byte offset) */
    let offset = ptr::read_unaligned((sym_addr + 0x6) as *const i32) as u64;
    // Instruction pointer
    let ip = sym_addr + 0xa;
    // ip + offset points to the GOT entry for the actual function
    let fn_got_entry = (ip + offset) as *mut unsafe extern "C" fn(*mut c_void);
    // Need to call the actual function from the hook
    OG_SWAP_WINDOW = Some(*fn_got_entry);
    // The page is already rw so no need to change permissions
    *fn_got_entry = hk_swap_window;
    warn!("Hooked SDL_GL_SwapWindow");

    // No dlclose, don't decrement the reference count
    Ok(())
}
