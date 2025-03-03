use log::{error, info};
use process::Process;

mod player;
mod process;

fn init() -> anyhow::Result<()> {
    // Get the process and localplayer
    let process = Process::new()?;

    let player1 = process.get_player1()?;
    info!("Player 1 health: {}", player1.health);

    Ok(())
}

#[used]
#[link_section = ".init_array"]
static INIT: extern "C" fn() = {
    extern "C" fn init_wrapper() {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
        if let Err(e) = init() {
            error!("Initialization error: {}", e);
        }
    }
    init_wrapper
};
