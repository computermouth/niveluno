use crate::{Args, Example, Shape, ToVec3, ToVector3, at_origin};
use glam::Vec3;
use mcap::{
    Surface, find_floor_height_m64, get_face_normal,
};
use raylib::prelude::*;
use mcap::scrap as mcap;

pub struct State {
    cam_start_pos: Vector3,
    cam_start_tgt: Vector3,
    start_pos: Vector3,
    update_pos: Vector3,
}

impl State {
    pub fn new() -> Self {
        Self {
            cam_start_pos: at_origin(Vector3::new(0., 1.5, -5.)),
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
        out.push((Shape::Triangle(tpos), Color::WHITE));

        let surf1 = Surface::new(
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

        // floor tri
        let tpos = [
            at_origin(Vector3::new(0., 2., -5.)),
            at_origin(Vector3::new(-3., 2., 0.)),
            at_origin(Vector3::new(3., 2., 0.)),
        ];
        out.push((Shape::Triangle(tpos), Color::WHITE));

        let surf2 = Surface::new(
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

        let surfs = [&surf1, &surf2];
        let snap = 1.2;

        let center = (tpos[0].to_mcapv3() + tpos[1].to_mcapv3() + tpos[2].to_mcapv3()) / 3.;
        // start exactly between, and bob with time.sin()
        let base = center + Vec3::ZERO.with_y(-1. + 3. * args.time.sin() as f32);

        out.push((
            Shape::CylinderWires {
                pos: base.to_rayv3(),
                height: 3.,
                radius: 1.,
            },
            Color::YELLOW,
        ));

        if let Some((a, b)) = find_floor_height_m64(base, snap, &surfs) {
            let v = a.verts();
            out.push((
                Shape::Triangle([v[0].to_rayv3(), v[1].to_rayv3(), v[2].to_rayv3()]),
                Color::RED,
            ));

            out.push((
                Shape::Cylinder {
                    pos: base.with_y(b).to_rayv3(),
                    height: 3.,
                    radius: 0.8,
                },
                Color::GRAY,
            ));
        }

        out
    }

    fn draw_2d(&mut self, _args: Args, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 140, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 140, Color::BLUE);
        d.draw_text(&format!("D. floor_height"), 20, 20, 20, Color::BLACK);
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
