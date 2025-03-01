use std::env;

use anyhow::anyhow;
use log::{error, info};
use process::Process;

mod process;

#[repr(C)]
pub struct Player {
    _pad_0x100: [u8; 0x100],
    pub health: i32,
}

fn init() -> anyhow::Result<()> {
    // Initialize environment variables
    dotenvy::from_path("/home/pat/vs/ac_rs/.env")
        .ok()
        .ok_or(anyhow!("Failed to load .env file"))?;
    let player1_offset = u64::from_str_radix(&env::var("PLAYER1_OFFSET")?, 16)?;

    // Get the process and localplayer
    let process = Process::new()?;
    let player1 =
        { unsafe { &**((process.base_address + player1_offset) as *const *const Player) } };

    info!("Player 1 health: {}", player1.health);

    Ok(())
}

#[used]
#[link_section = ".init_array"]
static INIT: extern "C" fn() = {
    extern "C" fn init_wrapper() {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .init();
        if let Err(e) = init() {
            error!("Initialization error: {}", e);
        }
    }
    init_wrapper
};
