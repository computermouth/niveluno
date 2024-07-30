use crate::e_entity::EntityInstance;
use crate::level::Entity;

use crate::render;

pub struct Player {
    base: Entity,
    pitch: f32,
    yaw: f32,
}

impl EntityInstance for Player {
    fn update(&mut self) {
        // self.pitch += 0.001;
        // self.pitch = 3.14 / 2.;
        // self.yaw += 0.001;
        render::set_camera_pitch(self.pitch).unwrap();
        render::set_camera_yaw(self.yaw).unwrap();

        render::set_camera_pos(self.base.location.into()).unwrap();
    }
    fn draw_model(&mut self) {}
}

impl Player {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            pitch: 0.,
            yaw: 0.,
        }
    }
}
