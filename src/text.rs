use std::ops::Deref;
use std::rc::Rc;

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

// #[derive(Clone)]
pub struct TextSurface<'a> {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    // todo, figure out what type this should be
    pub data: Rc<Surface<'a>>,
}

pub struct TimedSurface<'a> {
    ts: TextSurface<'a>,
    end_time: f32,
}

impl<'a> TimedSurface<'a> {
    pub fn new(ts: TextSurface, ms: u32) -> TimedSurface {
        TimedSurface {
            ts: ts,
            end_time: game::get_time() + ms as f32 / 1000.
        }
    }
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
    pub timed_surfaces: Vec<TimedSurface<'a>>,
    // gl things
    pub overlay_program: GLuint,
    pub overlay_position: GLuint,
    pub overlay_texcoord: GLuint,
    pub overlay_tex_u: GLuint,
    pub overlay_vbo: GLuint,
    pub overlay_texture: GLuint,
}

const V_SHADER_STR: &str = include_str!("game_vert.glsl");
const F_SHADER_STR: &str = include_str!("game_frag.glsl");

// this could probably be a refcell inside some NUGod struct
// that's just passed everywhere, maybe this is less annoying though
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

    let timed_surfaces: Vec<TimedSurface> = vec![];

    let overlay_program = render::create_program(
        render::compile_shader(gl::VERTEX_SHADER, V_SHADER_STR)?,
        render::compile_shader(gl::VERTEX_SHADER, V_SHADER_STR)?,
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

    let mut overlay_position: GLint = 0;
    let mut overlay_texcoord: GLint = 0;
    let mut overlay_tex: GLint = 0;
    let mut overlay_vbo: GLuint = 0;
    let mut overlay_texture: GLuint = 0;
    _ = (overlay_position, overlay_texcoord, overlay_tex, overlay_vbo, overlay_texture);

    unsafe {
        gl::UseProgram(overlay_program);
        overlay_position = gl::GetAttribLocation(overlay_program, "position".as_ptr() as *const i8);
        overlay_texcoord = gl::GetAttribLocation(overlay_program, "texcoord".as_ptr() as *const i8);
        overlay_tex = gl::GetAttribLocation(overlay_program, "tex".as_ptr() as *const i8);

        // Create a VBO and upload the vertex data
        gl::GenBuffers(1, &mut overlay_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, overlay_vbo);
        let sz = (OVLY_VERT.len() * std::mem::size_of::<f32>()) as isize;
        gl::BufferData(
            gl::ARRAY_BUFFER,
            sz,
            OVLY_VERT.as_ptr() as *const GLvoid,
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
    let ts: &mut Vec<TimedSurface>;
    unsafe {
        if let Some(tg) = &mut TEXT_GOD {
            ts = &mut tg.timed_surfaces;
        } else {
            return Err(NUError::MiscError("TEXT_GOD uninit".to_string()));
        }
    }

    /*
       // this seems fuckin dumb
       // should just be able to filter or something, right?
       let game_time = game::get_time();
       let mut remaining = Vec::new();
       for i in 0..ts.len() {
           if ts[i].end_time > game_time {
               text_push_surface(&ts[i].ts);
               remaining.push(ts[i]);
           }
       }
    */
    // todo, less dumb, but this reverse is wasteful probably
    ts.reverse();
    let game_time = game::get_time();
    let mut remaining = Vec::new();
    while let Some(t) = ts.pop() {
        if t.end_time > game_time {
            text_push_surface(&t.ts);
            remaining.push(t);
        }
    }

    unsafe {
        if let Some(tg) = &mut TEXT_GOD {
            tg.timed_surfaces = remaining;
        }
    }

    let mut overlay_tex_u = None;
    let mut overlay_texture = None;
    let mut overlay_surface = None;
    let mut overlay_program = None;
    let mut overlay_vbo = None;
    let mut overlay_position = None;
    let mut overlay_texcoord = None;
    unsafe {
        if let Some(tg) = &mut TEXT_GOD {
            overlay_tex_u = Some(tg.overlay_tex_u);
            overlay_texture = Some(tg.overlay_texture);
            overlay_surface = Some(&tg.overlay_surface);
            overlay_program = Some(tg.overlay_program);
            overlay_vbo = Some(tg.overlay_vbo);
            overlay_position = Some(tg.overlay_position);
            overlay_texcoord = Some(tg.overlay_texcoord);
        }
    };

    // set up overlay texture
    unsafe {
        let os = overlay_surface.unwrap();
        let pixels = os.with_lock(|p| p.to_vec());
        gl::BindTexture(gl::TEXTURE_2D, overlay_texture.unwrap());
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            os.width() as i32,
            os.height() as i32,
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
        gl::UseProgram(overlay_program.unwrap());
        gl::BindBuffer(gl::ARRAY_BUFFER, overlay_vbo.unwrap());
    }
    // vertices
    unsafe {
        let stride_sz = (4 * std::mem::size_of::<GLfloat>()) as i32;
        let pointr_sz = 2 * std::mem::size_of::<GLfloat>();
        gl::EnableVertexAttribArray(overlay_position.unwrap());
        gl::EnableVertexAttribArray(overlay_texcoord.unwrap());
        gl::VertexAttribPointer(
            overlay_position.unwrap(),
            2,
            gl::FLOAT,
            gl::FALSE,
            stride_sz,
            0 as *const GLvoid,
        );
        gl::VertexAttribPointer(
            overlay_texcoord.unwrap(),
            2,
            gl::FLOAT,
            gl::FALSE,
            stride_sz,
            pointr_sz as *const GLvoid,
        );
    }

    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, overlay_texture.unwrap());
        gl::Uniform1i(overlay_tex_u.unwrap() as i32, 0);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::Enable(gl::CULL_FACE);
    }

    Ok(())
}

pub fn text_create_surface<'a>(input: FontInput) -> Result<Box<TextSurface<'a>>, NUError> {
    let mut tg = None;
    _ = tg;
    unsafe {
        if let Some(t) = &TEXT_GOD {
            tg = Some(t);
        } else {
            return Err(NUError::MiscError("TEXT_GOD uninit".to_string()));
        }
    }

    let font = match input.size {
        FontSize::SM => &tg.unwrap().font_sm,
        FontSize::MD => &tg.unwrap().font_md,
        FontSize::LG => &tg.unwrap().font_lg,
    };

    let fg = sdl2::pixels::Color::RGBA(input.color.r, input.color.g, input.color.b, input.color.a);
    let tmp_fg = font
        .render(&input.text)
        .solid(fg)
        .map_err(|e| NUError::SDLError(e.to_string()))?;

    // todo, background??

    Ok(Box::new(TextSurface {
        x: 0,
        y: 0,
        w: tmp_fg.width(),
        h: tmp_fg.height(),
        data: Rc::new(tmp_fg),
    }))
}

pub fn text_prepare_frame() -> Result<(), NUError> {
    let mut os = None;
    unsafe {
        if let Some(tg) = &mut TEXT_GOD {
            os = Some(&mut tg.overlay_surface);
        } else {
            return Err(NUError::MiscError("TEXT_GOD uninit".to_string()));
        }
    }
    os.unwrap()
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
    Ok(())
}

static mut fart: Option<Vec<Box<TimedSurface>>> = None;

// i have no idea why this wor
pub fn text_push_timed_surface(time_surf: Box<TimedSurface>) -> Result<(), NUError> {
    let mut ts = None;
    unsafe {
        if let Some(tg) = &mut TEXT_GOD {
            ts = Some(&mut tg.timed_surfaces);
        } else {
            return Err(NUError::MiscError("TEXT_GOD uninit".to_string()));
        }
    }

    unsafe {
        if let Some(f) = &mut fart {
            f.push(time_surf);
        }
    }

    // ts.unwrap().push(time_surf);
    drop(time_surf);
    Ok(())
}

pub fn text_push_surface(ts: &TextSurface) {}
pub fn text_quit() {}
