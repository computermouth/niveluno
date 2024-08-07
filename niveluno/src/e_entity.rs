// would rather match on sum type, but calling
// other_e::func seems hairy. maybe if it took a copy
// of other_e, modified, bubbled up to a buffer to replace?

use raymath::Vector3;

use crate::time;

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
    // acceleration.y = -36. * GRAVITY;

    let delta_time = time::get_delta_time().unwrap() as f32;

    // Integrate acceleration & friction into velocity
    let df = 1.0f32.min(FRICTION * delta_time);
    let af = *acceleration * delta_time;
    let vf = *velocity
        * Vector3 {
            x: df,
            y: 0.,
            z: df,
        };
    *velocity += af - vf;

    let move_dist = *velocity * delta_time;
    // todo, cast ray, handle collision
    *position += move_dist;
}
