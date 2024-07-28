use crate::NUError;

use crate::level::{self, Entity, Decor};

struct GameGod {
    pub current_level: Option<level::Level>,
}

enum MapInstance {
    Entity(Entity),
    Decor(Decor),
}

struct GameInstances {
    pub entities: Vec<Entity>,
    pub decors: Vec<Decor>
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
            "floor" => {},
            unknown => { eprintln!("unrecognized decor '{}'", unknown) },
        }
    }

    for me in &level.map_entities {
        match level.payload.ern_data[me.index].as_str() {
            "player" => {},
            "light" => {},
            "trigger_levelchange" => {},
            "gcyl" => {},
            unknown => { eprintln!("unrecognized entity '{}'", unknown) },
        }
    }

    gg.current_level = Some(level);
    Ok(())
}

pub fn run() -> Result<(), NUError> {
    Ok(())
}