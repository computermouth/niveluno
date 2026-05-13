use core::f32;

use crate::e_player;
use crate::g_instance;
use crate::map::Entity;
use crate::text;
use crate::time;

use crate::g_game;
use crate::render;

use raymath::{self, Vector3};

#[derive(Debug)]
pub struct KeyB {
    base: Entity,
    pub dead: bool
}

impl KeyB {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            dead: false
        }
    }
    pub fn update(&mut self) {

        let player = g_instance::get_player_instance().unwrap();

        if raymath::vector3_distance(player.position, self.base.location.into()) < 3. {
            player.get_key();
            self.dead = true;

            let mut spawn = text::create_text_overlay_surface(text::TextInput {
                text: format!("{} FOUND", e_player::Equipment::Key.get_details().name),
                mode: text::Mode::Solid {
                    color: text::FontColor {
                        r: 64,
                        g: 32,
                        b: 196,
                        a: 255,
                    },
                },
                font: g_game::get_text_font_lg().unwrap(),
            })
            .unwrap();

            spawn.dst_rect.set_x(200);
            spawn.dst_rect.set_y(200);

            let ts = text::TimedSurface::new(spawn, 1000);
            text::push_timed_surface(ts).unwrap();
        }

    }

    pub fn draw_model(&mut self) {
        let ref_dec = g_game::get_ref_entity(self.base.ref_id).unwrap();

            let mat_s =
                raymath::matrix_scale(
                    self.base.scale[0],
                    self.base.scale[1],
                    self.base.scale[2]
                );
            
            let mat_r = raymath::quaternion_to_matrix(self.base.rotation.into());

            let pos = Vector3::new(
                self.base.location[0],
                self.base.location[1],
                self.base.location[2]
            );

            let mat_t = raymath::matrix_translate(
                pos.x,
                pos.y,
                pos.z,
            );

            let mut mat = raymath::matrix_identity();
            mat = raymath::matrix_multiply(mat, mat_s);
            mat = raymath::matrix_multiply(mat, mat_r);
            mat = raymath::matrix_multiply(mat, mat_t);

        let dc = render::DrawCall {
            matrix: mat,
            texture: ref_dec.texture_handle as u32,
            f1: ref_dec.frame_handles[0] as i32,
            f2: ref_dec.frame_handles[0] as i32,
            mix: 0.,
            num_verts: ref_dec.num_verts,
            glow: None,
        };
        render::draw(dc).unwrap();
    }

    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }
}
