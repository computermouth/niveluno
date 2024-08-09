use core::f32;

use crate::e_entity::EntityInstance;
use crate::map::Entity;
use crate::time;

use crate::g_game;
use crate::render;

#[derive(Debug)]
pub struct Menu {
    base: Entity,
    yaw: f32,
    pitch: f32,
}

impl EntityInstance for Menu {
    fn update(&mut self) {
        let time = time::get_run_time().unwrap() as f32;
        self.yaw = f32::consts::PI + time.sin() as f32 / 2.;
        self.pitch = (f32::consts::PI + time * 2.).sin() as f32 / 4.;
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

impl Menu {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            yaw: f32::consts::PI,
            pitch: 0.,
        }
    }
}
