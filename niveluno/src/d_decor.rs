// would rather match on sum type, but calling
// other_e::func seems hairy. maybe if it took a copy
// of other_e, modified, bubbled up to a buffer to replace?

use crate::{g_game, math};
use raymath::{self, vector3_distance, vector3_normalize, vector3_subtract, Vector3};

// todo, something like this??
/*
pub trait DecorInstance {
    fn update(&mut self);
    fn draw_model(&mut self);

    // Use the methods from `Decor`
    fn get_base(&self) -> &map::Decor;

    // Default implementations using the methods on `Base`
    fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        self.get_base().get_mesh()
    }

    fn get_matrix(&self) -> raymath::Matrix {
        self.get_base().get_matrix()
    }
}
*/

pub trait DecorInstance {
    fn update(&mut self);
    fn draw_model(&mut self);
    fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]>;
    fn get_matrix(&self) -> raymath::Matrix;
}

pub fn pos_is_visible(cam_pos: Vector3, point: Vector3) -> bool {
    let decs = g_game::get_decor_instances().unwrap();
    let dir = vector3_normalize(vector3_subtract(point, cam_pos));
    let distance = vector3_distance(cam_pos, point);
    let ray = raymath::Ray {
        position: cam_pos,
        direction: dir,
    };

    // find nearest decor collision
    for dec in decs {
        let mesh = dec.get_mesh();
        let mat = dec.get_matrix();

        let coll = math::get_ray_collision_mesh(ray, mesh, mat);
        // collides before reaching point
        if coll.hit && coll.distance < distance {
            return true;
        }
    }

    false
}
