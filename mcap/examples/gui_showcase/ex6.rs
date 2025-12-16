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

        let origin = at_origin(Vector3::zero());

        out.push((
            Shape::SphereWires {
                pos: Vector3::new(origin.x, origin.y + 2. * args.time.sin() as f32, origin.z),
                radius: 0.1,
            },
            Color::GRAY
        ));

        out
    }

    fn draw_2d(&mut self, args: Args, mut d: RaylibDrawHandle<'_>) {
        d.draw_circle_lines(320, (240. + (args.time * 2.).sin() * 100.) as i32, 10., Color::GRAY);

        d.draw_rectangle(10, 10, 300, 140, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 140, Color::BLUE);
        d.draw_text(&format!("6. Closest Point"), 20, 20, 20, Color::BLACK);
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
