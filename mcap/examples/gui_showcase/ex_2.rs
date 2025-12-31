use crate::{Args, Example, Shape, ToVec3, ToVector3, at_origin};
use mcap::{Surface, check_circle_tri_collision, get_face_normal};
use raylib::prelude::*;

pub struct State {
    cam_start_pos: Vector3,
    cam_start_tgt: Vector3,
    start_pos: Vector3,
    update_pos: Vector3,
}

impl State {
    pub fn new() -> Self {
        Self {
            cam_start_pos: at_origin(Vector3::new(0., 5., -5.)),
            cam_start_tgt: at_origin(Vector3::zero()),
            start_pos: at_origin(Vector3::new(0., 0., 0.)),
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
            let push = check_circle_tri_collision(new_pos.to_mcapv3(), 1., &wall);

            match push {
                None => panic!(),
                Some((p, _)) => {
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

        // solid final position
        out.push((
            Shape::CylinderWires {
                pos: self.update_pos,
                height: 3.,
                radius: 1.,
            },
            Color::GREEN,
        ));

        let walls = [wall2, wall1];

        let mut new_pos = self.start_pos;
        for wall in walls {
            let push = check_circle_tri_collision(new_pos.to_mcapv3(), 1., &wall);

            match push {
                None => panic!(),
                Some((p, _)) => {
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

        // solid final pos
        out.push((
            Shape::CylinderWires {
                pos: self.update_pos,
                height: 3.,
                radius: 1.,
            },
            Color::ORANGE,
        ));

        out
    }

    fn draw_2d(&mut self, _args: Args, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 140, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 140, Color::BLUE);
        d.draw_text(
            &format!("2. Corner Wall Collision"),
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
