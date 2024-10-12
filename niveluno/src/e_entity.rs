use core::{f32, panic};
use std::collections::HashSet;

use raymath::{
    get_ray_collision_mesh, vector3_add, vector3_divide, vector3_dot_product, vector3_length,
    vector3_multiply, vector3_negate, vector3_normalize, vector3_scale, vector3_subtract,
    BoundingBox, RayCollision, Vector3,
};

use crate::math::{
    get_padded_ray_collision_mesh, get_sat_aabb_collision_mesh, mesh_tranform, sat_aabb_tri,
    vec3_face_normal, vector3_transform,
};

use crate::{g_game, text, time};

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

// Function to adjust velocity based on the wall's normal
fn adjust_velocity_for_wall(velocity: Vector3, wall_normal: Vector3) -> Vector3 {
    // Project the velocity onto the wall normal
    let dot_product = vector3_dot_product(velocity, wall_normal);
    let proj_velocity_on_normal = vector3_scale(wall_normal, dot_product);

    // Subtract the projection from the original velocity
    vector3_subtract(velocity, proj_velocity_on_normal)
}

pub fn update_physics_2(
    acceleration: &mut Vector3,
    velocity: &mut Vector3,
    position: &mut Vector3,
    on_ground: &mut bool,
    height: f32,
    width: f32,
) {
    // Apply Gravity
    acceleration.y = -30. * GRAVITY;

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

    let steps = 16;

    let move_dist = vector3_scale(*velocity, delta_time);
    let mut step_size = vector3_scale(move_dist, 1. / steps as f32);

    let mut last_aabb = BoundingBox {
        min: Vector3::new(0., 0., 0.),
        max: Vector3::new(0., 0., 0.),
    };
    let mut last_floor = Vector3::new(0., 0., 0.);

    let decs = g_game::get_decor_instances().unwrap();

    *on_ground = false;

    for _ in 0..steps {
        // move one step
        let mut tmp_pos = vector3_add(*position, step_size);

        // FLOORS

        // cast ray down from top of head, should be fine so long as we're never moving
        // more than +2m/frame downward
        let ray = raymath::Ray {
            position: Vector3::new(tmp_pos.x, tmp_pos.y + height, tmp_pos.z),
            direction: Vector3::new(0., -1., 0.),
        };

        // find nearest floor collision
        // todo, only loop over floors (normal.y >= .707)
        let mut floor_hit: Option<RayCollision> = None;
        for dec in decs {
            let mesh = dec.get_mesh();
            let mat = dec.get_matrix();

            let coll = get_ray_collision_mesh(ray, mesh, mat);
            // collides, and is closest, and is less than 45 degree slope
            if coll.hit && coll.distance < height && -coll.normal.y >= 0.607 {
                // If nearest collision is not set or this one is closer, update nearest_hit
                if floor_hit.is_none() || floor_hit.unwrap().distance > coll.distance {
                    floor_hit = Some(coll);
                }
            }
        }

        // If we hit the floor, step back and stop vertical movement
        if floor_hit.is_some() {
            velocity.y = 0.; // Stop vertical velocity
            let floor_collision = floor_hit.unwrap();
            let floor_normal = floor_collision.normal;

            // Project step_size onto the floor's plane
            let dot = vector3_dot_product(step_size, floor_normal);
            let correction = vector3_scale(floor_normal, dot);

            // Remove the component of the step_size in the direction of the floor normal
            step_size = vector3_subtract(step_size, correction);

            // Ensure the player stays on or above the floor to prevent sinking
            let floor_y = floor_collision.point.y;
            if tmp_pos.y < floor_y {
                tmp_pos.y = floor_y; // Adjust player position to stay on the ground
            }

            // Mark the player as on the ground
            *on_ground = true;
            last_floor = floor_normal;
        }

        // WALLS
        let mut aabb = BoundingBox {
            min: Vector3::new(tmp_pos.x - width / 2., tmp_pos.y, tmp_pos.z - width / 2.),
            max: Vector3::new(
                tmp_pos.x + width / 2.,
                tmp_pos.y + height,
                tmp_pos.z + width / 2.,
            ),
        };

        // raise the aabb.min.y proportionally to the slope,
        // this prevents the player from intersecting walls next to the slope,
        // and getting pushed off the ledge
        match last_floor {
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            } => {
                aabb.min.y += height / 100.;
            }
            _ => {
                let slope_angle = vector3_dot_product(last_floor, Vector3::new(0.0, 1.0, 0.0));
                let slope_adjustment = width * (1.0 - slope_angle).abs();
                aabb.min.y = tmp_pos.y + slope_adjustment;
            }
        };

        last_aabb = aabb;

        let mut wall_collisions = HashSet::new();
        for dec in decs {
            let mesh = dec.get_mesh();
            let mat = dec.get_matrix();
            let mesh = mesh_tranform(mesh, mat);

            for (tri, normal) in mesh.into_iter().map(|tri| {
                (
                    tri,
                    vector3_negate(vec3_face_normal(tri[0], tri[1], tri[2])),
                )
            }) {
                // Perform SAT test to check for wall collision with bounding box
                if -normal.y < 0.607 && sat_aabb_tri(&aabb, tri) {
                    // Accumulate normals and count collisions
                    wall_collisions.insert(vector3_negate(normal));
                }
            }
        }

        for wall_normal in wall_collisions {
            // Project velocity onto each wall normal
            let dot = vector3_dot_product(step_size, wall_normal);
            if dot < 0. {
                // Cancel out movement along the wall normal to prevent pushing into walls
                let correction = vector3_scale(wall_normal, dot);
                step_size = vector3_subtract(step_size, correction);
            }

            // Enforce minimum movement in the direction away from the wall
            // step_size = vector3_add(step_size, vector3_scale(wall_normal, 0.0005));
            tmp_pos = vector3_add(
                *position,
                vector3_add(step_size, vector3_scale(wall_normal, 0.005)),
            );
        }

        *position = tmp_pos;
    }

    let mut v_text = text::create_surface(text::FontInput {
        text: format!(
            "velocity: {{{:>5.1},{:>5.1},{:>5.1}  }}",
            velocity.x, velocity.y, velocity.z
        ),
        mode: text::Mode::Solid {
            color: text::FontColor {
                r: 255,
                g: 167,
                b: 167,
                a: 255,
            },
        },
        font: g_game::get_text_font_sm().unwrap(),
    })
    .unwrap();
    v_text.y = 32;
    text::push_surface(&v_text).unwrap();

    let mut v_text = text::create_surface(text::FontInput {
        text: format!("on_ground: {:?}", *on_ground),
        mode: text::Mode::Solid {
            color: text::FontColor {
                r: 167,
                g: 255,
                b: 167,
                a: 255,
            },
        },
        font: g_game::get_text_font_sm().unwrap(),
    })
    .unwrap();
    v_text.y = 16 * 3;
    text::push_surface(&v_text).unwrap();

    let mut v_text = text::create_surface(text::FontInput {
        text: format!("position: {:?}", *position),
        mode: text::Mode::Solid {
            color: text::FontColor {
                r: 167,
                g: 167,
                b: 167,
                a: 255,
            },
        },
        font: g_game::get_text_font_sm().unwrap(),
    })
    .unwrap();
    v_text.y = 16 * 4;
    text::push_surface(&v_text).unwrap();

    let mut v_text = text::create_surface(text::FontInput {
        text: format!(
            "aabb.min: {{{:>5.1},{:>5.1},{:>5.1}  }}",
            last_aabb.min.x, last_aabb.min.y, last_aabb.min.z
        ),
        mode: text::Mode::Solid {
            color: text::FontColor {
                r: 255,
                g: 255,
                b: 167,
                a: 255,
            },
        },
        font: g_game::get_text_font_sm().unwrap(),
    })
    .unwrap();
    v_text.y = 16 * 5;
    text::push_surface(&v_text).unwrap();

    let mut v_text = text::create_surface(text::FontInput {
        text: format!(
            "aabb.max: {{{:>5.1},{:>5.1},{:>5.1}  }}",
            last_aabb.max.x, last_aabb.max.y, last_aabb.max.z
        ),
        mode: text::Mode::Solid {
            color: text::FontColor {
                r: 255,
                g: 255,
                b: 167,
                a: 255,
            },
        },
        font: g_game::get_text_font_sm().unwrap(),
    })
    .unwrap();
    v_text.y = 16 * 6;
    text::push_surface(&v_text).unwrap();

    let mut v_text = text::create_surface(text::FontInput {
        text: format!("last_floor: {:?}", last_floor),
        mode: text::Mode::Solid {
            color: text::FontColor {
                r: 255,
                g: 167,
                b: 255,
                a: 255,
            },
        },
        font: g_game::get_text_font_sm().unwrap(),
    })
    .unwrap();
    v_text.y = 16 * 7;
    text::push_surface(&v_text).unwrap();
}
