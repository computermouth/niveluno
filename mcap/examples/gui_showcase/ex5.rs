use crate::{Example, Shape, ToVec3, ToVector3, at_origin, Args};
use mcap::{Surface, Wall, check_wall_collision, get_face_normal, get_step_push};
use raylib::prelude::*;

pub struct State {
    start_pos: Vector3,
    velocity: Vector3,
    update_pos: Vector3,
}

impl State {
    pub fn new() -> Self {
        Self {
            start_pos: at_origin(Vector3::new(-3., 0., -3.)),
            velocity: Vector3::new(5., 0., 5.),
            update_pos: at_origin(Vector3::zero()),
        }
    }
}

impl Example for State {
    fn update(&mut self, args: Args) -> Vec<(Shape, Color)> {
        let mut out = vec![];

        out.push((
            Shape::Sphere {
                pos: at_origin(Vector3::zero()),
                radius: 0.1,
            },
            Color::GREEN,
        ));

        *self = Self::new();

        // blinking start position
        if (args.time % 1.0) < 0.5 {
            out.push((
                Shape::Cylinder {
                    pos: self.start_pos,
                    height: 3.,
                    radius: 1.,
                },
                Color::YELLOW,
            ));
        }

        let tpos1 = [
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(0., 3., 0.)),
            at_origin(Vector3::new(3., 0., 0.)),
        ];

        let tpos2 = [
            at_origin(Vector3::new(0., 0., 3.)),
            at_origin(Vector3::new(0., 3., 0.)),
            at_origin(Vector3::zero()),
        ];

        // push triangles at origin
        out.push((Shape::Triangle(tpos1), Color::WHITE));
        out.push((Shape::Triangle(tpos2), Color::PINK));

        let surf1 = Surface::new(
            [
                tpos1[0].to_mcapv3(),
                tpos1[1].to_mcapv3(),
                tpos1[2].to_mcapv3(),
            ],
            get_face_normal(
                tpos1[0].to_mcapv3(),
                tpos1[1].to_mcapv3(),
                tpos1[2].to_mcapv3(),
            ),
        );

        let surf2 = Surface::new(
            [
                tpos2[0].to_mcapv3(),
                tpos2[1].to_mcapv3(),
                tpos2[2].to_mcapv3(),
            ],
            get_face_normal(
                tpos2[0].to_mcapv3(),
                tpos2[1].to_mcapv3(),
                tpos2[2].to_mcapv3(),
            ),
        );

        let surfs = [surf1, surf2];

        let iterations = 8;
        let v_chunk = self.velocity.scale_by(1. / iterations as f32);

        let mut new_pos = self.start_pos;

        for i in 0..iterations {
            let out_diff = get_step_push(new_pos.to_mcapv3(), v_chunk.to_mcapv3(), 1., 3., &surfs);

            let diff = match out_diff {
                Some(v) => v.to_rayv3(),
                None => v_chunk,
            };

            new_pos += diff;

            // push wires at updated position
            out.push((
                Shape::CylinderWires {
                    pos: new_pos,
                    height: 3.,
                    radius: 1.,
                },
                Color::YELLOW.lerp(Color::GREEN, i as f32 / iterations as f32),
            ));
        }

        self.update_pos = new_pos;

        // blinking final position
        if (args.time % 1.0) > 0.5 {
            out.push((
                Shape::Cylinder {
                    pos: self.update_pos,
                    height: 3.,
                    radius: 1.,
                },
                Color::GREEN,
            ));
        }

        // push wires at original intended position
        out.push((
            Shape::CylinderWires {
                pos: self.start_pos + self.velocity,
                height: 3.,
                radius: 1.,
            },
            Color::RED,
        ));

        out.push((
            Shape::Arrow {
                start: self.start_pos,
                end: self.start_pos + self.velocity,
                radius: 0.1,
            },
            Color::RED,
        ));

        out
    }

    fn draw_2d(&mut self, args: Args, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 140, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 140, Color::BLUE);
        d.draw_text(
            &format!("5. Convex Stepped Corner"),
            20,
            20,
            20,
            Color::BLACK,
        );
        d.draw_text(
            &format!(
                "p1: {:.1} {:.1} {:.1}",
                self.start_pos.x, self.start_pos.y, self.start_pos.z
            ),
            20,
            40,
            20,
            Color::BLACK,
        );
        d.draw_text(
            &format!(
                "p2: {:.1} {:.1} {:.1}",
                self.update_pos.x, self.update_pos.y, self.update_pos.z
            ),
            20,
            60,
            20,
            Color::BLACK,
        );

        d.draw_text(
            &format!("(R)eset (N)ext (P)revious"),
            20,
            100,
            20,
            Color::BLACK,
        );
    }
}
