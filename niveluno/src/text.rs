use std::collections::HashMap;
use std::ffi::CString;

use sdl2::pixels;
use sdl2::rect;
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};

use gl;
use gl::types::*;

use crate::game;
use crate::nuerror::NUError;
use crate::render;

pub struct FontColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// todo, background color for all?
pub enum Mode {
    Solid {
        color: FontColor,
    },
    Shaded {
        color: FontColor,
        background: FontColor,
    },
    Blended {
        color: FontColor,
    },
}

pub struct FontInput {
    pub text: String,
    pub mode: Mode,
    pub font: SizedFontHandle,
}

// #[derive(Clone)]
pub struct TextSurface {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    // todo, figure out what type this should be
    pub data: Surface<'static>,
}

pub struct TimedSurface {
    ts: TextSurface,
    end_time: f32,
}

impl TimedSurface {
    pub fn new(ts: TextSurface, ms: u32) -> TimedSurface {
        TimedSurface {
            ts: ts,
            end_time: game::get_run_time().unwrap() as f32 + ms as f32 / 1000.,
        }
    }
}

const V_SHADER_STR: &str = include_str!("ovly_vert.glsl");
const F_SHADER_STR: &str = include_str!("ovly_frag.glsl");

struct TextGod<'a> {
    pub context: Sdl2TtfContext,
    // RefCell<Surface<'a>> ??
    pub overlay_surface: Option<Surface<'a>>,
    // pub font_sm: Option<Font<'a, 'a>>,
    // pub font_md: Option<Font<'a, 'a>>,
    // pub font_lg: Option<Font<'a, 'a>>,
    pub timed_surfaces: Vec<TimedSurface>,
    // gl things
    pub overlay_program: GLuint,
    pub overlay_position: GLint,
    pub overlay_texcoord: GLint,
    pub overlay_tex_u: GLint,
    pub overlay_vbo: GLuint,
    pub overlay_texture: GLuint,
    pub font_data_dict: HashMap<usize, Vec<u8>>,
    pub font_data_ids: usize,
    pub font_font_dict: HashMap<usize, Font<'a, 'a>>,
    pub font_font_ids: usize,
}

impl<'a> TextGod<'a> {
    pub fn get() -> Result<&'static mut TextGod<'static>, NUError> {
        unsafe {
            TEXT_GOD
                .as_mut()
                .ok_or_else(|| NUError::MiscError("TEXT_GOD uninit".to_string()))
        }
    }
}

// this could probably be a refcell inside some NUGod struct
// that's just passed everywhere, maybe this is less annoying though
//
// Actually do something like this:
// use std::sync::{Mutex, OnceLock};
// fn text_get_god<'a>() -> &'static Mutex<TextGod<'a>> {
//     static TEXT_GOD: OnceLock<Mutex<TextGod>> = OnceLock::new();
//     TEXT_GOD.get_or_init(|| init())
// }

static mut TEXT_GOD: Option<TextGod<'static>> = None;

pub fn init() -> Result<(), NUError> {
    if TextGod::get().is_ok() {
        return Err(NUError::MiscError("TEXT_GOD already init".to_string()));
    }

    unsafe {
        TEXT_GOD = Some(TextGod {
            context: sdl2::ttf::init().map_err(|e| NUError::SDLError(e.to_string()))?,
            overlay_surface: None,
            // font_sm: None,
            // font_md: None,
            // font_lg: None,
            timed_surfaces: vec![],
            overlay_program: 0,
            overlay_position: 0,
            overlay_texcoord: 0,
            overlay_tex_u: 0,
            overlay_vbo: 0,
            overlay_texture: 0,
            font_data_dict: HashMap::new(),
            font_data_ids: 0,
            font_font_dict: HashMap::new(),
            font_font_ids: 0,
        });
    }

    let tg = TextGod::get()?;

    // surface
    tg.overlay_surface = Some(
        Surface::new(
            render::INTERNAL_W as u32,
            render::INTERNAL_H as u32,
            pixels::PixelFormatEnum::ABGR8888,
        )
        .map_err(|e| NUError::SDLError(e))?,
    );
    tg.overlay_surface
        .as_mut()
        .unwrap()
        .fill_rect(
            rect::Rect::new(0, 0, render::INTERNAL_W as u32, render::INTERNAL_H as u32),
            pixels::Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        )
        .map_err(|e| NUError::SDLError(e))?;

    // // fonts (todo -- loading each font from a different rwops seemed to be necessary in C, not sure in rust)
    // let font_b = include_bytes!("/usr/share/fonts/truetype/liberation/LiberationMono-Bold.ttf");
    // let font_rwo = RWops::from_bytes(font_b).map_err(|e| NUError::SDLError(e))?;
    // tg.font_sm = Some(
    //     tg.context
    //         .load_font_from_rwops(font_rwo, 12)
    //         .map_err(|e| NUError::SDLError(e))?,
    // );
    // let font_rwo = RWops::from_bytes(font_b).map_err(|e| NUError::SDLError(e))?;
    // tg.font_md = Some(
    //     tg.context
    //         .load_font_from_rwops(font_rwo, 18)
    //         .map_err(|e| NUError::SDLError(e))?,
    // );
    // let font_rwo = RWops::from_bytes(font_b).map_err(|e| NUError::SDLError(e))?;
    // tg.font_lg = Some(
    //     tg.context
    //         .load_font_from_rwops(font_rwo, 32)
    //         .map_err(|e| NUError::SDLError(e))?,
    // );

    tg.overlay_program = render::create_program(
        render::compile_shader(gl::VERTEX_SHADER, V_SHADER_STR)?,
        render::compile_shader(gl::FRAGMENT_SHADER, F_SHADER_STR)?,
    )?;

    #[rustfmt::skip]
    const OVLY_VERT: [gl::types::GLfloat;24] = [
        // bottom right
        // Position     Texture coordinates
         1.0, -1.0,     1.0, 1.0,
        -1.0, -1.0,     0.0, 1.0,
         1.0,  1.0,     1.0, 0.0,
        // top left
        // Position     Texture coordinates
        -1.0,  1.0,     0.0, 0.0,
         1.0,  1.0,     1.0, 0.0,
        -1.0, -1.0,     0.0, 1.0,
    ];

    unsafe {
        gl::UseProgram(tg.overlay_program);
        tg.overlay_position = gl::GetAttribLocation(
            tg.overlay_program,
            CString::new("position")?.as_ptr() as *const i8,
        );
        tg.overlay_texcoord = gl::GetAttribLocation(
            tg.overlay_program,
            CString::new("texcoord")?.as_ptr() as *const i8,
        );
        tg.overlay_tex_u = gl::GetAttribLocation(
            tg.overlay_program,
            CString::new("tex")?.as_ptr() as *const i8,
        );

        // Create a VBO and upload the vertex data
        gl::GenBuffers(1, &mut tg.overlay_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, tg.overlay_vbo);
        let sz = (OVLY_VERT.len() * std::mem::size_of::<f32>()) as isize;
        gl::BufferData(
            gl::ARRAY_BUFFER,
            sz,
            OVLY_VERT.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Create GL texture from sdl surface
        gl::GenTextures(1, &mut tg.overlay_texture);
        gl::BindTexture(gl::TEXTURE_2D, tg.overlay_texture);
        let pixels = tg
            .overlay_surface
            .as_mut()
            .unwrap()
            .with_lock(|p| p.to_vec());
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            tg.overlay_surface.as_mut().unwrap().width() as i32,
            tg.overlay_surface.as_mut().unwrap().height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            pixels.as_ptr() as *const GLvoid,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }

    Ok(())
}

pub struct FontHandle {
    h: usize,
}

pub fn push_font(font_bytes: Vec<u8>) -> Result<FontHandle, NUError> {
    let tg = TextGod::get()?;

    let h = tg.font_data_ids;

    tg.font_data_dict.insert(h, font_bytes);
    tg.font_data_ids += 1;

    Ok(FontHandle { h })
}

#[derive(Copy, Clone)]
pub struct SizedFontHandle {
    h: usize,
}

pub fn create_sized_font(fh: FontHandle, size: u16) -> Result<SizedFontHandle, NUError> {
    let tg = TextGod::get()?;

    let h = tg.font_font_ids;

    let font_data = tg
        .font_data_dict
        .get(&fh.h)
        .ok_or_else(|| NUError::MiscError(format!("Font lookup failed on id {}", fh.h)))?;

    let font_rwo = RWops::from_bytes(font_data).map_err(|e| NUError::SDLError(e))?;

    let f = tg
        .context
        .load_font_from_rwops(font_rwo, size)
        .map_err(|e| NUError::SDLError(e))?;

    tg.font_font_dict.insert(h, f);
    tg.font_font_ids += 1;

    Ok(SizedFontHandle { h })
}

pub fn end_frame() -> Result<(), NUError> {
    let ts: &mut Vec<TimedSurface> = TextGod::get()?.timed_surfaces.as_mut();

    // todo, less dumb, but this reverse is wasteful probably
    // maybe VecDeque, pop-front
    ts.reverse();
    let game_time = game::get_run_time()? as f32;
    let mut remaining = Vec::new();
    while let Some(t) = ts.pop() {
        if t.end_time > game_time {
            push_surface(&t.ts)?;
            remaining.push(t);
        }
    }

    *ts = remaining;

    let overlay_tex_u = TextGod::get()?.overlay_tex_u;
    let overlay_texture = TextGod::get()?.overlay_texture;
    let overlay_surface = TextGod::get()?.overlay_surface.as_mut().unwrap();
    let overlay_program = TextGod::get()?.overlay_program;
    let overlay_vbo = TextGod::get()?.overlay_vbo;
    let overlay_position = TextGod::get()?.overlay_position;
    let overlay_texcoord = TextGod::get()?.overlay_texcoord;

    // set up overlay texture
    unsafe {
        let pixels = overlay_surface.with_lock(|p| p.to_vec());
        gl::BindTexture(gl::TEXTURE_2D, overlay_texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            overlay_surface.width() as i32,
            overlay_surface.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            pixels.as_ptr() as *const GLvoid,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }
    // program and buffer
    unsafe {
        gl::Disable(gl::CULL_FACE);
        gl::UseProgram(overlay_program);
        gl::BindBuffer(gl::ARRAY_BUFFER, overlay_vbo);
    }
    // vertices
    unsafe {
        let stride_sz = (4 * std::mem::size_of::<GLfloat>()) as i32;
        let pointr_sz = 2 * std::mem::size_of::<GLfloat>();
        gl::EnableVertexAttribArray(overlay_position as u32);

        loop {
            let s = gl::GetError();
            if s == gl::NO_ERROR {
                break;
            }
            eprintln!("glerror: {} {:x}", overlay_position, s);
            panic!();
        }
        gl::EnableVertexAttribArray(overlay_texcoord as u32);
        gl::VertexAttribPointer(
            overlay_position as u32,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride_sz,
            0 as *const GLvoid,
        );
        gl::VertexAttribPointer(
            overlay_texcoord as u32,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride_sz,
            pointr_sz as *const GLvoid,
        );
    }

    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, overlay_texture);
        gl::Uniform1i(overlay_tex_u as i32, 0);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::Enable(gl::CULL_FACE);
    }

    Ok(())
}

pub fn create_surface<'a>(input: FontInput) -> Result<Box<TextSurface>, NUError> {
    let tg = TextGod::get()?;

    let font = tg
        .font_font_dict
        .get(&input.font.h)
        .ok_or_else(|| NUError::MiscError(format!("Font not found with id {}", input.font.h)))?;

    let tmp_fg = match input.mode {
        Mode::Solid { color } => {
            let fg = sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a);
            font.render(&input.text)
                .solid(fg)
                .map_err(|e| NUError::SDLError(e.to_string()))?
        }
        Mode::Shaded { color, background } => {
            let fg = sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a);
            let bg =
                sdl2::pixels::Color::RGBA(background.r, background.g, background.b, background.a);
            font.render(&input.text)
                .shaded(fg, bg)
                .map_err(|e| NUError::SDLError(e.to_string()))?
        }
        Mode::Blended { color } => {
            let fg = sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a);
            font.render(&input.text)
                .blended(fg)
                .map_err(|e| NUError::SDLError(e.to_string()))?
        }
    };

    Ok(Box::new(TextSurface {
        x: 0,
        y: 0,
        w: tmp_fg.width(),
        h: tmp_fg.height(),
        data: tmp_fg,
    }))
}

pub fn prepare_frame() -> Result<(), NUError> {
    let os = &mut TextGod::get()?.overlay_surface.as_mut().unwrap();
    os.fill_rect(
        rect::Rect::new(0, 0, render::INTERNAL_W as u32, render::INTERNAL_H as u32),
        pixels::Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    )
    .map_err(|e| NUError::SDLError(e))?;
    Ok(())
}

pub fn push_timed_surface(time_surf: TimedSurface) -> Result<(), NUError> {
    let ts = &mut TextGod::get()?.timed_surfaces;

    ts.push(time_surf);
    Ok(())
}

pub fn push_surface(ts: &TextSurface) -> Result<(), NUError> {
    let os = &mut TextGod::get()?.overlay_surface.as_mut().unwrap();

    let dst_rect = sdl2::rect::Rect::new(ts.x as i32, ts.y as i32, ts.w, ts.h);
    ts.data
        .blit(None, os, dst_rect)
        .map_err(|e| NUError::SDLError(e))?;

    Ok(())
}

pub fn quit() {
    unsafe {
        TEXT_GOD = None;
    }
}
