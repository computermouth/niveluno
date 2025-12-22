use std::f32;

use mcap::{Surface, Vec3, check_circle_tri_collision};
use raylib::prelude::*;

mod ex1;
mod ex2;
mod ex3;
mod ex4;
mod ex5;
mod ex6;
mod ex7;
mod ex8;

trait ToVec3 {
    fn to_mcapv3(&self) -> Vec3;
}

trait ToVector3 {
    fn to_rayv3(&self) -> Vector3;
}

impl ToVec3 for Vector3 {
    fn to_mcapv3(&self) -> Vec3 {
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
    Cylinder {
        pos: Vector3,
        height: f32,
        radius: f32,
    },
    CylinderWires {
        pos: Vector3,
        height: f32,
        radius: f32,
    },
    Triangle([Vector3; 3]),
    Arrow {
        start: Vector3,
        end: Vector3,
        radius: f32,
    },
    Sphere {
        pos: Vector3,
        radius: f32,
    },
    SphereWires {
        pos: Vector3,
        radius: f32,
    },
}

#[derive(Copy, Clone)]
struct Args {
    fd: f32,
    time: f64,
    reset: bool,
}

trait Example {
    fn camera_start_pos(&mut self) -> Vector3;
    fn camera_start_tgt(&mut self) -> Vector3;
    fn update(&mut self, args: Args) -> Vec<(Shape, Color)>;
    fn draw_2d(&mut self, args: Args, d: RaylibDrawHandle<'_>);
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("gui showcase").build();

    let mut camera = Camera3D::perspective(
        at_origin(Vector3::new(0., 5., -5.)),
        at_origin(Vector3::zero()),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
    );

    let mut example_id: usize = 0;
    let mut example: Box<dyn Example> = Box::new(ex1::State::new());

    let mut cam_angle = Vector2::new(0., 0.);

    let mut cam_dir = 1.0;
    let mut cam_mov = true;

    while !rl.window_should_close() {
        let fd = rl.get_frame_time();
        let time = rl.get_time();

        let cam_stop = rl.is_key_pressed(KeyboardKey::KEY_S);
        let cam_flip = rl.is_key_pressed(KeyboardKey::KEY_F);
        let reset = rl.is_key_pressed(KeyboardKey::KEY_R);
        let prev = rl.is_key_pressed(KeyboardKey::KEY_P);
        let next = rl.is_key_pressed(KeyboardKey::KEY_N);
        let change = prev as i8 - next as i8;

        if cam_stop {
            cam_mov = !cam_mov;
        }

        if cam_flip {
            cam_dir *= -1.;
        }

        cam_angle.x = (360.
            + (cam_angle.x + (cam_dir * cam_mov as i32 as f32) * (rl.get_frame_time() / 0.01667)))
            % 360.;

        let cam_radius = ((camera.position.x - camera.target.x).powi(2)
            + (camera.position.z - camera.target.z).powi(2))
        .sqrt();
        camera.position.x =
            camera.target.x + cam_radius * (cam_angle.x * f32::consts::PI / 180.).cos();
        camera.position.z =
            camera.target.z + cam_radius * (cam_angle.x * f32::consts::PI / 180.).sin();

        let mut d = rl.begin_drawing(&thread);

        const NUM_EXAMPLES: usize = 8;
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
                4 => Box::new(ex5::State::new()),
                5 => Box::new(ex6::State::new()),
                6 => Box::new(ex7::State::new()),
                7 => Box::new(ex8::State::new()),
                _ => panic!(),
            };

            camera.position = example.camera_start_pos();
            camera.target = example.camera_start_tgt();
        }

        let args = Args { fd, time, reset };

        let draws = example.update(args);

        d.clear_background(Color::new(16, 16, 32, 255));
        d.draw_mode3D(camera, |mut d3d, _| {
            for (shape, color) in &draws {
                match shape {
                    Shape::Triangle(tri) => {
                        d3d.draw_triangle3D(tri[2], tri[1], tri[0], color.brightness(-0.5));
                        d3d.draw_triangle3D(tri[0], tri[1], tri[2], color);
                    }
                    Shape::Cylinder {
                        pos,
                        height,
                        radius,
                    } => {
                        d3d.draw_cylinder(pos, *radius, *radius, *height, 16, color);
                    }
                    Shape::CylinderWires {
                        pos,
                        height,
                        radius,
                    } => {
                        d3d.draw_cylinder_wires(pos, *radius, *radius, *height, 16, color);
                    }
                    Shape::Arrow { start, end, radius } => {
                        let direction = (*end - *start).normalized();
                        let arrow_len = start.distance_to(*end);

                        d3d.draw_cylinder_ex(
                            start,
                            *start + direction.scale_by(arrow_len * 0.8),
                            *radius,
                            *radius,
                            16,
                            color,
                        );
                        d3d.draw_cylinder_ex(
                            *start + direction.scale_by(arrow_len * 0.8),
                            end,
                            *radius * 2.,
                            0.,
                            16,
                            color,
                        );
                    }
                    Shape::Sphere { pos, radius } => {
                        d3d.draw_sphere(pos, *radius, color);
                    }
                    Shape::SphereWires { pos, radius } => {
                        d3d.draw_sphere_wires(pos, *radius, 7, 7, color);
                    }
                }
            }
        });

        example.draw_2d(args, d);
    }
}
