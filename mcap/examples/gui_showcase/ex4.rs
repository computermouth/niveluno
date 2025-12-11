use raylib::prelude::*;
use crate::{Example, ToVec3, ToVector3, at_origin, Shape};

pub struct State {
    sphere_pos: Vector3,
}

impl State {
    pub fn new() -> Self {
        Self {
            sphere_pos: at_origin(Vector3::new(0., 5., 2.)),
        }
    }
}

impl Example for State {
    fn update(&mut self, fd: f32, time: f64, reset: bool) -> Vec<(Shape, Color)> {
        let mut out = vec![];

        if reset {
            *self = Self::new();
        }

        // create diagonal floor cube, and push to out vec
        // let Vector3{x, y, z} = at_origin(Vector3::new(0., -2., 2.));
        // let cube_rotation = Matrix::rotate_z(45f32.to_radians());
        // let cube_scale = Matrix::scale(4., 1., 4.);
        // let cube_translate = Matrix::translate(x, y, z);
        // let cube_mat = cube_scale * cube_rotation * cube_translate;
        // out.push((Shape::Cube, cube_mat));

        // // get cube collision data
        // let cube_tris = transform_triangles(&meshes.cube, &cube_mat);

        // // prep parameters for sphere_sweep
        // let position = self.sphere_pos.to_sscv3();
        // let velocity = Vec3::new(0., -5. * fd, 0.);
        // let e_rad = Vec3::new(1.0, 3.0, 1.0);
        // // Note: The ellipsoid will fall/slide slower than a sphere due to Y-axis scaling.
        // // This is correct physics - the ellipsoid space transformation affects all movement.

        // let sweep_res = sphere_sweep(position, velocity, e_rad, &cube_tris, None);
        // let final_pos = sweep_res.position.to_rayv3();

        // // Debug: log all collision info when y is around the stuck position
        // // if self.sphere_pos.y > 95.0 && self.sphere_pos.y < 96.0 {
        // //     eprintln!("Y={:.4} | Contacts: {}, dist: {:.6}, vel_in: {:.6}, vel_out: {:.6}",
        // //         self.sphere_pos.y,
        // //         sweep_res.contacts.len(),
        // //         sweep_res.distance_traveled,
        // //         sweep_res.original_velocity.norm(),
        // //         sweep_res.final_velocity.norm()
        // //     );
        // //     if !sweep_res.contacts.is_empty() {
        // //         for (i, contact) in sweep_res.contacts.iter().enumerate() {
        // //             eprintln!("  C{}: normal=[{:.3}, {:.3}, {:.3}], vloss={:.3}, vafter_len={:.6}",
        // //                 i, contact.normal.x, contact.normal.y, contact.normal.z,
        // //                 contact.velocity_loss, contact.velocity_after.norm());
        // //         }
        // //         panic!();
        // //     }
        // // }

        // self.sphere_pos = final_pos;

        // // now that we have final position, get the final matrix for the sphere
        // let Vector3{x, y, z} = self.sphere_pos;
        // let sphere_scale = Matrix::scale(e_rad.x, e_rad.y, e_rad.z);
        // let sphere_translate = Matrix::translate(x, y, z);
        // let sphere_mat = sphere_scale * sphere_translate;
        // out.push((Shape::Sphere, sphere_mat));

        out
    }

    fn draw_2d(&mut self, mut d: RaylibDrawHandle<'_>) {
        d.draw_rectangle(10, 10, 300, 100, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 300, 100, Color::BLUE);
        d.draw_text(&format!("Ellipsoid Floor Slide"), 20, 20, 20, Color::BLACK);
        d.draw_text(&format!("Ellipsoid.y: {:.4}", self.sphere_pos.y), 20, 40, 20, Color::BLACK);

        d.draw_text(&format!("(R)eset (N)ext (P)revious"), 20, 80, 20, Color::BLACK);
    }
}