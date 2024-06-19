use gl;
use math::Vec3;
use mparse;
use render::draw;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{self, InitFlag};
use sdl2::video::{GLProfile, SwapInterval};

mod asset;
mod audio;
mod game;
mod input;
mod math;
mod nuerror;
mod render;
mod text;

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
        .gl_set_swap_interval(SwapInterval::Immediate)
        .map_err(|e| NUError::SDLError(e))?;

    Ok((sdl_context, window, ctx))
}

fn init_nu() -> Result<(), nuerror::NUError> {
    // text has to come before render init
    // not sure why, todo
    text::init()?;
    render::init()?;
    audio::init()?;
    input::init()?;
    asset::init()?;

    // todo, do this in somewhere like
    // render::end_frame, if num_verts has changed
    // since last buffer submission
    render::submit_buffer()?;

    Ok(())
}

fn main() -> Result<(), String> {
    let (sdl_context, mut window, _gl_context) = init_sdl()?;

    init_nu()?;

    let mut event_pump = sdl_context
        .event_pump()
        .map_err(|e| nuerror::NUError::SDLError(e))?;

    let title = text::create_surface(text::FontInput {
        text: "TITLE".to_string(),
        color: text::FontColor {
            r: 255,
            g: 167,
            b: 167,
            a: 255,
        },
        size: text::FontSize::LG,
    })?;

    let tex = render::placeholder_tex_id()?;
    let b = render::push_block(32., 32., 160., 32., 32., 32., tex)?;

    // todo, run this if r_buffer usage has changed
    render::submit_buffer()?;

    let start_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| NUError::SystemTimeError(e.to_string()))?
        .as_millis();

    let mut time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| NUError::SystemTimeError(e.to_string()))?
        .as_secs();
    let mut oldtime = time;
    let mut newtime;
    let mut frames = 0;

    let nmap = asset::get_file("nmap.mp")?
        .ok_or_else(|| NUError::MiscError("nmap not found".to_string()))?;

    let payload = mparse::unmarshal(&nmap).unwrap();
    _ = payload;

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

        for i in 0..10 {
            let dc = render::DrawCall {
                pos: Vec3 {
                    x: i as f32 * -18.,
                    y: i as f32 * -18.,
                    z: i as f32 * 36.,
                },
                yaw: 0.0,
                pitch: 0.0,
                texture: tex as u32,
                f1: b as i32,
                f2: b as i32,
                mix: 0.0,
                num_verts: 36,
                unlit: false,
            };
            draw(dc)?;
        }

        let ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| NUError::SystemTimeError(e.to_string()))?
            .as_millis();
        render::push_light(
            Vec3 {
                x: 25.,
                y: 25.,
                z: 25. + 100. * (0.001 * (ms - start_ms) as f32).sin(),
            },
            15.,
            100.,
            150.,
            50.,
        )?;

        text::push_surface(&title)?;

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
