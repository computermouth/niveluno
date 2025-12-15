use crate::{Example, Shape, ToVec3, ToVector3, at_origin};
use mcap::{Surface, Wall, check_wall_collision, get_face_normal};
use raylib::prelude::*;

pub struct State {
    start_pos: Vector3,
    update_pos: Vector3,
}

impl State {
    pub fn new() -> Self {
        Self {
            start_pos: at_origin(Vector3::zero()),
            update_pos: at_origin(Vector3::zero()),
        }
    }
}

impl Example for State {
    fn update(&mut self, fd: f32, time: f64, reset: bool) -> Vec<(Shape, Color)> {
        let mut out = vec![];

        if reset {
            *self = Self::new();
        }

        // blinking start position
        if (time % 1.0) < 0.5 {
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
            at_origin(Vector3::new(-1., 0., -3.)),
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

        let wall1 = match surf1 {
            Surface::Wall(w) => w,
            _ => panic!(),
        };

        let wall2 = match surf2 {
            Surface::Wall(w) => w,
            _ => panic!(),
        };

        let walls = [wall1, wall2];

        let mut new_pos = self.start_pos;
        for wall in walls {
            let push = check_wall_collision(self.start_pos.to_mcapv3(), 1., 3., &wall);

            match push {
                None => panic!(),
                Some(p) => {
                    new_pos += p.to_rayv3();
                }
            }

            out.push((
                Shape::CylinderWires {
                    pos: new_pos,
                    height: 3.,
                    radius: 1.,
                },
                Color::GRAY,
            ));
        }

        self.update_pos = new_pos;

        // blinking final position
        if (time % 1.0) > 0.5 {
            out.push((
                Shape::Cylinder {
                    pos: self.update_pos,
                    height: 3.,
                    radius: 1.,
                },
                Color::GREEN,
            ));
        }

        out
    }

    fn draw_2d(&mut self, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 120, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 120, Color::BLUE);
        d.draw_text(&format!("Corner Wall Collision"), 20, 20, 20, Color::BLACK);
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
