use crate::e_entity::EntityInstance;
use crate::map::Entity;
use crate::time::get_delta_time;

use crate::g_game;
use crate::render;

pub struct Gcyl {
    base: Entity,
    yaw: f32,
    pitch: f32,
}

impl EntityInstance for Gcyl {
    fn update(&mut self) {
        self.yaw += 1. * get_delta_time().unwrap() as f32;
        self.pitch -= 3. * get_delta_time().unwrap() as f32;
    }

    fn draw_model(&mut self) {
        let ref_ent = g_game::get_ref_entity(self.base.index).unwrap();

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

impl Gcyl {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            yaw: 0.,
            pitch: 0.,
        }
    }
}
