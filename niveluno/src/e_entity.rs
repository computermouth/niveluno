use core::f32;

use raymath::{
    get_ray_collision_mesh, vector3_add, vector3_dot_product, vector3_length, vector3_multiply,
    vector3_negate, vector3_normalize, vector3_scale, vector3_subtract, RayCollision, Vector3,
};

use crate::math::get_padded_ray_collision_mesh;

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

pub fn update_physics(
    acceleration: &mut Vector3,
    velocity: &mut Vector3,
    position: &mut Vector3,
    on_ground: &mut bool,
) {
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

    let mut move_dist = vector3_scale(*velocity, delta_time);

    'out: while vector3_length(move_dist) != 0.0 {
        let decs = g_game::get_decor_instances().unwrap();
        let mut nearest_hit: Option<RayCollision> = None;
        for dec in decs {
            let mesh = dec.get_mesh();
            let mat = dec.get_matrix();
            let ray = raymath::Ray {
                position: *position,
                direction: vector3_normalize(move_dist),
            };

            // check collision against decor wall, with each triangle moved 1.5 units along it's normal
            // let coll = get_padded_ray_collision_mesh(ray, mesh, mat, 1.5);
            let coll = get_ray_collision_mesh(ray, mesh, mat);
            // let coll = get_ray_collision_mesh(ray, mesh, mat);
            if coll.hit && coll.distance <= vector3_length(move_dist) {
                // If nearest collision is not set or this one is closer, update nearest_hit
                if nearest_hit.is_none() || nearest_hit.unwrap().distance > coll.distance {
                    nearest_hit = Some(coll);
                }
            }
        }

        match nearest_hit {
            // TODO
            // None => put AABB at new position, check AABB collision, step back
            // 1./20. * vector3_length(move_dist) if a collision is detected
            None => {
                *position = vector3_add(*position, move_dist);
                break 'out;
            }
            Some(coll) => {
                // Set player position to the collision point plus a small offset along the normal
                *position = vector3_add(
                    coll.point,
                    // if we start going through walls, or clipping through corners, increase this scalar
                    vector3_scale(vector3_negate(coll.normal), 0.0005 + f32::EPSILON),
                );

                if coll.normal.x != 0. {
                    move_dist.x = 0.;
                    velocity.x = 0.;
                }

                if coll.normal.y != 0. {
                    move_dist.y = 0.;
                    velocity.y = 0.;
                    *on_ground = true;
                }

                if coll.normal.z != 0. {
                    move_dist.z = 0.;
                    velocity.z = 0.;
                }
            }
        }
    }
}
