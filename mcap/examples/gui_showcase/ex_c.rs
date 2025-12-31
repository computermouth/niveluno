use crate::{Args, Example, Shape, ToVec3, ToVector3, at_origin};
use mcap::{Surface, check_circle_tri_collision, get_face_normal, solve_plane_y};
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
            cam_start_pos: at_origin(Vector3::new(0., 3., -5.)),
            cam_start_tgt: at_origin(Vector3::zero()),
            start_pos: at_origin(Vector3::zero()),
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

        // floor tri
        let tpos = [
            at_origin(Vector3::new(0., 0., -5.)),
            at_origin(Vector3::new(-3., 0., 0.)),
            at_origin(Vector3::new(3., 0., 0.)),
        ];

        let step = Vector3::new(0., 0., 5.);

        out.push((Shape::Triangle(tpos), Color::WHITE));
        // floor arrow
        let start = tpos[0];
        let end = start + step;
        out.push((
            Shape::Arrow {
                start,
                end,
                radius: 0.2,
            },
            Color::RED,
        ));

        // sloped floor tri
        let tpos = [
            at_origin(Vector3::new(-3., 0., 0.)),
            at_origin(Vector3::new(0., 2., 5.)),
            at_origin(Vector3::new(3., 0., 0.)),
        ];

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

        let w = match surf {
            Surface::Wall(w) => w,
            Surface::Cieling(w) => w,
            Surface::Floor(w) => w,
            Surface::Slide(w) => w,
        };

        let step1 = Vector3::new(0., 0., 5.).to_mcapv3();
        let step2 = step1 - (w.normal() * step1.dot(w.normal()));
        // eprintln!("step1: {:?} step2: {:?}", step1, step2);

        // sloped arrow
        out.push((
            Shape::Arrow {
                start: at_origin(Vector3::zero()),
                end: at_origin(step2.to_rayv3()),
                radius: 0.2,
            },
            Color::RED,
        ));

        out
    }

    fn draw_2d(&mut self, _args: Args, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 140, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 140, Color::BLUE);
        d.draw_text(&format!("C. Project V on normal"), 20, 20, 20, Color::BLACK);
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
