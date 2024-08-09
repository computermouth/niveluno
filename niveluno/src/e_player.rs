use raymath::{Matrix, Quaternion};

use crate::e_entity::EntityInstance;
use crate::g_game;
use crate::g_game::TopState;
use crate::map::{self, Entity};
use crate::math::Vector3;
use crate::render;
use crate::text;
use crate::{asset, e_entity};
use crate::{input, time};

pub struct Player {
    base: Entity,
    pitch: f32,
    yaw: f32,
    position: Vector3,
    hud: Box<text::TextSurface>,
    speed: f32,
    acceleration: Vector3,
    velocity: Vector3,
    on_ground: bool,
    friction: f32,
}

impl EntityInstance for Player {
    fn update(&mut self) {
        let _ = self.base;

        let (mouse_x, mouse_y) = input::get_mouse().unwrap();
        self.pitch = (self.pitch + mouse_y * 0.00015).clamp(-1.5, 1.5);
        self.yaw = (self.yaw + mouse_x * 0.00015) % (2. * std::f32::consts::PI);

        render::set_camera_pitch(self.pitch).unwrap();
        render::set_camera_yaw(self.yaw).unwrap();
        let delta = time::get_delta_time().unwrap() as f32;

        let keys = input::get_keys().unwrap();
        let key_r = keys[input::Key::Right as usize] as i8;
        let key_l = keys[input::Key::Left as usize] as i8;
        let key_u = keys[input::Key::Up as usize] as i8;
        let key_d = keys[input::Key::Down as usize] as i8;

        let speed_factor = match self.on_ground {
            true => 1.0,
            false => 0.3,
        };

        let friction = match self.on_ground {
            true => 10.,
            false => 2.5,
        };

        // let y_mat = Matrix::rotate_y(self.yaw);

        self.acceleration = Vector3 {
            x: (key_r - key_l) as f32,
            y: 0.,
            z: (key_u - key_d) as f32,
        }
        .rotate_by(Quaternion {
            x: 0.,
            y: (self.yaw / 2.).sin(),
            z: 0.,
            w: (self.yaw / 2.).cos(),
        }) * (self.speed * speed_factor);

        e_entity::update_physics(
            &mut self.acceleration,
            &mut self.velocity,
            &mut self.position,
        );

        // self.position += Vector3 {
        //     x: time::get_delta_time().unwrap() as f32 * (key_r - key_l) as f32,
        //     y: 0.,
        //     z: time::get_delta_time().unwrap() as f32 * (key_u - key_d) as f32,
        // };

        if keys[input::Key::Jump as usize] == true && g_game::get_state().unwrap() == TopState::Menu
        {
            let nmap = asset::get_file("map/nmap.mp").unwrap().unwrap();
            let payload = mparse::unmarshal(&nmap).unwrap();
            let level = map::load(payload).unwrap();
            g_game::set_state(TopState::Play).unwrap();
            g_game::stage_level(level.clone()).unwrap();
        } else {
            render::set_camera_pos(self.position).unwrap();

            self.update_hud();
        }
    }
    fn draw_model(&mut self) {}
}

impl Player {
    pub fn new(entt: &Entity) -> Self {
        // timed surface on spawn
        let mut spawn = text::create_surface(text::FontInput {
            text: "SPAWN".to_string(),
            mode: text::Mode::Solid {
                color: text::FontColor {
                    r: 32,
                    g: 196,
                    b: 64,
                    a: 255,
                },
            },
            font: g_game::get_text_font().unwrap(),
        })
        .unwrap();

        spawn.x = 100;
        spawn.y = 100;

        let ts = text::TimedSurface::new(spawn, 1000);

        text::push_timed_surface(ts).unwrap();

        Self {
            base: entt.clone(),
            pitch: 0.,
            yaw: 0.,
            position: entt.location.into(),
            speed: 36.,
            acceleration: Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            velocity: Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            on_ground: true,
            friction: 0.3,
            hud: match g_game::get_state().unwrap() {
                TopState::Menu => text::create_surface(text::FontInput {
                    text: "MAIN MENU".to_string(),
                    mode: text::Mode::Solid {
                        color: text::FontColor {
                            r: 255,
                            g: 167,
                            b: 167,
                            a: 255,
                        },
                    },
                    font: g_game::get_text_font().unwrap(),
                })
                .unwrap(),
                TopState::Play => text::create_surface(text::FontInput {
                    text: "󰊠󰘉".to_string(),
                    mode: text::Mode::Solid {
                        color: text::FontColor {
                            r: 167,
                            g: 167,
                            b: 255,
                            a: 255,
                        },
                    },
                    font: g_game::get_symb_font().unwrap(),
                })
                .unwrap(),
            },
        }
    }

    pub fn update_hud(&self) {
        text::push_surface(&self.hud).unwrap();
    }
}
