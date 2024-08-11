use raymath::{vector3_add, vector3_multiply, vector3_scale};

use crate::math::Vector3;

use crate::e_entity::EntityInstance;
use crate::map::Entity;

use crate::{g_game, render, time};

pub struct Light {
    base: Entity,
    r: u8,
    g: u8,
    b: u8,
    intensity: u8,
    orientation: Orientation,
    position: Vector3,
}

#[derive(PartialEq)]
enum Orientation {
    LeftRight,
    UpDown,
    BackForward,
}

impl EntityInstance for Light {
    fn update(&mut self) {
        let (axis, shift) = match self.orientation {
            Orientation::LeftRight => (
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                1.,
            ),
            Orientation::UpDown => (
                Vector3 {
                    x: 0.0,
                    y: 2.0,
                    z: 0.0,
                },
                2.,
            ),
            Orientation::BackForward => (
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                3.,
            ),
        };

        let time_factor = (time::get_run_time().unwrap() + shift).sin() as f32;

        // Calculate the new position with different speeds and intervals
        self.position = vector3_add(
            self.position,
            vector3_scale(vector3_scale(axis, 0.1), time_factor),
        );

        render::push_light(self.position, self.intensity, self.r, self.g, self.b).unwrap();
    }
    fn draw_model(&mut self) {}
}

impl Light {
    pub fn new(entt: &Entity) -> Self {
        let mut rgbi = [1, 128, 255, 255];

        for (i, v) in entt.params.iter().enumerate() {
            let key = g_game::get_param(*v as usize).unwrap();
            if key == "color" {
                let value = g_game::get_param(entt.params[i + 1] as usize).unwrap();
                let str_rgbi: Vec<&str> = value.split(',').collect();
                rgbi[0] = str_rgbi[0].parse().unwrap();
                rgbi[1] = str_rgbi[1].parse().unwrap();
                rgbi[2] = str_rgbi[2].parse().unwrap();
                rgbi[3] = str_rgbi[3].parse().unwrap();
                break;
            }
        }

        let mut orientation = Orientation::LeftRight;
        for (i, v) in entt.params.iter().enumerate() {
            let key = g_game::get_param(*v as usize).unwrap();
            if key == "orientation" {
                let value = g_game::get_param(entt.params[i + 1] as usize).unwrap();
                match value {
                    "lr" => orientation = Orientation::LeftRight,
                    "ud" => orientation = Orientation::UpDown,
                    "bf" => orientation = Orientation::BackForward,
                    _ => eprintln!("unmatched light orientation"),
                }
            }
        }

        Self {
            base: entt.clone(),
            r: rgbi[0],
            g: rgbi[1],
            b: rgbi[2],
            intensity: rgbi[3],
            orientation,
            position: entt.location.into(),
        }
    }
}
