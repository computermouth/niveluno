
use raylib::prelude::*;
use mcap::{Vec3, Surface, check_wall_collision};

mod ex1;
mod ex2;
mod ex3;
mod ex4;

trait ToVec3 {
    fn to_sscv3(&self) -> Vec3;
}

trait ToVector3 {
    fn to_rayv3(&self) -> Vector3;
}

impl ToVec3 for Vector3 {
    fn to_sscv3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl ToVector3 for Vec3 {
    fn to_rayv3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

pub fn at_origin(v: Vector3) -> Vector3 {
    v + Vector3::one() * 100.
}

enum Shape {
    Cylinder{pos: Vector3, height: f32, radius: f32},
    CylinderWires{pos: Vector3, height: f32, radius: f32},
    Triangle([Vector3;3])
}

trait Example {
    fn update(&mut self, fd: f32, time: f64, reset: bool) -> Vec<(Shape, Color)>;
    fn draw_2d(&mut self, d: RaylibDrawHandle<'_>);
}

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("gui showcase")
        .build();

    let c_position = at_origin(Vector3::new(0., 5., -5.));

    let camera = Camera3D::perspective(
        c_position,
        at_origin(Vector3::zero()),
            Vector3::new(0.0, 1.0, 0.0),
            90.0);

    const NUM_EXAMPLES: usize = 4;
    let mut example_id: usize = 0;
    let mut example: Box<dyn Example> = Box::new(ex1::State::new());

    while !rl.window_should_close() {

    let fd = rl.get_frame_time();
    let time = rl.get_time();
    let reset = rl.is_key_pressed(KeyboardKey::KEY_R);
    let prev = rl.is_key_pressed(KeyboardKey::KEY_P);
    let next = rl.is_key_pressed(KeyboardKey::KEY_N);
    let change = prev as i8 - next as i8;

    let mut d = rl.begin_drawing(&thread);
    
        if change != 0 {
            if prev {
                example_id = example_id.checked_sub(1).unwrap_or(NUM_EXAMPLES - 1);
            } else if next {
                example_id = (example_id + 1) % NUM_EXAMPLES;
            }
            example = match example_id {
                0 => Box::new(ex1::State::new()),
                1 => Box::new(ex2::State::new()),
                2 => Box::new(ex3::State::new()),
                3 => Box::new(ex4::State::new()),
                _ => panic!()
            }
        }

        let draws = example.update(fd, time, reset);

        d.clear_background(Color::new(16, 16, 32, 255));
        d.draw_mode3D(camera, |mut d3d, _| {

            for (shape, color) in &draws {
                match shape {
                    Shape::Triangle(tri) => {
                        // eat shit, normals
                        d3d.draw_triangle3D(tri[0], tri[1], tri[2], color);
                        d3d.draw_triangle3D(tri[2], tri[1], tri[0], color);
                    },
                    Shape::Cylinder{pos, height, radius} => {
                        d3d.draw_cylinder(pos, *radius, *radius, *height, 16, color);
                    }
                    Shape::CylinderWires{pos, height, radius} => {
                        d3d.draw_cylinder_wires(pos, *radius, *radius, *height, 16, color);
                    }
                }
            }
            
        });

        example.draw_2d(d);
    };
}