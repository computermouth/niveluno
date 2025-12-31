use std::f64::consts::PI;

use crate::{Args, Example, Shape, ToVec3, ToVector3, at_origin};
use mcap::{closest_point_on_segment, flattened_cylinder_intersects_flattened_triangle};
use raylib::prelude::*;

pub struct State {
    cam_start_pos: Vector3,
    cam_start_tgt: Vector3,
}

impl State {
    pub fn new() -> Self {
        Self {
            cam_start_pos: at_origin(Vector3::new(0., 5., -5.)),
            cam_start_tgt: at_origin(Vector3::zero()),
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

        let origin = at_origin(Vector3::zero());

        let arrow_start = at_origin(Vector3::new(-2., 0., 0.));
        let arrow_end = at_origin(Vector3::new(2., 2., 0.));

        out.push((
            Shape::Arrow {
                start: arrow_start,
                end: arrow_end,
                radius: 0.05,
            },
            Color::RED,
        ));
        out.push((
            Shape::Arrow {
                start: arrow_end,
                end: arrow_start,
                radius: 0.05,
            },
            Color::RED,
        ));

        // vertical path over arrow, should always be solid
        let pos = Vector3::new(origin.x, origin.y + 4. * args.time.sin() as f32, origin.z);
        let radius = 0.4;
        let res = match flattened_cylinder_intersects_flattened_triangle(
            pos.to_mcapv3(),
            radius,
            &[
                arrow_start.to_mcapv3(),
                arrow_start.to_mcapv3(),
                arrow_end.to_mcapv3(),
            ],
        ) {
            Some(_) => (Shape::Sphere { pos, radius }, Color::BLUE),
            None => (
                Shape::SphereWires { pos, radius },
                Color::BLUE.lerp(Color::GRAY, 0.5),
            ),
        };
        out.push(res);

        // closest blue point
        let pos = closest_point_on_segment(
            pos.to_mcapv3(),
            arrow_start.to_mcapv3(),
            arrow_end.to_mcapv3(),
        )
        .to_rayv3();
        out.push((
            Shape::Sphere {
                pos,
                radius: radius / 2.,
            },
            Color::BLUE,
        ));

        // horizontal x path through center, should sometimes be solid
        let pos = Vector3::new(
            origin.x + 4. * ((args.time + PI * 1. / 3.).sin()) as f32,
            origin.y,
            origin.z,
        );
        let radius = 0.4;
        let res = match flattened_cylinder_intersects_flattened_triangle(
            pos.to_mcapv3(),
            radius,
            &[
                arrow_start.to_mcapv3(),
                arrow_start.to_mcapv3(),
                arrow_end.to_mcapv3(),
            ],
        ) {
            Some(_) => (Shape::Sphere { pos, radius }, Color::ORANGE),
            None => (
                Shape::SphereWires { pos, radius },
                Color::ORANGE.lerp(Color::GRAY, 0.5),
            ),
        };
        out.push(res);

        // closest ORANGE point
        let pos = closest_point_on_segment(
            pos.to_mcapv3(),
            arrow_start.to_mcapv3(),
            arrow_end.to_mcapv3(),
        )
        .to_rayv3();
        out.push((
            Shape::Sphere {
                pos,
                radius: radius / 2.,
            },
            Color::ORANGE,
        ));

        // horizontal z path through center, should sometimes be solid
        let pos = Vector3::new(
            origin.x,
            origin.y,
            origin.z + 4. * ((args.time + PI * 2. / 3.).sin()) as f32,
        );
        let radius = 0.4;
        let res = match flattened_cylinder_intersects_flattened_triangle(
            pos.to_mcapv3(),
            radius,
            &[
                arrow_start.to_mcapv3(),
                arrow_start.to_mcapv3(),
                arrow_end.to_mcapv3(),
            ],
        ) {
            Some(_) => (Shape::Sphere { pos, radius }, Color::GREEN),
            None => (
                Shape::SphereWires { pos, radius },
                Color::GREEN.lerp(Color::GRAY, 0.5),
            ),
        };
        out.push(res);

        // closest GREEN point
        let pos = closest_point_on_segment(
            pos.to_mcapv3(),
            arrow_start.to_mcapv3(),
            arrow_end.to_mcapv3(),
        )
        .to_rayv3();
        out.push((
            Shape::Sphere {
                pos,
                radius: radius / 2.,
            },
            Color::GREEN,
        ));

        out
    }

    fn draw_2d(&mut self, _args: Args, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 100, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 100, Color::BLUE);
        d.draw_text(&format!("6. Closest Point"), 20, 20, 20, Color::BLACK);
        d.draw_text(&format!("(S)top (F)lip cam"), 20, 60, 20, Color::BLACK);
        d.draw_text(&format!("(N)ext (P)revious"), 20, 80, 20, Color::BLACK);
    }
}
