use core::f32;

use crate::g_instance;
use crate::map::Entity;
use crate::text;
use crate::time;

use crate::g_game;
use crate::render;

use raymath::{self, Vector3};

#[derive(Debug)]
pub struct PrototypeCoinA {
    base: Entity,
    scale_mat: raymath::Matrix,
    quat: raymath::Quaternion,
    position: Vector3,
    pub dead: bool,
}

impl PrototypeCoinA {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            quat: raymath::quaternion_identity(),
            scale_mat: raymath::matrix_scale(entt.scale[0], entt.scale[1], entt.scale[2]),
            position: entt.location.into(),
            dead: false,
        }
    }
    pub fn update(&mut self) {

        let quat_y = raymath::quaternion_from_axis_angle(Vector3::new(0., 1., 0.), 2. *  time::get_delta_time().unwrap() as f32);
        self.quat = raymath::quaternion_multiply(self.quat, quat_y);

        let base: Vector3 = self.base.location.into();

        let x_trans = raymath::vector3_scale(Vector3::new(5., 0., 0.), time::get_run_time().unwrap().sin() as f32);
        let z_trans = raymath::vector3_scale(Vector3::new(0., 0., 5.), time::get_run_time().unwrap().cos() as f32);
        self.position = raymath::vector3_add(raymath::vector3_add(base, x_trans), z_trans);


        render::push_light(
            self.position,
            16, 128, 255, 0
        ).unwrap();

        let player = g_instance::get_player_instance().unwrap();

        if raymath::vector3_distance(player.position, self.position) < 3. {
            player.get_coin();
            self.dead = true;


            // timed surface on coin-get
            let mut spawn = text::create_text_overlay_surface(text::TextInput {
                text: "COIN GET".to_string(),
                mode: text::Mode::Solid {
                    color: text::FontColor {
                        r: 32,
                        g: 196,
                        b: 64,
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
        let ref_ent = g_game::get_ref_entity(self.base.ref_id).unwrap();

        let mat_r = raymath::quaternion_to_matrix(self.quat);
        let mat_t = raymath::matrix_translate(
            self.position.x,
            self.position.y,
            self.position.z,
        );

        let mut mat = raymath::matrix_identity();
        mat = raymath::matrix_multiply(mat, self.scale_mat);
        mat = raymath::matrix_multiply(mat, mat_r);
        mat = raymath::matrix_multiply(mat, mat_t);

        let dc = render::DrawCall {
            matrix: mat,
            texture: ref_ent.texture_handle as u32,
            f1: ref_ent.frame_handles[0] as i32,
            f2: ref_ent.frame_handles[0] as i32,
            mix: 0.,
            num_verts: ref_ent.num_verts,
            glow: Some(Vector3::new(0.25, 1., 0.)),
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
