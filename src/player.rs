use std::ops::Sub;

#[repr(C)]
pub struct Player {
    _pad_0x2c: [u8; 0x2c],
    pub pos: WorldPosition,
    pub view_angles: ViewAngles,
    _pad_0x7a: [u8; 0x36],
    pub state: u8,
    _pad_0x100: [u8; 0x85],
    pub health: i32,
    _pad_0x320: [u8; 0x21c],
    pub team: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union WorldPosition {
    pub v: Vec3,
    pub f: [f32; 3],
    pub i: [i32; 3],
}

#[repr(C)]
pub struct ViewAngles {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.state == 0 // alive = 0, dead, spawning, lagged, editing, spectate
    }

    pub unsafe fn angles_to(&self, other: &Player) -> ViewAngles {
        let delta = other.pos - self.pos;
        // Horizontal distance
        let dist_xy = (delta.v.x * delta.v.x + delta.v.y * delta.v.y).sqrt();
        let yaw = delta.v.y.atan2(delta.v.x).to_degrees();
        let pitch = delta.v.z.atan2(dist_xy).to_degrees();
        ViewAngles {
            yaw: yaw + 90.0,
            pitch,
            roll: 0.0,
        }
    }
}

impl Default for WorldPosition {
    #[inline]
    fn default() -> Self {
        WorldPosition { v: Vec3::default() }
    }
}

impl Sub for WorldPosition {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe {
            WorldPosition {
                v: Vec3 {
                    x: self.v.x - rhs.v.x,
                    y: self.v.y - rhs.v.y,
                    z: self.v.z - rhs.v.z,
                },
            }
        }
    }
}

impl WorldPosition {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        WorldPosition {
            v: Vec3 { x, y, z },
        }
    }
}

impl ViewAngles {
    pub fn fov_to(&self, other: &ViewAngles) -> f32 {
        // C's remainderf
        fn remainder(x: f32, y: f32) -> f32 {
            let n = (x / y).round();
            x - n * y
        }

        let yaw = remainder(self.yaw - other.yaw, 360.0).clamp(0.0, 360.0);
        let pitch = remainder(self.pitch - other.pitch, 360.0).clamp(-90.0, 90.0);

        pitch.hypot(yaw)
    }
}
