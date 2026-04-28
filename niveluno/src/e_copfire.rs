use core::f32;

use rand::Rng;
use raymath::{vector3_add, vector3_scale};

use crate::map::Entity;
use crate::math::Vector3;

use crate::{g_game, g_instance, render, time};

struct Cloud {
    start_time: f64,
    dir: Vector3,
    scale_mult: f32,
    start_color: Vector3,
    end_color: Vector3,
}

impl Cloud {
    fn new(start_time: f64) -> Self {
        let rng = g_game::get_rng().unwrap();

        let dir = Vector3::new(
            rng.gen_range(-1f32..1f32),
            rng.gen_range(-1f32..1f32),
            rng.gen_range(-1f32..1f32)
        );

        let start_green = rng.gen_range(0.2f32..0.5f32);
        let start_color = Vector3::new(0.5, start_green, start_green / 2.);

        let end_grey = rng.gen_range((start_green / 2.)..start_green);
        let end_color = Vector3::new(end_grey, end_grey, end_grey);

        Self {
            start_time,
            scale_mult: rng.gen_range(1f32..3f32),
            dir: raymath::vector3_normalize(dir),
            start_color,
            end_color,
        }
    }
}

pub struct CopFire {
    base: Entity,
    clouds: Vec<Cloud>,
    current_time: f64,
}

const CLOUD_LIFETIME: f32 = 20.;

impl CopFire {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            clouds: vec![],
            current_time: 0.,
        }
    }

    pub fn update(&mut self) {

        self.current_time = time::get_run_time().unwrap();

        // remove all 10s+ old clouds
        self.clouds.retain(|cloud| cloud.start_time + CLOUD_LIFETIME as f64 > self.current_time);

        let clen = self.clouds.len();

        // push if empty
        if clen == 0  ||
            // push if the newest is more than Xs old
            self.clouds[clen - 1].start_time + 0.25 < self.current_time
            {
            self.clouds.push(Cloud::new(self.current_time));
        }

    }

    pub fn draw_model(&mut self) {

        let re = g_instance::ref_ent_from_str("icosphere").unwrap();

        for cloud in &self.clouds {

            // linear
            let progress = (self.current_time - cloud.start_time) as f32 / CLOUD_LIFETIME;
            // open and close
            let p_pi_sin = (progress * std::f32::consts::PI).sin();
            let scale = CLOUD_LIFETIME / 2. * p_pi_sin;
            // just trend open
            let p_pi_half_sin = (progress * std::f32::consts::PI / 2.).sin();
            let y = CLOUD_LIFETIME * p_pi_half_sin;

            let mat_s =
                raymath::matrix_scale(
                    self.base.scale[0] * scale * cloud.scale_mult,
                    self.base.scale[1] * scale * cloud.scale_mult,
                    self.base.scale[2] * scale * cloud.scale_mult
                );
            let mat_r = raymath::quaternion_to_matrix(self.base.rotation.into());

            let pos = Vector3::new(
                self.base.location[0] + cloud.dir.x * (0.1 + 2. * progress),
                self.base.location[1] + cloud.dir.y * (0.1 + 2. * progress) + y as f32,
                self.base.location[2] + cloud.dir.z * (0.1 + 2. * progress),
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

            let color = raymath::vector3_lerp(cloud.start_color, cloud.end_color, p_pi_half_sin);
            
            let dc = render::DrawCall {
                matrix: mat,
                texture: re.texture_handle as u32,
                f1: re.frame_handles[0] as i32,
                f2: re.frame_handles[0] as i32,
                mix: 0.0,
                num_verts: re.num_verts,
                glow: Some(color),
            };
            render::draw(dc).unwrap();

            render::push_light(
                pos,
                4,
                (cloud.start_color.x * 128.) as u8,
                (cloud.start_color.y * 128.) as u8,
                0
            ).unwrap();

        }
    }

    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }
}
