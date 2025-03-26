use std::ops::Sub;

#[repr(C)]
pub struct Player {
    _pad_0x2c: [u8; 0x2c],
    pub pos: WorldPosition,
    pub view_angles: ViewAngles,
    _pad_0x100: [u8; 0xbc],
    pub health: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct ViewAngles {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Player {
    pub fn angles_to(&self, other: &Player) -> ViewAngles {
        let delta = other.pos - self.pos;
        // Horizontal distance
        let dist_xy = (delta.x * delta.x + delta.y * delta.y).sqrt();
        let yaw = delta.y.atan2(delta.x).to_degrees();
        let pitch = delta.z.atan2(dist_xy).to_degrees();
        ViewAngles {
            yaw: yaw + 90.0,
            pitch,
            roll: 0.0,
        }
    }
}

impl Sub for WorldPosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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
