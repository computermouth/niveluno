use sdl2::mixer::{Chunk, Music};
use std::collections::HashMap;

use crate::nuerror::NUError;

struct AudioGod<'a> {
    pub ctx: sdl2::mixer::Sdl2MixerContext,
    pub sfx_map: HashMap<String, Chunk>,
    pub mus_map: HashMap<String, Music<'a>>,
}

impl<'a> AudioGod<'a> {
    pub fn get() -> Result<&'static mut AudioGod<'a>, NUError> {
        unsafe {
            AUDIO_GOD
                .as_mut()
                .ok_or_else(|| NUError::MiscError("AUDIO_GOD uninit".to_string()))
        }
    }
}

static mut AUDIO_GOD: Option<AudioGod> = None;

pub fn init() -> Result<(), NUError> {
    if AudioGod::get().is_ok() {
        return Err(NUError::MiscError("AUDIO_GOD already init".to_string()));
    }

    let mut flags = sdl2::mixer::InitFlag::empty();
    flags.insert(sdl2::mixer::InitFlag::OGG);

    let ag = AudioGod {
        ctx: sdl2::mixer::init(flags).map_err(|e| NUError::SDLError(e.to_string()))?,
        sfx_map: HashMap::new(),
        mus_map: HashMap::new(),
    };

    unsafe { AUDIO_GOD = Some(ag) }

    Ok(())
}

pub fn load_sfx() -> Result<(), NUError> {
    Ok(())
}

pub fn load_mus() -> Result<(), NUError> {
    Ok(())
}
