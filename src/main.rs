use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use gl;

mod nuerror;
mod render;

fn init_systems() -> Result<(), nuerror::NUError> {
    render::init()?;

    Ok(())
}

fn main() -> Result<(), u8> {
    let sdl_context = sdl2::init().map_err(|e| nuerror::NUError::SDLError(e))?;
    let video_subsystem = sdl_context
        .video()
        .map_err(|e| nuerror::NUError::SDLError(e))?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video_subsystem
        .window("niveluno", render::D_WINDOW_W, render::D_WINDOW_H)
        .opengl()
        .resizable()
        .build()
        .map_err(|_| nuerror::NUError::WindowBuildError)?;

    let _ctx = window.gl_create_context().map_err(|e| nuerror::NUError::SDLError(e))?;
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::GLES);
    debug_assert_eq!(gl_attr.context_version(), (3, 0));

    init_systems()?;

    let mut event_pump = sdl_context
        .event_pump()
        .map_err(|e| nuerror::NUError::SDLError(e))?;

    'running: loop {
        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(1. / 60.));
    }

    Ok(())
}
