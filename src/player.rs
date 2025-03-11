#[repr(C)]
pub struct Player {
    _pad_0x2c: [u8; 0x2c],
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    _pad_0x100: [u8; 0xbc],
    pub health: i32,
}
