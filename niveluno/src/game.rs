use crate::level::{LoadedDecorReference, LoadedEnttReference};
use crate::NUError;

use crate::level;

use crate::e_entity::EntityInstance;
use crate::e_gcyl::Gcyl;
use crate::e_light::Light;
use crate::e_player::Player;

use crate::d_decor::DecorInstance;
use crate::d_floor::Floor;

struct GameGod {
    pub current_level: Option<level::Level>,
    pub decor_inst: Vec<Box<dyn DecorInstance>>,
    pub entts_inst: Vec<Box<dyn EntityInstance>>,
}

impl GameGod {
    pub fn get() -> Result<&'static mut GameGod, NUError> {
        unsafe {
            GAME_GOD
                .as_mut()
                .ok_or_else(|| NUError::MiscError("GAME_GOD uninit".to_string()))
        }
    }
}

static mut GAME_GOD: Option<GameGod> = None;

pub fn init() -> Result<(), NUError> {
    if GameGod::get().is_ok() {
        return Err(NUError::MiscError("GAME_GOD already init".to_string()));
    }

    unsafe {
        GAME_GOD = Some(GameGod {
            current_level: None,
            decor_inst: vec![],
            entts_inst: vec![],
        });
    }

    Ok(())
}

pub fn get_time() -> f32 {
    1.0
}

pub fn set_and_init_level(level: level::Level) -> Result<(), NUError> {
    let gg = GameGod::get()?;

    let mut decor = vec![];
    for md in &level.map_decor {
        let dyn_decor_inst: Option<Box<dyn DecorInstance>> =
            match level.payload.drn_data[md.ref_id].as_str() {
                "floor" => Some(Box::new(Floor::new(md))),
                unknown => {
                    eprintln!("unrecognized decor '{}'", unknown);
                    None
                }
            };
        if dyn_decor_inst.is_some() {
            decor.push(dyn_decor_inst.unwrap());
        }
    }

    let mut entts = vec![];
    for me in &level.map_entities {
        let dyn_entt_inst: Option<Box<dyn EntityInstance>> =
            match level.payload.ern_data[me.index].as_str() {
                "gcyl" => Some(Box::new(Gcyl::new(me))),
                "light" => Some(Box::new(Light::new(me))),
                "player" => Some(Box::new(Player::new(me))),
                "trigger_levelchange" => None,
                unknown => {
                    eprintln!("unrecognized entity '{}'", unknown);
                    None
                }
            };
        if dyn_entt_inst.is_some() {
            entts.push(dyn_entt_inst.unwrap());
        }
    }

    gg.current_level = Some(level);
    gg.decor_inst = decor;
    gg.entts_inst = entts;

    Ok(())
}

pub fn run() -> Result<(), NUError> {
    let gg = GameGod::get()?;

    for decor in &mut gg.decor_inst {
        decor.update();
    }

    for entt in &mut gg.entts_inst {
        entt.update();
    }

    for decor in &mut gg.decor_inst {
        decor.draw_model();
    }

    for entt in &mut gg.entts_inst {
        entt.draw_model();
    }

    Ok(())
}

pub fn get_ref_decor(id: usize) -> Result<LoadedDecorReference, NUError> {
    let gg = GameGod::get()?;

    let level = &gg.current_level;
    let level = level
        .as_ref()
        .ok_or(NUError::MiscError("level not set".to_string()))?;

    if id >= level.ref_decor.len() {
        return Err(NUError::MiscError("id exceeds ref_decor len".to_string()));
    }

    // it'd be nice if this was a reference
    Ok(level.ref_decor[id].clone())
}

pub fn get_ref_entity(id: usize) -> Result<LoadedEnttReference, NUError> {
    let gg = GameGod::get()?;

    let level = &gg.current_level;
    let level = level
        .as_ref()
        .ok_or(NUError::MiscError("level not set".to_string()))?;

    if id >= level.ref_entities.len() {
        return Err(NUError::MiscError("id exceeds ref_entts len".to_string()));
    }

    // it'd be nice if this was a reference
    Ok(level.ref_entities[id].clone())
}
