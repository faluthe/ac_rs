#[repr(C)]
pub struct Player {
    _pad_0x100: [u8; 0x100],
    pub health: i32,
}
