use crate::{Args, Example, Shape, ToVec3, ToVector3, at_origin};
use mcap::{Surface, get_face_normal, get_step_push};
use raylib::prelude::*;

pub struct State {
    cam_start_pos: Vector3,
    cam_start_tgt: Vector3,
    start_pos: Vector3,
    velocity: Vector3,
    update_pos: Vector3,
}

impl State {
    pub fn new() -> Self {
        Self {
            cam_start_pos: at_origin(Vector3::new(0., 5., -5.)),
            cam_start_tgt: at_origin(Vector3::zero()),
            start_pos: at_origin(Vector3::new(-2., 0., -2.)),
            velocity: Vector3::new(3., 0., 3.),
            update_pos: at_origin(Vector3::zero()),
        }
    }
}

impl Example for State {
    fn camera_start_pos(&mut self) -> Vector3 {
        self.cam_start_pos
    }

    fn camera_start_tgt(&mut self) -> Vector3 {
        self.cam_start_tgt
    }

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

        let tpos = [
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(0., 3., 0.)),
            at_origin(Vector3::new(3., 0., 0.)),
        ];

        // push triangle at origin
        out.push((Shape::Triangle(tpos), Color::WHITE));

        let surf = Surface::new(
            [
                tpos[0].to_mcapv3(),
                tpos[1].to_mcapv3(),
                tpos[2].to_mcapv3(),
            ],
            get_face_normal(
                tpos[0].to_mcapv3(),
                tpos[1].to_mcapv3(),
                tpos[2].to_mcapv3(),
            ),
        );

        let iterations = 8;
        let v_chunk = self.velocity.scale_by(1. / iterations as f32);

        let mut new_pos = self.start_pos;

        for i in 0..iterations {
            let res = get_step_push(
                new_pos.to_mcapv3(),
                v_chunk.to_mcapv3(),
                1.,
                3.,
                0.4,
                &[surf],
            );
            new_pos = res.final_pos.to_rayv3();

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
            &format!("3. Stepped Wall Collision"),
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
        d.draw_text(&format!("(S)top (F)lip cam"), 20, 100, 20, Color::BLACK);
        d.draw_text(&format!("(N)ext (P)revious"), 20, 120, 20, Color::BLACK);
    }
}
