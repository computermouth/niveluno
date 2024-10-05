use core::f32;

use raymath::{
    get_ray_collision_mesh, vector3_add, vector3_distance, vector3_multiply, vector3_negate,
    vector3_normalize, vector3_scale, vector3_subtract, Vector3,
};

use crate::{g_game, time};

pub trait EntityInstance {
    // name: String,

    fn update(&mut self);
    //     fn update_physics(&mut self);
    //     fn collides(&mut self);
    //     fn did_collide(&mut self);
    //     fn did_collide_with_entity(&mut self);
    fn draw_model(&mut self);
    //     fn spawn_particles(&mut self);
    //     fn recv_damage(&mut self);
    //     fn play_sound(&mut self);
    //     fn kill(&mut self);
    //     fn pickup(&mut self);
    //     fn set_state(&mut self);
    //     fn spawn_projectile(&mut self);
    //     fn attack(&mut self);
}

const GRAVITY: f32 = 1.0;
const FRICTION: f32 = 10.0;

pub fn update_physics(acceleration: &mut Vector3, velocity: &mut Vector3, position: &mut Vector3) {
    // Apply Gravity
    acceleration.y = -36. * GRAVITY;

    let delta_time = time::get_delta_time().unwrap() as f32;

    // Integrate acceleration & friction into velocity
    let df = 1.0f32.min(FRICTION * delta_time);
    let af = vector3_scale(*acceleration, delta_time);
    let vf = vector3_multiply(
        *velocity,
        Vector3 {
            x: df,
            y: 0.,
            z: df,
        },
    );
    *velocity = vector3_add(*velocity, vector3_subtract(af, vf));

    let move_dist = vector3_scale(*velocity, delta_time);

    let mut out_pos = vector3_add(*position, move_dist);

    let decs = g_game::get_decor_instances().unwrap();
    for dec in decs {
        let mesh = dec.get_mesh();
        let mat = dec.get_matrix();
        let ray = raymath::Ray {
            position: *position,
            direction: vector3_normalize(move_dist),
        };

        let coll = get_ray_collision_mesh(ray, mesh, mat);
        if coll.hit && coll.distance <= 1. + f32::EPSILON {
            out_pos = vector3_add(coll.point, vector3_negate(coll.normal));
        }
    }

    *position = out_pos;
}
