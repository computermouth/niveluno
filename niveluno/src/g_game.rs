use crate::asset;
use crate::map::{self, LoadedDecorReference, LoadedEnttReference};
use crate::nuerror::NUError;
use crate::text;

use crate::e_entity::EntityInstance;
use crate::e_gcyl::Gcyl;
use crate::e_light::Light;
use crate::e_menu::Menu;
use crate::e_player::Player;

use crate::d_decor::DecorInstance;
use crate::d_floor::Floor;

struct GameGod {
    pub current_level: Option<map::Map>,
    pub decor_inst: Vec<Box<dyn DecorInstance>>,
    pub entts_inst: Vec<Box<dyn EntityInstance>>,
    pub top_state: TopState,
    pub text_font: Option<text::SizedFontHandle>,
    pub symb_font: Option<text::SizedFontHandle>,
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

#[derive(Copy, Clone)]
pub enum TopState {
    Menu,
    Play,
}

pub fn init() -> Result<(), NUError> {
    if GameGod::get().is_ok() {
        return Err(NUError::MiscError("GAME_GOD already init".to_string()));
    }

    unsafe {
        GAME_GOD = Some(GameGod {
            current_level: None,
            decor_inst: vec![],
            entts_inst: vec![],
            top_state: TopState::Menu,
            text_font: None,
            symb_font: None,
        });
    }

    let gg = GameGod::get()?;

    let lib_mono_bold_bytes = asset::get_file("ttf/LiberationMono-Bold.ttf")?
        .ok_or_else(|| NUError::MiscError("libmonobold not found".to_string()))?;
    let lib_mono_bold_font = text::push_font(lib_mono_bold_bytes)?;
    gg.text_font = Some(text::create_sized_font(lib_mono_bold_font, 32)?);

    let nerd_symbols_bytes = asset::get_file("ttf/SymbolsNerdFontMono-Regular.ttf")?
        .ok_or_else(|| NUError::MiscError("nerd_symbols not found".to_string()))?;
    let nerd_symbols_font = text::push_font(nerd_symbols_bytes)?;
    gg.symb_font = Some(text::create_sized_font(nerd_symbols_font, 24)?);

    let menu = asset::get_file("map/menu.mp")?
        .ok_or_else(|| NUError::MiscError("menu map not found".to_string()))?;
    let payload = mparse::unmarshal(&menu).unwrap();
    let level = map::load(payload)?;
    set_and_init_level(level.clone())?;

    Ok(())
}

pub fn get_text_font() -> Result<text::SizedFontHandle, NUError> {
    let gg = GameGod::get()?;
    Ok(gg.text_font.unwrap())
}

pub fn get_symb_font() -> Result<text::SizedFontHandle, NUError> {
    let gg = GameGod::get()?;
    Ok(gg.symb_font.unwrap())
}

pub fn get_state() -> Result<TopState, NUError> {
    let gg = GameGod::get()?;
    Ok(gg.top_state)
}

pub fn set_and_init_level(level: map::Map) -> Result<(), NUError> {
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
                "menu_m" => Some(Box::new(Menu::new(me))),
                "menu_e" => Some(Box::new(Menu::new(me))),
                "menu_n" => Some(Box::new(Menu::new(me))),
                "menu_u" => Some(Box::new(Menu::new(me))),
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

    match gg.top_state {
        TopState::Menu => {}
        TopState::Play => {}
    }

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
    // do I have to use Rc?? :(
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
