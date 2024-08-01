use rand::prelude::*;

use crate::e_entity::EntityInstance;
use crate::game::get_delta_time;
use crate::level::Entity;

use crate::game;
use crate::render;

pub struct Menu {
    base: Entity,
    yaw: f32,
    pitch: f32,
    speed_x: f32,
    speed_y: f32,
}

impl EntityInstance for Menu {
    fn update(&mut self) {
        self.yaw += self.speed_x * get_delta_time().unwrap() as f32;
        self.pitch += self.speed_y * get_delta_time().unwrap() as f32;
    }

    fn draw_model(&mut self) {
        let ref_ent = game::get_ref_entity(self.base.index).unwrap();

        let dc = render::DrawCall {
            pos: self.base.location.into(),
            yaw: self.yaw,
            pitch: self.pitch,
            texture: ref_ent.texture_handle as u32,
            f1: ref_ent.frame_handles[0] as i32,
            f2: ref_ent.frame_handles[0] as i32,
            mix: 0.,
            num_verts: ref_ent.num_verts,
            unlit: false,
        };
        render::draw(dc).unwrap();
    }
}

impl Menu {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            yaw: 0.,
            pitch: 0.,
            speed_x: (rand::random::<f32>() - 0.5),
            speed_y: (rand::random::<f32>() - 0.5),
        }
    }
}
