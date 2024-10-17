use crate::d_floor::Floor;
use crate::d_platform::Platform;

use crate::e_barrier::Barrier;
use crate::e_gcyl::Gcyl;
use crate::e_light::Light;
use crate::e_menu::Menu;
use crate::e_pig::Pig;
use crate::e_player::Player;

use crate::g_game;
use crate::map::{Entity, LoadedEnttReference};
use crate::nuerror::NUError;

use raymath::{vector3_distance, vector3_normalize, vector3_subtract, Vector3};

use crate::math;

pub enum Instance {
    // Decor
    DFloor(Floor),
    DPlatform(Platform),
    // Entities
    EBarrier(Barrier),
    EGcyl(Gcyl),
    ELight(Light),
    EPlayer(Player),
    EPig(Pig),
    EMenuM(Menu),
    EMenuE(Menu),
    EMenuN(Menu),
    EMenuU(Menu),
    ETriggerLevelChange, // todo
}

// todo -- perf
// cache these lookups, probably perform the cache at map_load
pub fn ref_ent_from_str(s: &str) -> Option<LoadedEnttReference> {
    let ents = g_game::get_map_ref_ents().unwrap();
    let erns = g_game::get_map_ern_data().unwrap();

    let mut out = None;
    for (i, ern) in erns.iter().enumerate() {
        if s == ern {
            out = Some(ents[i].clone())
        }
    }

    out
}

pub fn instance_from_str(s: &str, entt: &Entity) -> Option<Instance> {
    match s {
        // decor
        "floor" => Some(Instance::DFloor(Floor::new(entt))),
        "platform" => Some(Instance::DPlatform(Platform::new(entt))),
        // entities
        "barrier" => Some(Instance::EBarrier(Barrier::new(entt))),
        "gcyl" => Some(Instance::EGcyl(Gcyl::new(entt))),
        "light" => Some(Instance::ELight(Light::new(entt))),
        "player" => Some(Instance::EPlayer(Player::new(entt))),
        "pig" => Some(Instance::EPig(Pig::new(entt))),
        "menu_m" => Some(Instance::EMenuM(Menu::new(entt))),
        "menu_e" => Some(Instance::EMenuE(Menu::new(entt))),
        "menu_n" => Some(Instance::EMenuN(Menu::new(entt))),
        "menu_u" => Some(Instance::EMenuU(Menu::new(entt))),
        "trigger_levelchange" => None,
        unknown => {
            eprintln!("unrecognized entity '{}'", unknown);
            None
        }
    }
}

impl Instance {
    pub fn update(&mut self) {
        match self {
            Self::DFloor(e) => e.update(),
            Self::DPlatform(e) => e.update(),
            Self::EBarrier(e) => e.update(),
            Self::EGcyl(e) => e.update(),
            Self::ELight(e) => e.update(),
            Self::EPlayer(e) => e.update(),
            Self::EPig(e) => e.update(),
            Self::EMenuM(e) => e.update(),
            Self::EMenuE(e) => e.update(),
            Self::EMenuN(e) => e.update(),
            Self::EMenuU(e) => e.update(),
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }

    pub fn draw_model(&mut self) {
        match self {
            Self::DFloor(e) => e.draw_model(),
            Self::DPlatform(e) => e.draw_model(),
            Self::EBarrier(e) => e.draw_model(),
            Self::EGcyl(e) => e.draw_model(),
            Self::ELight(e) => e.draw_model(),
            Self::EPlayer(e) => e.draw_model(),
            Self::EPig(e) => e.draw_model(),
            Self::EMenuM(e) => e.draw_model(),
            Self::EMenuE(e) => e.draw_model(),
            Self::EMenuN(e) => e.draw_model(),
            Self::EMenuU(e) => e.draw_model(),
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }

    pub fn is_decor(&mut self) -> bool {
        match self {
            Self::DFloor(_) => true,
            Self::DPlatform(_) => true,
            Self::EBarrier(_) => false,
            Self::EGcyl(_) => false,
            Self::ELight(_) => false,
            Self::EPlayer(_) => false,
            Self::EPig(_) => false,
            Self::EMenuM(_) => false,
            Self::EMenuE(_) => false,
            Self::EMenuN(_) => false,
            Self::EMenuU(_) => false,
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }

    pub fn get_mesh(&mut self) -> Vec<[raymath::Vector3; 3]> {
        match self {
            // get meshes for decor
            Self::DFloor(e) => e.get_mesh(),
            Self::DPlatform(e) => e.get_mesh(),
            // rest will panic
            Self::EBarrier(e) => e.get_mesh(),
            Self::EGcyl(e) => e.get_mesh(),
            Self::ELight(e) => e.get_mesh(),
            Self::EPlayer(e) => e.get_mesh(),
            Self::EPig(e) => e.get_mesh(),
            Self::EMenuM(e) => e.get_mesh(),
            Self::EMenuE(e) => e.get_mesh(),
            Self::EMenuN(e) => e.get_mesh(),
            Self::EMenuU(e) => e.get_mesh(),
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }

    pub fn get_matrix(&mut self) -> raymath::Matrix {
        match self {
            // get mat for decor
            Self::DFloor(e) => e.get_matrix(),
            Self::DPlatform(e) => e.get_matrix(),
            // rest will panic
            Self::EBarrier(e) => e.get_matrix(),
            Self::EGcyl(e) => e.get_matrix(),
            Self::ELight(e) => e.get_matrix(),
            Self::EPlayer(e) => e.get_matrix(),
            Self::EPig(e) => e.get_matrix(),
            Self::EMenuM(e) => e.get_matrix(),
            Self::EMenuE(e) => e.get_matrix(),
            Self::EMenuN(e) => e.get_matrix(),
            Self::EMenuU(e) => e.get_matrix(),
            Self::ETriggerLevelChange => {
                panic!("unimplemented")
            }
        }
    }
}

pub fn pos_is_visible(cam_pos: Vector3, point: Vector3) -> bool {
    let decs = get_decor_instances().unwrap();
    let dir = vector3_normalize(vector3_subtract(point, cam_pos));
    let distance = vector3_distance(cam_pos, point);
    let ray = raymath::Ray {
        position: cam_pos,
        direction: dir,
    };

    // find nearest decor collision
    for dec in decs {
        let dec = &mut *dec;
        let mesh = dec.get_mesh();
        let mat = dec.get_matrix();

        // max distance hack. exclude 
        if vector3_distance(cam_pos, point) > 64. {
            return false;
        }

        let coll = math::get_ray_collision_mesh(ray, mesh, mat, None);
        // collides before reaching point
        if coll.hit && coll.distance < distance {
            return false;
        }
    }

    true
}

pub fn get_decor_instances<'a>() -> Result<Vec<&'a mut Instance>, NUError> {
    g_game::get_filtered_instances(|inst| inst.is_decor())
}

pub fn get_barrier_instances<'a>() -> Result<Vec<&'a mut Instance>, NUError> {
    g_game::get_filtered_instances(|inst| match inst {
        Instance::EBarrier(_) => true,
        _ => false,
    })
}
