use crate::{Args, Example, Shape, ToVec3, ToVector3, at_origin};
use glam::Vec3;
use mcap::{HotDog, Surface, get_face_normal, get_step_push};
use raylib::prelude::*;

pub struct State {
    cam_start_pos: Vector3,
    cam_start_tgt: Vector3,
    start_pos: Vector3,
    velocity: Vector3,
    update_pos: Vector3,
    last_good_angle: f32,
}

impl State {
    pub fn new() -> Self {
        Self {
            cam_start_pos: at_origin(Vector3::new(0., 10., -10.)),
            cam_start_tgt: at_origin(Vector3::zero()),
            start_pos: at_origin(Vector3::new(0., 0., -1.006)),
            velocity: Vector3::new(0., 0., 0.0061),
            update_pos: at_origin(Vector3::zero()),
            last_good_angle: 0.,
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

        // looks like
        // angle = .0000038 / radius
        //
        // .05  -> .000076
        // .04  -> .000095
        // .03  -> .00012
        // .02  -> .00019
        // .01  -> .00038
        // .005 -> .00076
        // .001 -> .0039
        //
        // tests with exaggerated minimum
        // angle = .000005 / radius
        //
        // .05  -> .0001
        // .04  -> .000125
        // .03  -> .00016
        // .02  -> .00025
        // .01  -> .0005
        // .005 -> .0001
        // .001 -> .005

        let f = 500.;

        self.start_pos.x += (args.time.sin() * f) as f32;
        self.velocity.x += -(args.time.sin() * f * 2.) as f32;
        // self.start_pos.x += (1. * f) as f32;
        // self.velocity.x += -(1. * f * 2.) as f32;

        // blinking start position
        out.push((
            Shape::CylinderWires {
                pos: self.start_pos,
                height: 3.,
                radius: 1.,
            },
            Color::YELLOW,
        ));

        let tpos1 = [
            at_origin(Vector3::new(-6000., 0., 0.)),
            at_origin(Vector3::new(-6000., 3., 0.)),
            at_origin(Vector3::new(6000., 0., 0.)),
        ];

        let tpos2 = [
            at_origin(Vector3::new(6000., 0., 0.)),
            at_origin(Vector3::new(-6000., 3., 0.)),
            at_origin(Vector3::new(6000., 3., 0.)),
        ];

        // push triangles at origin
        out.push((Shape::Triangle(tpos1), Color::WHITE));
        out.push((Shape::Triangle(tpos2), Color::WHITE));

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

        let surfs = [&surf1, &surf2];

        let src = self.start_pos;
        let dst = self.start_pos + self.velocity;
        let hotdog = HotDog::new(
            src.to_mcapv3(), 
            dst.to_mcapv3(),
            1.0,
            self.velocity.normalized().to_mcapv3());

        let hdc1 = hotdog.check_walls_c2(&surfs).unwrap();

        // stopped pos
        out.push((
            Shape::CylinderWires {
                pos: Vector3::new(hdc1.dest_xz.x, src.y, hdc1.dest_xz.y),
                height: 3.,
                radius: 1.,
            },
            Color::GREEN,
        ));

        // next pos
        out.push((
            Shape::CylinderWires {
                pos: Vector3::new(hdc1.dest_xz.x + hdc1.next_move.x, src.y, hdc1.dest_xz.y + hdc1.next_move.y),
                height: 3.,
                radius: 1.,
            },
            Color::PINK,
        ));

        let hd2_src = Vec3::new(hdc1.dest_xz.x + hdc1.next_move.x, src.y, hdc1.dest_xz.y + hdc1.next_move.y);
        let hd2_dst = hd2_src + Vec3::new(hdc1.next_move.x, 0., hdc1.next_move.y);
        let hotdog2 = HotDog::new(
            hd2_src,
            hd2_dst,
            1.,
            self.velocity.normalized().to_mcapv3()
        );

        let hdc2 = hotdog2.check_walls_c2(&surfs);
        if let Some(hdc2) = hdc2 {
            // stopped pos
            out.push((
                Shape::CylinderWires {
                    pos: Vector3::new(hdc2.dest_xz.x, src.y, hdc2.dest_xz.y),
                    height: 3.,
                    radius: 1.,
                },
                Color::FUCHSIA,
            ));

            let hd3_src = Vec3::new(hdc2.dest_xz.x + hdc2.next_move.x, src.y, hdc2.dest_xz.y + hdc2.next_move.y);
            let hd3_dst = hd3_src + Vec3::new(hdc2.next_move.x, 0., hdc2.next_move.y);
            let hotdog3 = HotDog::new(
                hd3_src,
                hd3_dst,
                1.,
                self.velocity.normalized().to_mcapv3()
            );
            assert!(hotdog3.check_walls_c2(&surfs).is_none());
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

    fn draw_2d(&mut self, _args: Args, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 140, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 140, Color::BLUE);
        d.draw_text(
            &format!("F. Projected skin backstep"),
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
