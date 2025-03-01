use log::{error, info};
use process::Process;

mod process;

fn init() -> anyhow::Result<()> {
    let process = Process::new()?;
    info!("Hello world! Base address: {:#X}", process.base_address);
    info!("player1 address: {:#X}", process.base_address + 0x1ab4b8);
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
