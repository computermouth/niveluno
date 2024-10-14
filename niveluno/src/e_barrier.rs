use raymath;

use crate::math::{self, Vector3};

use crate::e_entity::EntityInstance;
use crate::map::Entity;

use crate::text::{BannerInput, FontColor};
use crate::{d_decor, e_player, g_game, render, text};

pub struct Barrier {
    base: Entity,
    id: Option<u32>,
}

const BANNER_COLORS_V3: [[f32; 3]; 9] = [
    [0.0, 0.0, 1.0],
    [0.0, 0.5, 1.0],
    [0.0, 1.0, 1.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 0.5, 0.0],
    [1.0, 0.0, 0.0],
    [0.5, 0.0, 0.0],
    [1.0, 1.0, 1.0],
];

const BANNER_COLORS_RGB: [[u8; 3]; 9] = [
    [0x00, 0x00, 0xFF],
    [0x00, 0x80, 0xFF],
    [0x00, 0xFF, 0xFF],
    [0x00, 0xFF, 0x00],
    [0xFF, 0xFF, 0x00],
    [0xFF, 0x80, 0x00],
    [0xFF, 0x00, 0x00],
    [0x80, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
];

impl EntityInstance for Barrier {
    fn update(&mut self) {
        // todo, push the barrier level surface to a queue with a distance,
        // to perform an ordered draw with the final camera params
        let pos_2d = math::world_point_to_screen_coord(
            self.base.location.into(),
            render::get_camera_pos().unwrap(),
            -render::get_camera_yaw().unwrap(),
            -render::get_camera_pitch().unwrap(),
            render::INTERNAL_W as f32,
            render::INTERNAL_H as f32,
        );

        match pos_2d {
            None => {}
            Some(pos) => {
                // if !d_decor::pos_is_visible(pos) {
                //     return;
                // }

                let color = BANNER_COLORS_RGB[self.id.unwrap() as usize];

                let mut v_text = text::create_barrier_level_surface(BannerInput {
                    color: FontColor {
                        r: color[0],
                        g: color[1],
                        b: color[2],
                        a: 255,
                    },
                    level: self.id.unwrap() * 10,
                })
                .unwrap();

                let pos_x = pos.x as i32;
                let pos_y = pos.y as i32;
                let vtx_w = v_text.w as i32;
                let vtx_h = v_text.h as i32;
                let int_w = render::INTERNAL_W as i32;
                let int_h = render::INTERNAL_H as i32;

                if pos_x < (0 - vtx_w)
                    || pos_y < (0 - vtx_h)
                    || pos_x > (int_w + vtx_w)
                    || pos_y > (int_h + vtx_h)
                {
                    return;
                }

                v_text.x = pos.x as u32 - v_text.w / 2;
                v_text.y = pos.y as u32 - v_text.h / 2;
                text::push_surface(&v_text).unwrap();
            }
        }
    }

    fn draw_model(&mut self) {
        let ref_ent = g_game::get_ref_entity(self.base.index).unwrap();

        let mat_s =
            raymath::matrix_scale(self.base.scale[0], self.base.scale[1], self.base.scale[2]);
        let mat_r = raymath::quaternion_to_matrix(self.base.rotation.into());
        let mat_t = raymath::matrix_translate(
            self.base.location[0],
            self.base.location[1],
            self.base.location[2],
        );

        let mut mat = raymath::matrix_identity();
        mat = raymath::matrix_multiply(mat, mat_s);
        mat = raymath::matrix_multiply(mat, mat_r);
        mat = raymath::matrix_multiply(mat, mat_t);

        let color = BANNER_COLORS_V3[self.id.unwrap() as usize];

        let cam_pos = render::get_camera_pos().unwrap();
        let cam_light_dist = math::vector3_distance(self.base.location.into(), cam_pos);

        let fade = math::scale(cam_light_dist, 32., 34., 1., 0.).clamp(0., 1.);
        let color = math::vector3_scale(color.into(), fade);

        let dc = render::DrawCall {
            matrix: mat,
            texture: ref_ent.texture_handle as u32,
            f1: ref_ent.frame_handles[0] as i32,
            f2: ref_ent.frame_handles[0] as i32,
            mix: 0.0,
            num_verts: ref_ent.num_verts,
            glow: Some(color),
        };
        render::draw(dc).unwrap();
    }
}

impl Barrier {
    pub fn new(entt: &Entity) -> Self {
        let mut id = None;

        for (i, v) in entt.params.iter().enumerate() {
            let key = g_game::get_param(*v as usize).unwrap();
            if key == "id" {
                let value = g_game::get_param(entt.params[i + 1] as usize).unwrap();
                id = Some(value.parse().unwrap());
                break;
            }
        }

        Self {
            base: entt.clone(),
            id,
        }
    }
}
