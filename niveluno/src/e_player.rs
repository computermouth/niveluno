use crate::e_entity::EntityInstance;
use crate::level::Entity;

use crate::math::Vec3;
use crate::render;
use crate::{game, input};

pub struct Player {
    base: Entity,
    pitch: f32,
    yaw: f32,
    position: Vec3,
}

impl EntityInstance for Player {
    fn update(&mut self) {
        // self.pitch += 0.001;
        // self.pitch = 3.14 / 2.;
        // self.yaw += 0.001;
        render::set_camera_pitch(self.pitch).unwrap();
        render::set_camera_yaw(self.yaw).unwrap();
        let delta = game::get_delta_time().unwrap() as f32;

        let keys = input::get_keys().unwrap();

        if keys[input::Key::Up as usize] == true {
            self.position.z += 2. * delta;
        } else if keys[input::Key::Down as usize] == true {
            self.position.z -= 2. * delta;
        }

        if keys[input::Key::Left as usize] == true {
            self.position.x -= 2. * delta;
        } else if keys[input::Key::Right as usize] == true {
            self.position.x += 2. * delta;
        }

        render::set_camera_pos(self.position).unwrap();
    }
    fn draw_model(&mut self) {}
}

impl Player {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            pitch: 0.,
            yaw: 0.,
            position: entt.location.into(),
        }
    }
}
