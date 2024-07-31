use crate::e_entity::EntityInstance;
use crate::level::Entity;

use crate::game;
use crate::render;

pub struct Gcyl {
    base: Entity,
    yaw: f32,
    pitch: f32,
}

impl EntityInstance for Gcyl {
    fn update(&mut self) {
        self.yaw += 0.0001;
        self.pitch -= 0.0003;
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

impl Gcyl {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            yaw: 0.,
            pitch: 0.,
        }
    }
}
