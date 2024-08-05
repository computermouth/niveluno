use crate::e_entity::EntityInstance;
use crate::g_game;
use crate::g_game::TopState;
use crate::map::Entity;
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

        render::set_camera_pos(self.position).unwrap();

        self.update_hud();
    }
    fn draw_model(&mut self) {}
}

impl Player {
    pub fn new(entt: &Entity) -> Self {
        Self {
            base: entt.clone(),
            pitch: 0.,
            yaw: 0.,
            position: entt.location.into(),
            hud: match g_game::get_state().unwrap() {
                TopState::Menu => text::create_surface(text::FontInput {
                    text: "TITLE".to_string(),
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
                TopState::Play => {
                    let mut ghost = text::create_surface(text::FontInput {
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
                    .unwrap();

                    ghost.x = 100;
                    ghost.y = 100;

                    ghost
                }
            },
        }
    }

    pub fn update_hud(&self) {
        text::push_surface(&self.hud).unwrap();
    }
}
