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

pub enum FontSize {
    SM,
    MD,
    LG,
}

pub struct FontInput {
    pub text: String,
    pub color: FontColor,
    pub size: FontSize,
}

#[derive(Clone)]
pub struct TextSurface {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    // todo, figure out what type this should be
    pub data: Vec<u8>,
}

pub struct TimedSurface {
    pub ts: TextSurface,
    pub ms: u32,
}

#[derive(Clone)]
struct InternalTimedSurface {
    pub ts: TextSurface,
    pub end_time: f32,
}

struct TextGod<'a> {
    pub context: Sdl2TtfContext,
    // RefCell<Surface<'a>> ??
    pub overlay_surface: Surface<'a>,
    pub font_sm_rw: RWops<'a>,
    pub font_md_rw: RWops<'a>,
    pub font_lg_rw: RWops<'a>,
    pub font_sm: Font<'a, 'a>,
    pub font_md: Font<'a, 'a>,
    pub font_lg: Font<'a, 'a>,
    pub timed_surfaces: Vec<InternalTimedSurface>,
    // gl things
    pub overlay_program: GLuint,
    pub overlay_position: GLuint,
    pub overlay_texcoord: GLuint,
    pub overlay_tex: GLuint,
    pub overlay_vbo: GLuint,
    pub overlay_texture: GLuint,
}

const V_SHADER_STR: &str = include_str!("game_vert.glsl");
const F_SHADER_STR: &str = include_str!("game_frag.glsl");

static mut TEXT_GOD: Option<TextGod<'static>> = None;

pub fn text_init() -> Result<(), NUError> {
    // context
    let ctx = sdl2::ttf::init().map_err(|e| NUError::SDLError(e.to_string()))?;
    // surface
    let mut overlay_surface = Surface::new(
        render::INTERNAL_W as u32,
        render::INTERNAL_H as u32,
        pixels::PixelFormatEnum::ABGR8888,
    )
    .map_err(|e| NUError::SDLError(e))?;
    overlay_surface
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
    // fonts (todo -- loading each font from a different rwops seemed to be necessary in C, not sure in rust)
    let font_b = include_bytes!("/usr/share/fonts/truetype/liberation/LiberationMono-Bold.ttf");
    let font_sm_rw = RWops::from_bytes(font_b).map_err(|e| NUError::SDLError(e))?;
    let font_sm = ctx
        .load_font_from_rwops(font_sm_rw, 12)
        .map_err(|e| NUError::SDLError(e))?;
    let font_md_rw = RWops::from_bytes(font_b).map_err(|e| NUError::SDLError(e))?;
    let font_md = ctx
        .load_font_from_rwops(font_md_rw, 18)
        .map_err(|e| NUError::SDLError(e))?;
    let font_lg_rw = RWops::from_bytes(font_b).map_err(|e| NUError::SDLError(e))?;
    let font_lg = ctx
        .load_font_from_rwops(font_lg_rw, 32)
        .map_err(|e| NUError::SDLError(e))?;

    let timed_surfaces: Vec<InternalTimedSurface> = vec![];

    let overlay_program = render::create_program(
        render::compile_shader(gl::VERTEX_SHADER, V_SHADER_STR)?,
        render::compile_shader(gl::VERTEX_SHADER, V_SHADER_STR)?,
    )?;

    #[rustfmt::skip]
    mod u {
        pub const _OVLY_VERT: [gl::types::GLfloat;24] = [
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
    }

    let mut overlay_position: GLint = 0;
    let mut overlay_texcoord: GLint = 0;
    let mut overlay_tex: GLint = 0;
    let mut overlay_vbo: GLuint = 0;
    let mut overlay_texture: GLuint = 0;

    unsafe {
        gl::UseProgram(overlay_program);
        overlay_position = gl::GetAttribLocation(overlay_program, "position".as_ptr() as *const i8);
        overlay_texcoord = gl::GetAttribLocation(overlay_program, "texcoord".as_ptr() as *const i8);
        overlay_tex = gl::GetAttribLocation(overlay_program, "tex".as_ptr() as *const i8);

        // Create a VBO and upload the vertex data
        gl::GenBuffers(1, &mut overlay_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, overlay_vbo);
        let sz = (u::_OVLY_VERT.len() * std::mem::size_of::<f32>()) as isize;
        gl::BufferData(
            gl::ARRAY_BUFFER,
            sz,
            u::_OVLY_VERT.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // Create GL texture from sdl surface
        gl::GenTextures(1, &mut overlay_texture);
        gl::BindTexture(gl::TEXTURE_2D, overlay_texture);
        let pixels = overlay_surface.with_lock(|p| p.to_vec());
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

    Ok(())
}

pub fn text_end_frame() -> Result<(), NUError> {
    let ts: &mut Vec<InternalTimedSurface>;
    unsafe {
        if let Some(tg) = &mut TEXT_GOD {
            ts = &mut tg.timed_surfaces;
        } else {
            return Err(NUError::MiscError("TEXT_GOD uninit".to_string()));
        }
    }

    let game_time = game::get_time();
    let mut remaining = Vec::new();
    for i in 0..ts.len() {
        if ts[i].end_time > game_time {
            text_push_surface(&ts[i].ts);
            remaining.push(ts[i].clone());
        }
    }

    unsafe {
        if let Some(tg) = &mut TEXT_GOD {
            tg.timed_surfaces = remaining;
        }
    }

    // let remaining: Vec<InternalTimedSurface> = ts.iter().filter(|v|{
    //     v.end_time > game::get_time()
    // }).map(|v| {text_push_surface(v.ts); v}).collect();

    Ok(())
}

pub fn text_create_surface(input: FontInput) -> Box<TextSurface> {
    Box::new(TextSurface {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
        data: Vec::new(),
    })
}
pub fn text_push_timed_surface(time_surf: TimedSurface) {}
pub fn text_push_surface(ts: &TextSurface) {}
pub fn text_prepare_frame() {}
pub fn text_quit() {}
