use crate::{Args, Example, Shape, ToVec3, ToVector3, at_origin};
use mcap::{Surface, get_face_normal};
use raylib::prelude::*;

pub struct State {
    cam_start_pos: Vector3,
    cam_start_tgt: Vector3,
    current_surf: Surface,
}

impl State {
    pub fn new() -> Self {
        let tpos = [
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(0., 0., 1.)),
            at_origin(Vector3::new(1., 0., 0.)),
        ];

        Self {
            cam_start_pos: at_origin(Vector3::new(0., 5., -5.)),
            cam_start_tgt: at_origin(Vector3::zero()),
            current_surf: Surface::new(
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
            ),
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

        let tpos = [
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(0., 0., 1.)),
            at_origin(Vector3::new(1., 0., 0.)),
        ];

        // push triangle at origin
        out.push((Shape::Triangle(tpos), Color::WHITE));

        let y = ((args.time / 2.).cos() * 5.) as f32;
        let z = ((args.time / 2.).sin() * 5.) as f32;

        // triangle with rotating second point
        let tpos = [
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(0., y, z)),
            at_origin(Vector3::new(5., 0., 0.)),
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

        // for updating 2d ui
        self.current_surf = surf;

        // push normal arrow
        let center = (tpos[0].to_mcapv3() + tpos[1].to_mcapv3() + tpos[2].to_mcapv3()) / 3.;
        let norm = get_face_normal(
            tpos[0].to_mcapv3(),
            tpos[1].to_mcapv3(),
            tpos[2].to_mcapv3(),
        );
        out.push((
            Shape::Arrow {
                start: center.to_rayv3(),
                end: (center + norm).to_rayv3(),
                radius: 0.1,
            },
            Color::GRAY,
        ));

        out
    }

    fn draw_2d(&mut self, args: Args, mut d: RaylibDrawHandle<'_>) {
        let t: Vector3;

        let surf_str = match self.current_surf {
            Surface::Wall(triangle) => {
                t = triangle.normal().to_rayv3();
                "wall"
            }
            Surface::Floor(triangle) => {
                t = triangle.normal().to_rayv3();
                "floor"
            }
            Surface::Slide(triangle) => {
                t = triangle.normal().to_rayv3();
                "slide"
            }
            Surface::Cieling(triangle) => {
                t = triangle.normal().to_rayv3();
                "cieling"
            }
        };

        d.draw_rectangle(10, 10, 300, 140, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 140, Color::BLUE);
        d.draw_text(&format!("8. Moving T-cat"), 20, 20, 20, Color::BLACK);
        d.draw_text(&format!("type: {}", surf_str), 20, 40, 20, Color::BLACK);
        d.draw_text(
            &format!("norm: {:.1} {:.1} {:.1}", t.x, t.y, t.z),
            20,
            60,
            20,
            Color::BLACK,
        );
        d.draw_text(&format!("(S)top (F)lip cam"), 20, 100, 20, Color::BLACK);
        d.draw_text(&format!("(N)ext (P)revious"), 20, 120, 20, Color::BLACK);
    }
}
