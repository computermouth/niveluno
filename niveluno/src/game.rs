use crate::NUError;

use crate::level;

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

pub fn set_level(level: level::Level) -> Result<(), NUError> {
    let gg = GameGod::get()?;
    gg.current_level = Some(level);
    Ok(())
}
