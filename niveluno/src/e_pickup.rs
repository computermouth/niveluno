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

    const FLAG_ENTITY_NAME: &'static str = "medieval.flag_blue";
    const FLAG_NAME: &'static str = "flag";
    const FLAG_CHAR: char = '\u{f024}';

    const CANDY_CANE_ENTITY_NAME: &'static str = "xmas.candycane_small";
    const CANDY_CANE_NAME: &'static str = "candy_cane";
    const CANDY_CANE_CHAR: char = '\u{ef3a}';

    const WRENCH_ENTITY_NAME: &'static str = "tools.wrench_B";
    const WRENCH_NAME: &'static str = "wrench";
    const WRENCH_CHAR: char = '\u{f0ad}';

    const ICE_CREAM_ENTITY_NAME: &'static str = "restaurant.food_icecream_cone_vanilla";
    const ICE_CREAM_NAME: &'static str = "ice_cream";
    const ICE_CREAM_CHAR: char = '\u{ef88}';

    const BOTTLE_ENTITY_NAME: &'static str = "dungeon.bottle_C_green";
    const BOTTLE_NAME: &'static str = "bottle";
    const BOTTLE_CHAR: char = '\u{f1132}';

    const APPLE_ENTITY_NAME: &'static str = "resource.Food_Apple_Red";
    const APPLE_NAME: &'static str = "apple";
    const APPLE_CHAR: char = '\u{e29e}';

    const BOOK_ENTITY_NAME: &'static str = "tools.journal_open";
    const BOOK_NAME: &'static str = "book";
    const BOOK_CHAR: char = '\u{ede2}';

    const KNIFE_ENTITY_NAME: &'static str = "restaurant.knife";
    const KNIFE_NAME: &'static str = "knife";
    const KNIFE_CHAR: char = '\u{f09fb}';

    const COOKIE_ENTITY_NAME: &'static str = "xmas.cookie";
    const COOKIE_NAME: &'static str = "cookie";
    const COOKIE_CHAR: char = '\u{f0198}';

    const KEY_ENTITY_NAME: &'static str = "tools.key_B";
    const KEY_NAME: &'static str = "key";
    const KEY_CHAR: char = '\u{f084}';

    const ANCHOR_ENTITY_NAME: &'static str = "medieval.anchor";
    const ANCHOR_NAME: &'static str = "anchor";
    const ANCHOR_CHAR: char = '\u{f0031}';

    const CANDLE_ENTITY_NAME: &'static str = "dungeon.candle_thin_lit";
    const CANDLE_NAME: &'static str = "candle";
    const CANDLE_CHAR: char = '\u{f05e2}';


    pub fn get_details(&self) -> EquipmentDetails {
        match self {
            &Equipment::Pistol => EquipmentDetails { name: Self::PISTOL_NAME, icon: Self::PISTOL_CHAR },
            &Equipment::Flag => EquipmentDetails { name: Self::FLAG_NAME, icon: Self::FLAG_CHAR },
            &Equipment::CandyCane => EquipmentDetails { name: Self::CANDY_CANE_NAME, icon: Self::CANDY_CANE_CHAR },
            &Equipment::Wrench => EquipmentDetails { name: Self::WRENCH_NAME, icon: Self::WRENCH_CHAR },
            &Equipment::IceCream => EquipmentDetails { name: Self::ICE_CREAM_NAME, icon: Self::ICE_CREAM_CHAR },
            &Equipment::Bottle => EquipmentDetails { name: Self::BOTTLE_NAME, icon: Self::BOTTLE_CHAR },
            &Equipment::Apple => EquipmentDetails { name: Self::APPLE_NAME, icon: Self::APPLE_CHAR },
            &Equipment::Book => EquipmentDetails { name: Self::BOOK_NAME, icon: Self::BOOK_CHAR },
            &Equipment::Knife => EquipmentDetails { name: Self::KNIFE_NAME, icon: Self::KNIFE_CHAR },
            &Equipment::Cookie => EquipmentDetails { name: Self::COOKIE_NAME, icon: Self::COOKIE_CHAR },
            &Equipment::Key => EquipmentDetails { name: Self::KEY_NAME, icon: Self::KEY_CHAR },
            &Equipment::Anchor => EquipmentDetails { name: Self::ANCHOR_NAME, icon: Self::ANCHOR_CHAR },
            &Equipment::Candle => EquipmentDetails { name: Self::CANDLE_NAME, icon: Self::CANDLE_CHAR },
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

        eprintln!("equip param len: {}", entt.params.len());

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

            let mut spawn = text::create_text_overlay_surface(text::TextInput {
                text: format!("{} ACQUIRED", self.equipment.get_details().name),
                mode: text::Mode::Solid {
                    color: text::FontColor {
                        r: 64,
                        g: 32,
                        b: 196,
                        a: 255,
                    },
                },
                font: g_game::get_text_font_lg().unwrap(),
            })
            .unwrap();

            spawn.dst_rect.set_x(200);
            spawn.dst_rect.set_y(200);

            let ts = text::TimedSurface::new(spawn, 1000);
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
