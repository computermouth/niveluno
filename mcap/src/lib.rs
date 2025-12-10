
use core::f32;

pub use glam::Vec3;

pub enum Surface {
    Wall{ tri: [Vec3;3], normal: Vec3, origin_offset: f32 },
    Floor{ tri: [Vec3;3], normal: Vec3 },
    Cieling{ tri: [Vec3;3], normal: Vec3 },
}

impl Surface {
    pub fn new(tri: [Vec3;3], normal: Vec3) -> Self {

        if normal.y.abs() < 0.01 {
            return Surface::Wall { tri, normal, origin_offset: -normal.dot(tri[0]) }
        }

        match normal.y {
            y if (-f32::INFINITY..-0.01).contains(&y) => {
                Surface::Cieling { tri, normal }
            },
            y if (0.01..f32::INFINITY).contains(&y) => {
                Surface::Floor { tri, normal }
            },
            _ => {
                Surface::Wall { tri, normal, origin_offset: -normal.dot(tri[0]) }
            }
        }
    }
}

pub fn check_wall_collision(pos: &mut Vec3, surface: Surface) -> Option<Vec3> {
    None
}
