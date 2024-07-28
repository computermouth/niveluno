use crate::e_entity::EntityInstance;
use crate::NUError;

use crate::level;

use crate::e_gcyl::Gcyl;
use crate::e_light::Light;
use crate::e_player::Player;

struct GameGod {
    pub current_level: Option<level::Level>,
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
        });
    }

    Ok(())
}

pub fn get_time() -> f32 {
    1.0
}

pub fn set_and_init_level(level: level::Level) -> Result<(), NUError> {
    let gg = GameGod::get()?;

    for md in &level.map_decor {
        match level.payload.drn_data[md.ref_id].as_str() {
            "floor" => {}
            unknown => {
                eprintln!("unrecognized decor '{}'", unknown)
            }
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
    Ok(())
}

pub fn run() -> Result<(), NUError> {
    Ok(())
}
