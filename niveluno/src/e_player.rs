use raymath::{matrix_rotate_y, vector3_add, vector3_scale, vector3_transform};

use crate::g_game::TopState;
use crate::map::{self, Entity};
use crate::math::Vector3;
use crate::render;
use crate::text;
use crate::{asset, g_game};
use crate::{g_instance, input};

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
    height: f32,
    width: f32,
}

impl Player {
    pub fn new(entt: &Entity) -> Self {
        // timed surface on spawn
        let mut spawn = text::create_surface(text::TextInput {
            text: "SPAWN".to_string(),
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

        spawn.x = 100;
        spawn.y = 100;

        let ts = text::TimedSurface::new(spawn, 1000);

        text::push_timed_surface(ts).unwrap();

        Self {
            base: entt.clone(),
            pitch: 0.,
            yaw: 0.,
            position: entt.location.into(),
            speed: 96.,
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
                TopState::Menu => text::create_surface(text::TextInput {
                    text: "MAIN MENU".to_string(),
                    mode: text::Mode::Solid {
                        color: text::FontColor {
                            r: 255,
                            g: 167,
                            b: 167,
                            a: 255,
                        },
                    },
                    font: g_game::get_text_font_lg().unwrap(),
                })
                .unwrap(),
                TopState::Play => text::create_surface(text::TextInput {
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
            height: 2.1,
            width: 3.,
        }
    }

    pub fn update_hud(&self) {
        text::push_surface(&self.hud).unwrap();
    }

    pub fn update(&mut self) {
        let _ = self.base;

        let keys = input::get_keys().unwrap();

        if keys[input::Key::Jump as usize] == true && g_game::get_state().unwrap() == TopState::Menu
        {
            let nmap = asset::get_file("map/nmap.mp").unwrap().unwrap();
            let payload = mparse::unmarshal(&nmap).unwrap();
            let level = map::load(payload).unwrap();
            g_game::set_state(TopState::Play).unwrap();
            g_game::stage_level(level.clone()).unwrap();
            return;
        } else if g_game::get_state().unwrap() == TopState::Menu {
            render::set_camera_pos(self.position).unwrap();
            return;
        }

        let (mouse_x, mouse_y) = input::get_mouse().unwrap();
        self.pitch = (self.pitch + mouse_y * 0.00015).clamp(-1.5, 1.5);
        self.yaw = (self.yaw + mouse_x * 0.00015) % (2. * std::f32::consts::PI);

        render::set_camera_pitch(self.pitch).unwrap();
        render::set_camera_yaw(self.yaw).unwrap();

        let y_mat = matrix_rotate_y(self.yaw);

        let key_r = keys[input::Key::Right as usize] as i8;
        let key_l = keys[input::Key::Left as usize] as i8;
        let key_u = keys[input::Key::Up as usize] as i8;
        let key_d = keys[input::Key::Down as usize] as i8;

        self.acceleration = vector3_transform(
            Vector3 {
                x: (key_r - key_l) as f32,
                y: 0.,
                z: (key_u - key_d) as f32,
            },
            y_mat,
        );

        let key_jump = keys[input::Key::Jump as usize] as i8;
        if key_jump == 1 && self.on_ground {
            self.velocity.y = 15.;
            self.on_ground = false;
        }

        let speed_factor = match self.on_ground {
            true => 1.0,
            false => 0.9,
        };
        self.acceleration = vector3_scale(self.acceleration, self.speed * speed_factor);

        self.friction = match self.on_ground {
            true => 10.,
            false => 2.5,
        };

        g_instance::update_physics(
            &mut self.acceleration,
            &mut self.velocity,
            &mut self.position,
            &mut self.on_ground,
            self.height,
            self.width,
        );

        let bis = g_instance::get_barrier_instances().unwrap();
        for ba in bis {
            let barrier = match ba {
                g_instance::Instance::EBarrier(b) => b,
                _ => panic!(), // this sucks ass
            };

            if barrier.position_is_inside(vector3_add(
                self.position,
                Vector3::new(0., self.height / 2., 0.),
            )) {
                if cfg!(debug_assertions) {
                    let mut v_text = text::create_surface(text::TextInput {
                        text: format!("barrier id: {}", barrier.get_id()),
                        mode: text::Mode::Solid {
                            color: text::FontColor {
                                r: 255,
                                g: 255,
                                b: 255,
                                a: 255,
                            },
                        },
                        font: g_game::get_text_font_sm().unwrap(),
                    })
                    .unwrap();
                    v_text.y = 16 * 8;
                    text::push_surface(&v_text).unwrap();
                }
            }
        }

        // TODO: Smooth step up on stairs
        // r_camera.y = e->p.y + 8 - clamp(game_time - e->_stepped_up_at, 0, 0.1) * -160;
        let camera_pos = vector3_add(self.position, Vector3::new(0., self.height, 0.));
        render::set_camera_pos(camera_pos).unwrap();

        self.update_hud();
    }
    pub fn draw_model(&mut self) {}
    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }
}
