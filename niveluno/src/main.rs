use gl;
use raymath::{BoundingBox, Vector3};
use sdl2::mixer::{self, InitFlag};
use sdl2::video::{GLProfile, SwapInterval};

mod asset;
mod audio;
mod input;
mod map;
mod math;
mod nuerror;
mod render;
mod text;
mod time;

// pak module
mod g_game;
// pak/entity??
mod e_barrier;
mod e_entity;
mod e_gcyl;
mod e_light;
mod e_menu;
mod e_pig;
mod e_player;
// pak/decor??
mod d_decor;
mod d_floor;
mod d_platform;

use nuerror::NUError;

fn init_sdl() -> Result<(sdl2::Sdl, sdl2::video::Window, sdl2::video::GLContext), nuerror::NUError>
{
    let sdl_context = sdl2::init().map_err(|e| nuerror::NUError::SDLError(e))?;

    // video
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

    let ctx = window
        .gl_create_context()
        .map_err(|e| nuerror::NUError::SDLError(e))?;
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::GLES);
    debug_assert_eq!(gl_attr.context_version(), (3, 0));

    // mixer
    sdl_context
        .audio()
        .map_err(|e| nuerror::NUError::SDLError(e))?;
    mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, 2, 2048)
        .map_err(|e| nuerror::NUError::SDLError(e))?;
    sdl2::mixer::init(InitFlag::OGG).map_err(|e| nuerror::NUError::SDLError(e))?;
    // todo, tune?
    mixer::allocate_channels(64);

    // timer
    sdl_context
        .timer()
        .map_err(|e| nuerror::NUError::SDLError(e))?;

    // todo, text init here?
    video_subsystem
        // .gl_set_swap_interval(SwapInterval::Immediate)
        .gl_set_swap_interval(SwapInterval::VSync)
        .map_err(|e| NUError::SDLError(e))?;

    Ok((sdl_context, window, ctx))
}

fn init_nu() -> Result<(), nuerror::NUError> {
    time::init()?;
    text::init()?;
    render::init()?;
    audio::init()?;
    input::init()?;
    asset::init()?;

    // always last
    g_game::init()?;

    Ok(())
}

fn main() -> Result<(), String> {
    let (sdl_context, mut window, _gl_context) = init_sdl()?;

    init_nu()?;

    let mut event_pump = sdl_context
        .event_pump()
        .map_err(|e| nuerror::NUError::SDLError(e))?;

    let mut time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| NUError::SystemTimeError(e.to_string()))?
        .as_secs();
    let mut oldtime = time;
    let mut newtime;
    let mut frames = 0;

    'running: loop {
        frames += 1;
        time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| NUError::SystemTimeError(e.to_string()))?
            .as_secs();
        newtime = time;
        if newtime - oldtime >= 2 {
            eprintln!("fps: {}", frames as f32 / 2.0);
            oldtime = newtime;
            frames = 0;
        }

        input::consume(&mut window, &sdl_context.mouse(), &mut event_pump)?;
        if input::get_quit()? {
            break 'running;
        }

        render::prepare_frame()?;
        time::update_time()?;

        g_game::run()?;

        render::end_frame()?;
        window.gl_swap_window();

        loop {
            let s = sdl2::get_error();
            if s == "" {
                break;
            }
            eprintln!("sdlerror: {}", s);
            panic!();
        }

        loop {
            let s = unsafe { gl::GetError() };
            if s == gl::NO_ERROR {
                break;
            }
            eprintln!("glerror: {:x}", s);
            panic!();
        }
    }

    // probably unnecessary
    text::quit();
    render::quit();

    Ok(())
}
