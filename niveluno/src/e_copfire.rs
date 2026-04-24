use core::f32;

use rand::Rng;
use raymath::{vector3_add, vector3_scale};

use crate::map::Entity;
use crate::math::Vector3;

use crate::{g_game, g_instance, render, time};

struct Cloud {
    start_time: f64,
    skew: Vector3,
}

impl Cloud {
    fn new(start_time: f64) -> Self {
        let rng = g_game::get_rng().unwrap();

        let skew = Vector3::new(
            rng.gen_range(-1f32..1f32),
            rng.gen_range(-1f32..1f32),
            rng.gen_range(-1f32..1f32)
        );

        Self {
            start_time,
            skew: raymath::vector3_normalize(skew)
        }
    }
}

pub struct CopFire {
    base: Entity,
    clouds: Vec<Cloud>,
    current_time: f64,
}

const CLOUD_LIFETIME: f64 = 10.;

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
        self.clouds.retain(|cloud| cloud.start_time + CLOUD_LIFETIME > self.current_time);

        let clen = self.clouds.len();
        if clen == 0 {
            self.clouds.push(Cloud::new(self.current_time));
        }

    }

    pub fn draw_model(&mut self) {

        let re = g_instance::ref_ent_from_str("icosphere").unwrap();

        for cloud in &self.clouds {

            let progress = ((self.current_time - cloud.start_time) / CLOUD_LIFETIME) as f32;
            let scale = 10. * (progress * std::f32::consts::PI).sin();
            let y = 10. * (progress * std::f32::consts::PI / 2.).sin();

            // eprintln!("progress: {progress}");

            let mat_s =
                raymath::matrix_scale(self.base.scale[0] * scale, self.base.scale[1] * scale, self.base.scale[2] * scale);
            let mat_r = raymath::quaternion_to_matrix(self.base.rotation.into());
            let mat_t = raymath::matrix_translate(
                self.base.location[0],
                self.base.location[1] + y as f32,
                self.base.location[2],
            );

            let mut mat = raymath::matrix_identity();
            mat = raymath::matrix_multiply(mat, mat_s);
            mat = raymath::matrix_multiply(mat, mat_r);
            mat = raymath::matrix_multiply(mat, mat_t);
            
            let dc = render::DrawCall {
                matrix: mat,
                texture: re.texture_handle as u32,
                f1: re.frame_handles[0] as i32,
                f2: re.frame_handles[0] as i32,
                mix: 0.0,
                num_verts: re.num_verts,
                glow: Some(Vector3::new(255., 0., 0.)),
            };
            render::draw(dc).unwrap();

        }
    }

    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }
}
