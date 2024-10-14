use crate::asset;
use crate::g_instance::Instance;
use crate::map::{self, LoadedEnttReference};
use crate::nuerror::NUError;
use crate::text;

struct GameGod {
    pub current_level: Option<map::Map>,
    pub next_level: Option<map::Map>,
    pub entts_inst: Vec<Instance>,
    pub top_state: TopState,
    pub text_font_lg: Option<text::SizedFontHandle>,
    pub text_font_sm: Option<text::SizedFontHandle>,
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

#[derive(Copy, Clone, PartialEq)]
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
            next_level: None,
            entts_inst: vec![],
            top_state: TopState::Menu,
            text_font_lg: None,
            text_font_sm: None,
            symb_font: None,
        });
    }

    let gg = GameGod::get()?;

    let lib_mono_bold_bytes = asset::get_file("ttf/LiberationMono-Bold.ttf")?
        .ok_or_else(|| NUError::MiscError("libmonobold not found".to_string()))?;
    let lib_mono_bold_font = text::push_font(lib_mono_bold_bytes)?;
    gg.text_font_lg = Some(text::create_sized_font(lib_mono_bold_font, 32)?);
    gg.text_font_sm = Some(text::create_sized_font(lib_mono_bold_font, 16)?);

    let nerd_symbols_bytes = asset::get_file("ttf/SymbolsNerdFontMono-Regular.ttf")?
        .ok_or_else(|| NUError::MiscError("nerd_symbols not found".to_string()))?;
    let nerd_symbols_font = text::push_font(nerd_symbols_bytes)?;
    gg.symb_font = Some(text::create_sized_font(nerd_symbols_font, 24)?);

    let menu = asset::get_file("map/menu.mp")?
        .ok_or_else(|| NUError::MiscError("menu map not found".to_string()))?;
    let payload = mparse::unmarshal(&menu).unwrap();
    let level = map::load(payload)?;
    stage_level(level.clone())?;

    Ok(())
}

pub fn get_text_font_lg() -> Result<text::SizedFontHandle, NUError> {
    let gg = GameGod::get()?;
    Ok(gg.text_font_lg.unwrap())
}

pub fn get_text_font_sm() -> Result<text::SizedFontHandle, NUError> {
    let gg = GameGod::get()?;
    Ok(gg.text_font_sm.unwrap())
}

pub fn get_symb_font() -> Result<text::SizedFontHandle, NUError> {
    let gg = GameGod::get()?;
    Ok(gg.symb_font.unwrap())
}

pub fn get_state() -> Result<TopState, NUError> {
    let gg = GameGod::get()?;
    Ok(gg.top_state)
}

pub fn set_state(ts: TopState) -> Result<(), NUError> {
    let gg = GameGod::get()?;
    gg.top_state = ts;
    Ok(())
}

pub fn stage_level(level: map::Map) -> Result<(), NUError> {
    let gg = GameGod::get()?;
    gg.next_level = Some(level);
    Ok(())
}

pub fn init_level(level: &map::Map) -> Result<(), NUError> {
    let gg = GameGod::get()?;

    // let mut animations = HashMap::new();
    for mr in &level.ref_entities {
        match level.payload.ern_data[mr.index].as_str() {
            s => eprintln!("mr ern: {s}"),
        }
    }

    let mut entts = vec![];
    for me in &level.map_entities {
        let entt_inst = Instance::from_str(level.payload.ern_data[me.ref_id].as_str(), me);
        if entt_inst.is_some() {
            entts.push(entt_inst.unwrap());
        }
    }

    gg.entts_inst = entts;

    Ok(())
}

pub fn run() -> Result<(), NUError> {
    let gg = GameGod::get()?;

    // only swap in/out entities outside of the update loop
    if gg.next_level.is_some() {
        gg.current_level = gg.next_level.take();
        init_level(gg.current_level.as_ref().unwrap())?;
    }

    for entt in &mut gg.entts_inst {
        entt.update();
    }

    for entt in &mut gg.entts_inst {
        entt.draw_model();
    }

    Ok(())
}

pub fn get_param<'a>(id: usize) -> Result<&'a str, NUError> {
    let gg = GameGod::get()?;

    let level = &gg.current_level;
    let level = level
        .as_ref()
        .ok_or(NUError::MiscError("level not set".to_string()))?;

    if id >= level.payload.kvs_data.len() {
        return Err(NUError::MiscError("id exceeds kvs_data len".to_string()));
    }

    Ok(&level.payload.kvs_data[id])
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

pub fn get_animation_ids(animations: &[&[&str]], ref_ent: &LoadedEnttReference) -> Vec<Vec<usize>> {
    let mut animation_ids = vec![];

    for animation in animations {
        let mut frame_ids = vec![];
        for frame in *animation {
            let id = ref_ent.frame_names.iter().position(|r| r == frame);

            let id = match id {
                Some(i) => i,
                None => {
                    eprintln!("frame '{frame}' not matched");
                    0
                }
            };

            frame_ids.push(id);
        }
        animation_ids.push(frame_ids);
    }

    animation_ids
}

pub fn get_filtered_instances<'a, F>(filter_fn: F) -> Result<Vec<&'a mut Instance>, NUError>
where
    F: Fn(&mut Instance) -> bool, // Change to accept mutable reference
{
    let gg = GameGod::get()?;
    let all = &mut gg.entts_inst;

    let mut filtered: Vec<&'a mut Instance> = Vec::new();

    for instance in all.iter_mut() {
        if filter_fn(instance) {
            let instance_ptr = instance as *mut Instance;
            unsafe {
                filtered.push(&mut *instance_ptr);
            }
        }
    }

    Ok(filtered)
}

pub fn get_decor_instances<'a>() -> Result<Vec<&'a mut Instance>, NUError> {
    get_filtered_instances(|inst| inst.is_decor()) // Now works correctly with mutable reference
}
