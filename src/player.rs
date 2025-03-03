use crate::process::Process;

#[repr(C)]
pub struct Player {
    _pad_0x100: [u8; 0x100],
    pub health: i32,
}

impl Player {
    pub fn get_player1(p: &Process) -> &'static Player {
        let player1_offset = 0x1AB4B8;
        unsafe { &**((p.base_address + player1_offset) as *const *const Player) }
    }
}
