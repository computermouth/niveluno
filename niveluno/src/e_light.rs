use crate::e_entity::EntityInstance;
use crate::map::Entity;

use crate::{g_game, render};

pub struct Light {
    base: Entity,
    r: u8,
    g: u8,
    b: u8,
    intensity: u8,
}

impl EntityInstance for Light {
    fn update(&mut self) {
        render::push_light(
            self.base.location.into(),
            self.intensity,
            self.r,
            self.g,
            self.b,
        )
        .unwrap();
    }
    fn draw_model(&mut self) {}
}

impl Light {
    pub fn new(entt: &Entity) -> Self {
        let mut rgbi = [1, 128, 255, 255];

        for v in &entt.params {
            let key = g_game::get_param(*v as usize).unwrap();
            if key == "color" {
                let value = g_game::get_param(1 + *v as usize).unwrap();
                let str_rgbi: Vec<&str> = value.split(',').collect();
                rgbi[0] = str_rgbi[0].parse().unwrap();
                rgbi[1] = str_rgbi[1].parse().unwrap();
                rgbi[2] = str_rgbi[2].parse().unwrap();
                rgbi[3] = str_rgbi[2].parse().unwrap();
                break;
            }
        }

        Self {
            base: entt.clone(),
            r: rgbi[0],
            g: rgbi[1],
            b: rgbi[2],
            intensity: rgbi[3],
        }
    }
}
