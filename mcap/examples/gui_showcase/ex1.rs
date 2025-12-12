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

        let wall = match surf {
            Surface::Wall(w) => w,
            _ => panic!(),
        };

        let mut base = Vector3::new(self.start_pos.x, self.start_pos.y - 1.5, self.start_pos.z);
        let push = check_wall_collision(base.to_mcapv3(), 1., 3., wall);

        match push {
            None => panic!(),
            Some(p) => {
                base += p.to_rayv3();
            }
        }

        self.update_pos.x = base.x;
        self.update_pos.z = base.z;

        // push wires at final position
        out.push((
            Shape::CylinderWires {
                pos: self.update_pos,
                height: 3.,
                radius: 1.,
            },
            Color::GRAY,
        ));

        out
    }

    fn draw_2d(&mut self, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 120, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 120, Color::BLUE);
        d.draw_text(&format!("Basic Wall Collision"), 20, 20, 20, Color::BLACK);
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
