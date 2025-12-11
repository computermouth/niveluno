use raylib::prelude::*;
use crate::{Example, ToVec3, ToVector3, at_origin, Shape};

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

        if (time % 1.0) < 0.5 {
            out.push((Shape::Cylinder{pos: self.start_pos, height: 3., radius: 1.}, Color::YELLOW));
        }

        out.push((Shape::CylinderWires{pos: self.start_pos, height: 3., radius: 1.}, Color::GRAY));
        out.push((Shape::Triangle([
            at_origin(Vector3::zero()),
            at_origin(Vector3::new(0., 3., 0.)),
            at_origin(Vector3::new(3., 0., 0.)),
            ]), Color::WHITE));


        // // create floor cube, and push to out vec
        // let Vector3{x, y, z} = at_origin(Vector3::new(0., -5., 2.));
        // let cube_scale = Matrix::scale(10., 1., 10.);
        // let cube_translate = Matrix::translate(x, y, z);
        // let cube_mat = cube_scale * cube_translate;
        // out.push((Shape::Cube, cube_mat));

        // // get cube collision data
        // let cube_tris = transform_triangles(&meshes.cube, &cube_mat);

        // // prep parameters for sphere_sweep
        // let position = self.sphere_pos.to_sscv3();
        // let velocity = Vec3::new(0., -5. * fd, 0.);
        // let e_rad = Vec3::new(1.0, 1.0, 1.0);

        // let sweep_res = sphere_sweep(position, velocity, e_rad, &cube_tris, None);
        // let final_pos = sweep_res.position.to_rayv3();

        // self.sphere_pos = final_pos;

        // // now that we have final position, get the final matrix for the sphere
        // let Vector3{x, y, z} = self.sphere_pos;
        // let sphere_mat = Matrix::translate(x, y, z);
        // out.push((Shape::Sphere, sphere_mat));

        out
    }

    fn draw_2d(&mut self, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 100, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 100, Color::BLUE);
        d.draw_text(&format!("Basic Floor Collision"), 20, 20, 20, Color::BLACK);
        // d.draw_text(&format!("Sphere.y: {:.4}", self.sphere_pos.y), 20, 40, 20, Color::BLACK);

        d.draw_text(&format!("(R)eset (N)ext (P)revious"), 20, 80, 20, Color::BLACK);
    }
}