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
    Sprint,
}

struct InputGod {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_speed: f32,
    pub mouse_invert: bool,
    pub quit: bool,
    pub keys_pressed: [bool; 9],
    pub keys_down: [bool; 9],
    pub keys_released: [bool; 9],
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
        mouse_speed: 10., // 0-50 // todo, verify
        mouse_invert: false,
        quit: false,
        keys_pressed: [false; 9],
        keys_down: [false; 9],
        keys_released: [false; 9],
        fullscreen: false,
    };

    unsafe { INPUT_GOD = Some(ig) }

    Ok(())
}

#[rustfmt::skip]
pub fn consume(window: &mut Window, mouse: &MouseUtil, event_pump: &mut EventPump) -> Result<(), NUError> {

    let ig = InputGod::get()?;

    ig.mouse_x = 0.;
    ig.mouse_y = 0.;

    // something's up with later versions of sdl2 here, it panics on values from the reserved
    // range, so now we're going to check scancodes individually
    let kb = event_pump.keyboard_state();
    let alt_pressed =
        kb.is_scancode_pressed(Scancode::LAlt) || kb.is_scancode_pressed(Scancode::RAlt);

    ig.keys_pressed.fill(false);
    // ig.keys_down.fill(false);
    ig.keys_released.fill(false);

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} => { ig.quit = true },
            Event::KeyDown {
                keycode: Some(key),
                repeat: false,
                ..
            } => {
                match key {
                    Keycode::Up    | Keycode::W => { ig.keys_down[Key::Up     as usize] = true; ig.keys_pressed[Key::Up     as usize] = true; },
                    Keycode::Left  | Keycode::A => { ig.keys_down[Key::Left   as usize] = true; ig.keys_pressed[Key::Left   as usize] = true; },
                    Keycode::Down  | Keycode::S => { ig.keys_down[Key::Down   as usize] = true; ig.keys_pressed[Key::Down   as usize] = true; },
                    Keycode::Right | Keycode::D => { ig.keys_down[Key::Right  as usize] = true; ig.keys_pressed[Key::Right  as usize] = true; },
                                     Keycode::Q => { ig.keys_down[Key::Prev   as usize] = true; ig.keys_pressed[Key::Prev   as usize] = true; },
                                     Keycode::E => { ig.keys_down[Key::Next   as usize] = true; ig.keys_pressed[Key::Next   as usize] = true; },
                    Keycode::Space              => { ig.keys_down[Key::Jump   as usize] = true; ig.keys_pressed[Key::Jump   as usize] = true; },
                    Keycode::LShift             => { ig.keys_down[Key::Sprint as usize] = true; ig.keys_pressed[Key::Sprint as usize] = true; }
                    _ => {}
                }
            },
            Event::KeyUp {
                keycode: Some(key),
                ..
            } => {
                match key {
                    Keycode::Escape             => { mouse.set_relative_mouse_mode(false); },
                    Keycode::Up    | Keycode::W => { ig.keys_down[Key::Up     as usize] = false; ig.keys_released[Key::Up     as usize] = true;},
                    Keycode::Left  | Keycode::A => { ig.keys_down[Key::Left   as usize] = false; ig.keys_released[Key::Left   as usize] = true;},
                    Keycode::Down  | Keycode::S => { ig.keys_down[Key::Down   as usize] = false; ig.keys_released[Key::Down   as usize] = true;},
                    Keycode::Right | Keycode::D => { ig.keys_down[Key::Right  as usize] = false; ig.keys_released[Key::Right  as usize] = true;},
                                     Keycode::Q => { ig.keys_down[Key::Prev   as usize] = false; ig.keys_released[Key::Prev   as usize] = true;},
                                     Keycode::E => { ig.keys_down[Key::Next   as usize] = false; ig.keys_released[Key::Next   as usize] = true;},
                    Keycode::Space              => { ig.keys_down[Key::Jump   as usize] = false; ig.keys_released[Key::Jump   as usize] = true;},
                    Keycode::LShift             => { ig.keys_down[Key::Sprint as usize] = false; ig.keys_released[Key::Sprint as usize] = true;}
                    Keycode::Return if alt_pressed => {
                        let mut fs = FullscreenType::Desktop;
                        if ig.fullscreen {
                            fs = FullscreenType::Off;
                        }
                        ig.fullscreen = !ig.fullscreen;
                        window.set_fullscreen(fs).map_err(|e| NUError::SDLError(e))?;
                    },
                    _ => {}
                }
            },
            Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                if mouse.relative_mouse_mode() == false {
                    mouse.set_relative_mouse_mode(true);
                }
                ig.keys_down[Key::Action as usize] = true;
            },
            Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                ig.keys_down[Key::Action as usize] = false;
            },
            Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => {
                ig.keys_down[Key::Jump as usize] = true;
            },
            Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => {
                ig.keys_down[Key::Jump as usize] = false;
            }
            Event::MouseWheel { y, .. } => {
                match y {
                    std::i32::MIN..=-1 => { ig.keys_down[Key::Next as usize] = true; ig.keys_pressed[Key::Next   as usize] = true; },
                    1..=std::i32::MAX  => { ig.keys_down[Key::Prev as usize] = true; ig.keys_pressed[Key::Prev   as usize] = true; },
                    _ => {}
                }
            }
            Event::MouseMotion { xrel, yrel, .. } => {

                if mouse.relative_mouse_mode() == true {
                    let f_invert = match ig.mouse_invert {
                        true => -1.0,
                        false => 1.0,
                    };
                    ig.mouse_x += xrel as f32 * ig.mouse_speed;
                    ig.mouse_y += yrel as f32 * ig.mouse_speed * f_invert;
                }
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

pub fn get_mouse() -> Result<(f32, f32), NUError> {
    let ig = InputGod::get()?;
    Ok((ig.mouse_x, ig.mouse_y))
}

pub fn get_keys_pressed() -> Result<[bool; 9], NUError> {
    Ok(InputGod::get()?.keys_pressed)
}

pub fn get_keys_down() -> Result<[bool; 9], NUError> {
    Ok(InputGod::get()?.keys_down)
}

pub fn get_keys_released() -> Result<[bool; 9], NUError> {
    Ok(InputGod::get()?.keys_released)
}
