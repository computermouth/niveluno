use crate::e_player;
use crate::g_instance;
use crate::map::Entity;
use crate::text;

use crate::g_game;
use crate::render;

use raymath::{self, Vector3};


pub struct EquipmentDetails {
    pub name: &'static str,
    pub icon: char,
    pub action: &'static str,
    pub font_color: &'static text::FontColor,
}

#[derive(Debug, Copy, Clone)]
pub enum Equipment {
    Pistol,
    Flag,
    CandyCane,
    Wrench,
    IceCream,
    Bottle,
    Apple,
    Book,
    Knife,
    Cookie,
    Key,
    Anchor,
    Candle,
}

impl Equipment {

    const PISTOL_ENTITY_NAME: &'static str = "prototype.Gun_Pistol";
    const PISTOL_NAME: &'static str = "pistol";
    const PISTOL_CHAR: char = '\u{f0703}';
    const PISTOL_ACTION: &'static str = "ACQUIRED";
    const PISTOL_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 196, g: 32, b: 32, a: 255 };

    const FLAG_ENTITY_NAME: &'static str = "medieval.flag_blue";
    const FLAG_NAME: &'static str = "flag";
    const FLAG_CHAR: char = '\u{f024}';
    const FLAG_ACTION: &'static str = "DISCOVERED";
    const FLAG_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 32, g: 64, b: 196, a: 255 };

    const CANDY_CANE_ENTITY_NAME: &'static str = "xmas.candycane_small";
    const CANDY_CANE_NAME: &'static str = "candy cane";
    const CANDY_CANE_CHAR: char = '\u{ef3a}';
    const CANDY_CANE_ACTION: &'static str = "UNEARTHED";
    const CANDY_CANE_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 240, g: 128, b: 160, a: 255 };

    const WRENCH_ENTITY_NAME: &'static str = "tools.wrench_B";
    const WRENCH_NAME: &'static str = "wrench";
    const WRENCH_CHAR: char = '\u{f0ad}';
    const WRENCH_ACTION: &'static str = "SALVAGED";
    const WRENCH_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 128, g: 128, b: 144, a: 255 };

    const ICE_CREAM_ENTITY_NAME: &'static str = "restaurant.food_icecream_cone_vanilla";
    const ICE_CREAM_NAME: &'static str = "ice cream";
    const ICE_CREAM_CHAR: char = '\u{ef88}';
    const ICE_CREAM_ACTION: &'static str = "SWIPED";
    const ICE_CREAM_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 240, g: 220, b: 180, a: 255 };

    const BOTTLE_ENTITY_NAME: &'static str = "dungeon.bottle_C_green";
    const BOTTLE_NAME: &'static str = "bottle";
    const BOTTLE_CHAR: char = '\u{f1132}';
    const BOTTLE_ACTION: &'static str = "SCAVENGED";
    const BOTTLE_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 32, g: 160, b: 64, a: 255 };

    const APPLE_ENTITY_NAME: &'static str = "resource.Food_Apple_Red";
    const APPLE_NAME: &'static str = "apple";
    const APPLE_CHAR: char = '\u{e29e}';
    const APPLE_ACTION: &'static str = "PLUCKED";
    const APPLE_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 220, g: 40, b: 60, a: 255 };

    const BOOK_ENTITY_NAME: &'static str = "tools.journal_open";
    const BOOK_NAME: &'static str = "book";
    const BOOK_CHAR: char = '\u{ede2}';
    const BOOK_ACTION: &'static str = "UNCOVERED";
    const BOOK_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 140, g: 90, b: 50, a: 255 };

    const KNIFE_ENTITY_NAME: &'static str = "restaurant.knife";
    const KNIFE_NAME: &'static str = "knife";
    const KNIFE_CHAR: char = '\u{f09fb}';
    const KNIFE_ACTION: &'static str = "RECOVERED";
    const KNIFE_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 200, g: 200, b: 210, a: 255 };

    const COOKIE_ENTITY_NAME: &'static str = "xmas.cookie";
    const COOKIE_NAME: &'static str = "cookie";
    const COOKIE_CHAR: char = '\u{f0198}';
    const COOKIE_ACTION: &'static str = "FOUND";
    const COOKIE_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 180, g: 130, b: 80, a: 255 };

    const KEY_ENTITY_NAME: &'static str = "tools.key_B";
    const KEY_NAME: &'static str = "key";
    const KEY_CHAR: char = '\u{f084}';
    const KEY_ACTION: &'static str = "OBTAINED";
    const KEY_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 220, g: 180, b: 32, a: 255 };

    const ANCHOR_ENTITY_NAME: &'static str = "medieval.anchor";
    const ANCHOR_NAME: &'static str = "anchor";
    const ANCHOR_CHAR: char = '\u{f0031}';
    const ANCHOR_ACTION: &'static str = "HAULED UP";
    const ANCHOR_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 80, g: 140, b: 200, a: 255 };

    const CANDLE_ENTITY_NAME: &'static str = "dungeon.candle_thin_lit";
    const CANDLE_NAME: &'static str = "candle";
    const CANDLE_CHAR: char = '\u{f05e2}';
    const CANDLE_ACTION: &'static str = "GRABBED";
    const CANDLE_FONT_COLOR: &'static text::FontColor = &text::FontColor { r: 240, g: 160, b: 40, a: 255 };


    pub fn get_details(&self) -> EquipmentDetails {
        match self {
            &Equipment::Pistol => EquipmentDetails { name: Self::PISTOL_NAME, icon: Self::PISTOL_CHAR, action: Self::PISTOL_ACTION, font_color: Self::PISTOL_FONT_COLOR },
            &Equipment::Flag => EquipmentDetails { name: Self::FLAG_NAME, icon: Self::FLAG_CHAR, action: Self::FLAG_ACTION, font_color: Self::FLAG_FONT_COLOR },
            &Equipment::CandyCane => EquipmentDetails { name: Self::CANDY_CANE_NAME, icon: Self::CANDY_CANE_CHAR, action: Self::CANDY_CANE_ACTION, font_color: Self::CANDY_CANE_FONT_COLOR },
            &Equipment::Wrench => EquipmentDetails { name: Self::WRENCH_NAME, icon: Self::WRENCH_CHAR, action: Self::WRENCH_ACTION, font_color: Self::WRENCH_FONT_COLOR },
            &Equipment::IceCream => EquipmentDetails { name: Self::ICE_CREAM_NAME, icon: Self::ICE_CREAM_CHAR, action: Self::ICE_CREAM_ACTION, font_color: Self::ICE_CREAM_FONT_COLOR },
            &Equipment::Bottle => EquipmentDetails { name: Self::BOTTLE_NAME, icon: Self::BOTTLE_CHAR, action: Self::BOTTLE_ACTION, font_color: Self::BOTTLE_FONT_COLOR },
            &Equipment::Apple => EquipmentDetails { name: Self::APPLE_NAME, icon: Self::APPLE_CHAR, action: Self::APPLE_ACTION, font_color: Self::APPLE_FONT_COLOR },
            &Equipment::Book => EquipmentDetails { name: Self::BOOK_NAME, icon: Self::BOOK_CHAR, action: Self::BOOK_ACTION, font_color: Self::BOOK_FONT_COLOR },
            &Equipment::Knife => EquipmentDetails { name: Self::KNIFE_NAME, icon: Self::KNIFE_CHAR, action: Self::KNIFE_ACTION, font_color: Self::KNIFE_FONT_COLOR },
            &Equipment::Cookie => EquipmentDetails { name: Self::COOKIE_NAME, icon: Self::COOKIE_CHAR, action: Self::COOKIE_ACTION, font_color: Self::COOKIE_FONT_COLOR },
            &Equipment::Key => EquipmentDetails { name: Self::KEY_NAME, icon: Self::KEY_CHAR, action: Self::KEY_ACTION, font_color: Self::KEY_FONT_COLOR },
            &Equipment::Anchor => EquipmentDetails { name: Self::ANCHOR_NAME, icon: Self::ANCHOR_CHAR, action: Self::ANCHOR_ACTION, font_color: Self::ANCHOR_FONT_COLOR },
            &Equipment::Candle => EquipmentDetails { name: Self::CANDLE_NAME, icon: Self::CANDLE_CHAR, action: Self::CANDLE_ACTION, font_color: Self::CANDLE_FONT_COLOR },
        }
    }
}

#[derive(Debug)]
pub struct Pickup {
    base: Entity,
    equipment: Equipment,
    pub dead: bool
}

impl Pickup {
    pub fn new(entt: &Entity) -> Self {

        let mut equipment = Equipment::Candle;

        for (i, v) in entt.params.iter().enumerate() {
            let key = g_game::get_param(*v as usize).unwrap();
            if key == "_decor" {
                let value = g_game::get_param(entt.params[i + 1] as usize).unwrap();
                equipment = match value {
                    v if v == Equipment::PISTOL_ENTITY_NAME => Equipment::Pistol,
                    v if v == Equipment::FLAG_ENTITY_NAME => Equipment::Flag,
                    v if v == Equipment::CANDY_CANE_ENTITY_NAME => Equipment::CandyCane,
                    v if v == Equipment::WRENCH_ENTITY_NAME => Equipment::Wrench,
                    v if v == Equipment::ICE_CREAM_ENTITY_NAME => Equipment::IceCream,
                    v if v == Equipment::BOTTLE_ENTITY_NAME => Equipment::Bottle,
                    v if v == Equipment::APPLE_ENTITY_NAME => Equipment::Apple,
                    v if v == Equipment::BOOK_ENTITY_NAME => Equipment::Book,
                    v if v == Equipment::KNIFE_ENTITY_NAME => Equipment::Knife,
                    v if v == Equipment::COOKIE_ENTITY_NAME => Equipment::Cookie,
                    v if v == Equipment::KEY_ENTITY_NAME => Equipment::Key,
                    v if v == Equipment::ANCHOR_ENTITY_NAME => Equipment::Anchor,
                    v if v == Equipment::CANDLE_ENTITY_NAME => Equipment::Candle,
                    _ => unreachable!(),
                };
                break;
            }
        }

        Self {
            base: entt.clone(),
            equipment,
            dead: false
        }
    }
    pub fn update(&mut self) {

        let player = g_instance::get_player_instance().unwrap();

        if raymath::vector3_distance(player.position, self.base.location.into()) < 3. {
            player.push_equip(self.equipment);
            self.dead = true;

            let g_w = render::INTERNAL_W;
            let g_h = render::INTERNAL_H;

            // drop shadow
            let details = self.equipment.get_details();
            let mut text_drop_shadow = text::create_text_overlay_surface(text::TextInput {
                text: format!("{}: {}", details.name.to_uppercase(), details.action),
                mode: text::Mode::Solid {
                    color: text::FontColor { r: 96, b: 96, g: 96, a: 96},
                },
                font: g_game::get_text_font_lg().unwrap(),
            })
            .unwrap();

            let t_x = g_w / 2 - (text_drop_shadow.src_rect.w / 2);
            let t_y = g_h / 2 - (text_drop_shadow.src_rect.h / 2);

            text_drop_shadow.dst_rect.set_x(t_x - 3);
            text_drop_shadow.dst_rect.set_y(t_y - 3);

            // actual text
            let ts = text::TimedSurface::new(text_drop_shadow, 1000);
            text::push_timed_surface(ts).unwrap();

            let details = self.equipment.get_details();
            let mut text_colored = text::create_text_overlay_surface(text::TextInput {
                text: format!("{}: {}", details.name.to_uppercase(), details.action),
                mode: text::Mode::Solid {
                    color: *details.font_color,
                },
                font: g_game::get_text_font_lg().unwrap(),
            })
            .unwrap();

            text_colored.dst_rect.set_x(t_x);
            text_colored.dst_rect.set_y(t_y);

            let ts = text::TimedSurface::new(text_colored, 1000);
            text::push_timed_surface(ts).unwrap();
        }

    }

    pub fn draw_model(&mut self) {
        let ref_dec = g_game::get_ref_entity(self.base.ref_id).unwrap();

            let mat_s =
                raymath::matrix_scale(
                    self.base.scale[0],
                    self.base.scale[1],
                    self.base.scale[2]
                );
            
            let mat_r = raymath::quaternion_to_matrix(self.base.rotation.into());

            let pos = Vector3::new(
                self.base.location[0],
                self.base.location[1],
                self.base.location[2]
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

        let dc = render::DrawCall {
            matrix: mat,
            texture: ref_dec.texture_handle as u32,
            f1: ref_dec.frame_handles[0] as i32,
            f2: ref_dec.frame_handles[0] as i32,
            mix: 0.,
            num_verts: ref_dec.num_verts,
            glow: None,
        };
        render::draw(dc).unwrap();
    }

    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        panic!("don't fetch entity meshes")
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        panic!("don't fetch entity meshes")
    }
}
