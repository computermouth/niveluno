use core::f32;

use raymath::{vector3_add, vector3_scale};

use crate::map::Entity;
use crate::math::Vector3;

use crate::{g_game, render, time};

pub struct CopLight {
    base: Entity,
}

impl CopLight {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
        }
    }
    pub fn update(&mut self) {

        let run_time = time::get_run_time().unwrap();
        let speed = 10.;

        let x_trans = raymath::vector3_scale(Vector3::new(1., 0., 0.), (run_time * speed).sin() as f32);
        let z_trans = raymath::vector3_scale(Vector3::new(0., 0., 1.), (run_time * speed).cos() as f32);
        let position = raymath::vector3_add(raymath::vector3_add(self.base.location.into(), x_trans), z_trans);

        render::push_light(position, 16, 255, 0, 0).unwrap();

        let x_trans = raymath::vector3_scale(Vector3::new(1., 0., 0.), (run_time * speed + std::f64::consts::PI).sin() as f32);
        let z_trans = raymath::vector3_scale(Vector3::new(0., 0., 1.), (run_time * speed + std::f64::consts::PI).cos() as f32);
        let position = raymath::vector3_add(raymath::vector3_add(self.base.location.into(), x_trans), z_trans);

        render::push_light(position, 16, 0, 0, 255).unwrap();
    }

    pub fn draw_model(&mut self) {}

    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }
}
