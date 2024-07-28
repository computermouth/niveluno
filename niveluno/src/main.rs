use gl;
use math::Vec3;
use mparse;
use render::draw;
use sdl2::mixer::{self, InitFlag};
use sdl2::video::{GLProfile, SwapInterval};

mod asset;
mod audio;
mod game;
mod input;
mod level;
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
    text::init()?;
    render::init()?;
    audio::init()?;
    input::init()?;
    asset::init()?;
    game::init()?;

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

    let nmap = asset::get_file("nmap.mp")?
        .ok_or_else(|| NUError::MiscError("nmap not found".to_string()))?;
    let payload = mparse::unmarshal(&nmap).unwrap();
    eprintln!("{:?}", payload.drn_data);
    eprintln!("{:?}", payload.ern_data);
    eprintln!("{:?}", payload.kvs_data);
    let level = level::load_level(payload)?;

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

    enum State {
        Menu,
        Level,
    }

    let mut state = State::Menu;

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

        match state {
            State::Menu => {
                let ms = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| NUError::SystemTimeError(e.to_string()))?
                    .as_millis();

                let dc = render::DrawCall {
                    pos: Vec3 {
                        x: 0.,
                        y: 0.,
                        z: -45.,
                    },
                    yaw: 1.,
                    pitch: 0.,
                    texture: level.ref_entities[0].texture_handle as u32,
                    f1: level.ref_entities[0].frame_handles[0] as i32,
                    f2: level.ref_entities[0].frame_handles[2] as i32,
                    mix: ((1. * (0.001 * (ms - start_ms) as f32).sin()) + 1.0) / 2.,
                    num_verts: 372,
                    unlit: true,
                };
                draw(dc)?;

                for i in 0..10 {
                    let dc = render::DrawCall {
                        pos: Vec3 {
                            x: i as f32 * -18.,
                            y: i as f32 * -18.,
                            z: i as f32 * 36.,
                        },
                        yaw: 1. * (0.003 * (ms - start_ms) as f32).sin(),
                        pitch: 1. * (0.005 * (ms - start_ms) as f32).sin(),
                        texture: tex as u32,
                        f1: b as i32,
                        f2: b as i32,
                        mix: 0.0,
                        num_verts: 36,
                        unlit: false,
                    };
                    draw(dc)?;
                }

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

                if input::get_keys()?[input::Key::Action as usize] == true {
                    state = State::Level;
                    // stupid fucking clone
                    game::set_and_init_level(level.clone())?;
                }
            }
            State::Level => {
                // todo BEGIN REMOVE
                let ms = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| NUError::SystemTimeError(e.to_string()))?
                    .as_millis();

                for i in 0..10 {
                    let dc = render::DrawCall {
                        pos: Vec3 {
                            x: i as f32 * -18.,
                            y: i as f32 * -18.,
                            z: i as f32 * 36.,
                        },
                        yaw: 1. * (0.003 * (ms - start_ms) as f32).sin(),
                        pitch: 1. * (0.005 * (ms - start_ms) as f32).sin(),
                        texture: tex as u32,
                        f1: b as i32,
                        f2: b as i32,
                        mix: 0.0,
                        num_verts: 36,
                        unlit: false,
                    };
                    draw(dc)?;
                }

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
                // todo END REMOVE

                game::run()?;
            }
        }

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
