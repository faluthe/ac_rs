use std::{
    ffi::{c_void, CString},
    ptr,
    sync::OnceLock,
};

use anyhow::anyhow;
use libc::{dlopen, dlsym, RTLD_LAZY, RTLD_NOLOAD};
use log::{debug, warn};

use crate::process::Process;

/// The path to SDL2. Adjust if necessary.
const SDL2_LIB: &str = "/usr/lib/x86_64-linux-gnu/libSDL2-2.0.so.0.3000.0";

static PROCESS: OnceLock<Process> = OnceLock::new();
static mut SDL_HANDLE: Option<*mut c_void> = None;
static mut SWAP_WINDOW_HOOK: Option<SDLHook<unsafe extern "C" fn(*mut c_void)>> = None;

pub unsafe fn init() -> anyhow::Result<()> {
    SWAP_WINDOW_HOOK = Some(SDLHook::new("SDL_GL_SwapWindow", hk_swap_window as _)?);
    Ok(())
}

pub unsafe fn fini() -> anyhow::Result<()> {
    match SWAP_WINDOW_HOOK {
        Some(ref h) => {
            h.restore();
            SWAP_WINDOW_HOOK = None;
        }
        None => return Err(anyhow!("Hook was not initialized before restore")),
    }
    Ok(())
}

unsafe extern "C" fn hk_swap_window(window: *mut c_void) {
    unsafe fn hk(window: *mut c_void) -> anyhow::Result<()> {
        let og = match SWAP_WINDOW_HOOK {
            Some(ref h) => h.og,
            // You really shouldn't be here
            None => return Err(anyhow!("Hook was not initialized properly")),
        };
        let process = PROCESS.get_or_init(|| Process::new().expect("Failed to initialize process"));

        og(window);

        let player1 = process.get_player1()?;
        // Find the angles to the player with the smallest fov
        let best_angles = process
            .get_players()?
            .into_iter()
            .map(|player| player1.angles_to(player))
            .min_by(|a, b| {
                player1
                    .view_angles
                    .fov_to(a)
                    .partial_cmp(&player1.view_angles.fov_to(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        if let Some(best_angles) = best_angles {
            player1.view_angles = best_angles;
        }

        Ok(())
    }

    if let Err(e) = hk(window) {
        warn!("Error in hk_swap_window: {}", e);
    }
}

/// Generic hook for SDL2 functions
struct SDLHook<T: Copy> {
    sym_name: String,
    got_entry: *mut T,
    og: T,
}

impl<T: Copy> SDLHook<T> {
    unsafe fn new(sym_name: &str, hk: T) -> anyhow::Result<Self> {
        let lib = match SDL_HANDLE {
            Some(h) => h,
            None => {
                let s = CString::new(SDL2_LIB)?;
                let h = dlopen(s.as_ptr(), RTLD_NOLOAD | RTLD_LAZY);
                if h.is_null() {
                    return Err(anyhow!("Failed to load SDL2 @ {}", SDL2_LIB));
                }
                debug!("SDL2 handle @ {:#X}", h as u64);
                SDL_HANDLE = Some(h);
                h
            }
        };

        // The symbol resolves to a wrapper around the actual function
        let c_sym_name = CString::new(sym_name)?;
        let sym_addr = {
            let sym = dlsym(lib, c_sym_name.as_ptr());
            if sym.is_null() {
                return Err(anyhow!("Failed to find {}", sym_name));
            }
            sym as u64
        };
        debug!("{} @ {:#X}", sym_name, sym_addr);

        /* SDL Wrapper Function:
         * endbr64            ; (4 bytes)
         * jmp *(ip + offset) ; (2 byte jmp, 4 byte offset) */
        let offset = ptr::read_unaligned((sym_addr + 0x6) as *const i32) as u64;
        // Instruction pointer
        let ip = sym_addr + 0xa;
        // ip + offset points to the GOT entry for the actual function
        let got_entry = (ip + offset) as *mut T;
        let og = *got_entry;
        // The page is already rw so no need to change permissions
        *got_entry = hk;
        warn!("Hooked {}", sym_name);

        // No dlclose, don't decrement the reference count
        Ok(Self {
            sym_name: sym_name.to_string(),
            got_entry,
            // Need to call the actual function from the hook
            og,
        })
    }

    unsafe fn restore(&self) {
        *self.got_entry = self.og;
        warn!("Restored {}", self.sym_name);
    }
}
