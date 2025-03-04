use std::ffi::CString;

use anyhow::anyhow;
use libc::{dlopen, dlsym, RTLD_LAZY, RTLD_NOLOAD};
use log::debug;

const SDL2_LIB: &str = "/usr/lib/x86_64-linux-gnu/libSDL2-2.0.so.0.3000.0";

pub unsafe fn hook_swap_window() -> anyhow::Result<()> {
    let lib = CString::new(SDL2_LIB)?;
    let h_sdl = dlopen(lib.as_ptr(), RTLD_NOLOAD | RTLD_LAZY);
    if h_sdl.is_null() {
        return Err(anyhow!("Failed to load SDL2 @ {}", SDL2_LIB));
    }
    debug!("SDL2 handle: {:#X}", h_sdl as u64);

    // The symbol resolves to a wrapper around the actual function
    let fn_name = CString::new("SDL_GL_SwapWindow")?;
    let sym = {
        let sym = dlsym(h_sdl, fn_name.as_ptr());
        if sym.is_null() {
            return Err(anyhow!("Failed to find SDL_GL_SwapWindow"));
        }
        sym as u64
    };
    debug!("SDL_GL_SwapWindow: {:#X}", sym);

    /* SDL_GL_SwapWindow:
     * endbr64            ; (4 bytes)
     * jmp *(ip + offset) ; (2 byte jmp, 4 byte offset) */
    let offset = *((sym + 0x6) as *const i32);
    let ip = sym + 0xa;
    debug!("SDL_GL_SwapWindow GOT offset: {:#X}", offset);
    debug!("SDL_GL_SwapWindow ip: {:#X}", ip);

    Ok(())
}
