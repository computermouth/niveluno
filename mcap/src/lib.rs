use core::f32;

pub use glam::Vec3;

#[derive(Debug)]
pub struct Wall {
    tri: [Vec3; 3],
    normal: Vec3,
    origin_offset: f32,
}
#[derive(Debug)]
pub struct Floor {
    tri: [Vec3; 3],
    normal: Vec3,
}
#[derive(Debug)]
pub struct Cieling {
    tri: [Vec3; 3],
    normal: Vec3,
}

#[derive(Debug)]
pub enum Surface {
    Wall(Wall),
    Floor(Floor),
    Cieling(Cieling),
}

impl Surface {
    pub fn new(tri: [Vec3; 3], normal: Vec3) -> Self {
        match normal.y {
            y if (-f32::INFINITY..-0.01).contains(&y) => Surface::Cieling(Cieling { tri, normal }),
            y if (0.01..f32::INFINITY).contains(&y) => Surface::Floor(Floor { tri, normal }),
            _ => Surface::Wall(Wall {
                tri,
                normal,
                origin_offset: -normal.dot(tri[0]),
            }),
        }
    }
}

pub fn get_face_normal(v1_pos: Vec3, v2_pos: Vec3, v3_pos: Vec3) -> Vec3 {
    let edge1 = v2_pos - v1_pos;
    let edge2 = v3_pos - v1_pos;

    edge1.cross(edge2).normalize()
}

pub fn check_wall_collision(pos: Vec3, radius: f32, height: f32, wall: Wall) -> Option<Vec3> {
    // distance from pos to wall
    let offset = wall.normal.dot(pos) + wall.origin_offset;

    if offset.abs() >= radius {
        return None;
    }

    // triangle's min and max y
    let min_y = wall.tri[0].y.min(wall.tri[1].y).min(wall.tri[2].y);
    let max_y = wall.tri[0].y.max(wall.tri[1].y).max(wall.tri[2].y);

    let bot = pos.y;
    let top = pos.y + height;

    // too low or too high
    if top < min_y || bot > max_y {
        return None;
    }

    let depth = radius - offset.abs();
    let push = depth * offset.signum();

    Some(Vec3::new(wall.normal.x * push, 0., wall.normal.z * push))
}
