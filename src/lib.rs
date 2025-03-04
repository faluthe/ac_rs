use log::{error, info};
use process::Process;

mod hooks;
mod player;
mod process;

unsafe fn init() -> anyhow::Result<()> {
    // Get the process and localplayer
    let process = Process::new()?;

    let player1 = process.get_player1()?;
    info!("Player 1 health: {}", player1.health);

    let players = process.get_players()?;
    for player in players {
        info!("Player health: {}", player.health);
    }

    hooks::init()?;

    Ok(())
}

#[used]
#[link_section = ".init_array"]
static INIT: extern "C" fn() = {
    extern "C" fn init_wrapper() {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
        if let Err(e) = unsafe { init() } {
            error!("Initialization error: {}", e);
        }
    }
    init_wrapper
};
