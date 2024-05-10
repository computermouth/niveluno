use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::{MouseButton, MouseUtil};
use sdl2::video::{FullscreenType, Window};
use sdl2::EventPump;

use crate::render;
use crate::NUError;

pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Prev,
    Next,
    Action,
    Jump,
}

struct InputGod {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub last_wheel_event: f32,
    pub mouse_speed: isize,
    pub mouse_invert: bool,
    pub quit: bool,
    pub keys: [bool; 8],
    pub fullscreen: bool,
}

impl InputGod {
    pub fn get() -> Result<&'static mut InputGod, NUError> {
        unsafe {
            INPUT_GOD
                .as_mut()
                .ok_or_else(|| NUError::MiscError("INPUT_GOD uninit".to_string()))
        }
    }
}

static mut INPUT_GOD: Option<InputGod> = None;

pub fn init() -> Result<(), NUError> {
    if InputGod::get().is_ok() {
        return Err(NUError::MiscError("INPUT_GOD already init".to_string()));
    }

    let ig = InputGod {
        mouse_x: 0.,
        mouse_y: 0.,
        last_wheel_event: 0.,
        mouse_speed: 10, // 0-50 // todo, verify
        mouse_invert: false,
        quit: false,
        keys: [false; 8],
        fullscreen: false,
    };

    unsafe { INPUT_GOD = Some(ig) }

    Ok(())
}

#[rustfmt::skip]
pub fn consume(window: &mut Window, mouse: &MouseUtil, event_pump: &mut EventPump) -> Result<(), NUError> {

    let ig = InputGod::get()?;

    let current_states: Vec<_> = event_pump.keyboard_state().scancodes().collect();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} => { ig.quit = true },
            Event::KeyDown {
                keycode: Some(key),
                repeat: false,
                ..
            } => {
                match key {
                    Keycode::Up    | Keycode::W => { ig.keys[Key::Up    as usize] = true; },
                    Keycode::Left  | Keycode::A => { ig.keys[Key::Left  as usize] = true; },
                    Keycode::Down  | Keycode::S => { ig.keys[Key::Down  as usize] = true; },
                    Keycode::Right | Keycode::D => { ig.keys[Key::Right as usize] = true; },
                                     Keycode::Q => { ig.keys[Key::Prev  as usize] = true; },
                                     Keycode::E => { ig.keys[Key::Next  as usize] = true; },
                    Keycode::Space              => { ig.keys[Key::Jump  as usize] = true; },
                    _ => {}
                }
            },
            Event::KeyUp {
                keycode: Some(key),
                ..
            } => {
                match key {
                    Keycode::Escape             => { mouse.set_relative_mouse_mode(false); },
                    Keycode::Up    | Keycode::W => { ig.keys[Key::Up    as usize] = false; },
                    Keycode::Left  | Keycode::A => { ig.keys[Key::Left  as usize] = false; },
                    Keycode::Down  | Keycode::S => { ig.keys[Key::Down  as usize] = false; },
                    Keycode::Right | Keycode::D => { ig.keys[Key::Right as usize] = false; },
                                     Keycode::Q => { ig.keys[Key::Prev  as usize] = false; },
                                     Keycode::E => { ig.keys[Key::Next  as usize] = false; },
                    Keycode::Space              => { ig.keys[Key::Jump  as usize] = false; },
                    Keycode::Return             => {                        
                        if current_states.contains(&(Scancode::RAlt, true)) || 
                            current_states.contains(&(Scancode::LAlt, true)) {
                            
                            let mut fs = FullscreenType::Desktop;
                            if ig.fullscreen {
                                fs = FullscreenType::Off;
                            }
                            ig.fullscreen = !ig.fullscreen;
                            window.set_fullscreen(fs).map_err(|e| NUError::SDLError(e))?;
                        }
                    },
                    _ => {}
                }
            },
            Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                if mouse.relative_mouse_mode() == false {
                    mouse.set_relative_mouse_mode(true);
                }
                ig.keys[Key::Action as usize] = true;
            },
            Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                ig.keys[Key::Action as usize] = false;
            },
            Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => {
                ig.keys[Key::Jump as usize] = true;
            },
            Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => {
                ig.keys[Key::Jump as usize] = false;
            }
            Event::MouseWheel { y, .. } => {
                match y {
                    std::i32::MIN..=-1 => { ig.keys[Key::Prev as usize] = true },
                    1..=std::i32::MAX  => { ig.keys[Key::Next as usize] = true },
                    _ => {}
                }
            }
            Event::MouseMotion { xrel, yrel, .. } => {
                ig.mouse_x += xrel as f32;
                ig.mouse_y += yrel as f32;
            }
            Event::Window { win_event, .. } => {
                match win_event {
                    WindowEvent::FocusLost => {
                        mouse.set_relative_mouse_mode(false);
                    }
                    WindowEvent::SizeChanged(w, h) => {
                        render::change_window_size(w, h)?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn get_quit() -> Result<bool, NUError> {
    Ok(InputGod::get()?.quit)
}

pub fn get_mouse() -> (f32, f32) {
    (1., 1.)
}
