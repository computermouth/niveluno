use crate::d_decor::DecorInstance;
use crate::level::Decor;

use crate::game;
use crate::render;

pub struct Floor {
    base: Decor,
}

impl DecorInstance for Floor {
    fn update(&mut self) {
        // self.base.location[1] += 0.001;
        // eprintln!("floor pos: {:?}", self.base.location);
    }

    fn draw_model(&mut self) {
        let ref_dec = game::get_ref_decor(self.base.ref_id).unwrap();

        let dc = render::DrawCall {
            pos: self.base.location.into(),
            yaw: 0.,
            pitch: 0.,
            texture: ref_dec.texture_handle as u32,
            f1: ref_dec.frame_handle as i32,
            f2: ref_dec.frame_handle as i32,
            mix: 0.,
            num_verts: ref_dec.num_verts,
            unlit: false,
        };
        render::draw(dc).unwrap();
    }
}

impl Floor {
    pub fn new(decor: &Decor) -> Self {
        Self {
            base: decor.clone(),
        }
    }
}
