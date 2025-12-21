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

        let mut surfs = vec![];

        // wall
        let tpos = [
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(0., 3., 0.)),
            at_origin(Vector3::new(3., 0., 0.)),
        ];

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
        surfs.push(surf);

        // floor
        let tpos = [
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(0., 0., 3.)),
            at_origin(Vector3::new(3., 0., 0.)),
        ];

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
        surfs.push(surf);

        // cieling
        let tpos = [
            at_origin(Vector3::new(0., 0., -3.)),
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(-3., 0., 0.)),
        ];

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
        surfs.push(surf);

        // slide
        let tpos = [
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(-3., 0., 0.)),
            at_origin(Vector3::new(0., -3., 1.)),
        ];

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
        surfs.push(surf);

        for surf in surfs {
            match surf {
                Surface::Wall(triangle) => {
                    let v = triangle.verts();
                    let v = [v[0].to_rayv3(), v[1].to_rayv3(), v[2].to_rayv3()];
                    out.push((Shape::Triangle(v), Color::GREEN));
                }
                Surface::Floor(triangle) => {
                    let v = triangle.verts();
                    let v = [v[0].to_rayv3(), v[1].to_rayv3(), v[2].to_rayv3()];
                    out.push((Shape::Triangle(v), Color::RED));
                }
                Surface::Slide(triangle) => {
                    let v = triangle.verts();
                    let v = [v[0].to_rayv3(), v[1].to_rayv3(), v[2].to_rayv3()];
                    out.push((Shape::Triangle(v), Color::BLUE));
                }
                Surface::Cieling(triangle) => {
                    let v = triangle.verts();
                    let v = [v[0].to_rayv3(), v[1].to_rayv3(), v[2].to_rayv3()];
                    out.push((Shape::Triangle(v), Color::YELLOW));
                }
            }
        }

        out
    }

    fn draw_2d(&mut self, args: Args, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 100, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 100, Color::BLUE);
        d.draw_text(
            &format!("7. Triangle Categorization"),
            20,
            20,
            20,
            Color::BLACK,
        );
        d.draw_text(&format!("(S)top (F)lip cam"), 20, 60, 20, Color::BLACK);
        d.draw_text(&format!("(N)ext (P)revious"), 20, 80, 20, Color::BLACK);
    }
}
