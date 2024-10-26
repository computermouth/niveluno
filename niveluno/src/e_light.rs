use raymath::{vector3_add, vector3_scale};

use crate::map::Entity;
use crate::math::Vector3;

use crate::{g_game, render, time};

pub struct Light {
    _base: Entity,
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
    Unset,
}

impl Light {
    pub fn new(entt: &Entity) -> Self {
        // let ref_ent = g_game::get_ref_entity(entt.index).unwrap();

        // eprintln!("re.names: {:?}", ref_ent.frame_names);

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
                orientation = match value {
                    "lr" => Orientation::LeftRight,
                    "ud" => Orientation::UpDown,
                    "bf" => Orientation::BackForward,
                    _ => Orientation::Unset,
                }
            }
        }

        Self {
            _base: entt.clone(),
            r: rgbi[0],
            g: rgbi[1],
            b: rgbi[2],
            intensity: rgbi[3],
            orientation,
            position: entt.location.into(),
        }
    }
    pub fn update(&mut self) {
        // if self.orientation != Orientation::Unset {
        //     self.move_on_orientation()
        // }

        render::push_light(self.position, self.intensity, self.r, self.g, self.b).unwrap();
    }
    pub fn draw_model(&mut self) {}

    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }

    fn move_on_orientation(&mut self) {
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
            Orientation::Unset => {
                unreachable!("light had unset orientation in move_on_orientation")
            }
        };

        let time_factor = (time::get_run_time().unwrap() + shift).sin() as f32;

        // Calculate the new position with different speeds and intervals
        self.position = vector3_add(
            self.position,
            vector3_scale(vector3_scale(axis, 0.1), time_factor),
        );
    }
}
