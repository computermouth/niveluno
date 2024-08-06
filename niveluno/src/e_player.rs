use crate::asset;
use crate::e_entity::EntityInstance;
use crate::g_game;
use crate::g_game::TopState;
use crate::map::{self, Entity};
use crate::math::Vec3;
use crate::render;
use crate::text;
use crate::{input, time};

pub struct Player {
    base: Entity,
    pitch: f32,
    yaw: f32,
    position: Vec3,
    hud: Box<text::TextSurface>,
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

        if keys[input::Key::Up as usize] == true {
            self.position.z += 2. * delta;
        } else if keys[input::Key::Down as usize] == true {
            self.position.z -= 2. * delta;
        }

        if keys[input::Key::Left as usize] == true {
            self.position.x -= 2. * delta;
        } else if keys[input::Key::Right as usize] == true {
            self.position.x += 2. * delta;
        }

        if keys[input::Key::Jump as usize] == true && g_game::get_state().unwrap() == TopState::Menu
        {
            let nmap = asset::get_file("map/nmap.mp").unwrap().unwrap();
            let payload = mparse::unmarshal(&nmap).unwrap();
            let level = map::load(payload).unwrap();
            g_game::set_state(TopState::Play).unwrap();
            g_game::set_and_init_level(level.clone()).unwrap();
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
