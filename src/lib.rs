use log::error;

mod aimbot;
mod esp;
mod gl;
mod hooks;
mod player;
mod process;

#[used]
#[link_section = ".init_array"]
static INIT: unsafe extern "C" fn() = {
    unsafe extern "C" fn init() {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
        if let Err(e) = hooks::init() {
            error!("{}", e);
        }
    }
    init
};

#[used]
#[link_section = ".fini_array"]
static FINI: unsafe extern "C" fn() = {
    unsafe extern "C" fn fini() {
        if let Err(e) = hooks::fini() {
            error!("{}", e);
        }
    }
    fini
};
