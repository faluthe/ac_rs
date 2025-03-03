use log::{error, info};
use player::Player;
use process::Process;

mod player;
mod process;
mod symbol;

fn init() -> anyhow::Result<()> {
    // Get the process and localplayer
    let process = Process::new()?;
    let player1 = Player::get_player1(&process);

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
