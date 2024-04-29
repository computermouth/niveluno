use std::ffi::{CString, NulError};
use std::ptr::addr_of_mut;

use gl;
use gl::types::*;

use crate::math::Vec3;
use crate::nuerror::NUError;

// draw texture res, default window res
pub const INTERNAL_W: i32 = 320;
pub const INTERNAL_H: i32 = 180;
pub const D_WINDOW_W: u32 = 640;
pub const D_WINDOW_H: u32 = 360;

const V_SHADER_STR: &str = include_str!("game_vert.glsl");
const F_SHADER_STR: &str = include_str!("game_frag.glsl");

struct RenderGod {
    // OU-global collections
    pub draw_calls: Vec<i32>,
    pub textures: Vec<i32>,

    // shader stuff
    pub shader_program: GLuint,
    pub vertex_buffer: GLuint,

    // uniforms
    pub u_camera: GLint,
    pub u_lights: GLint,
    pub u_light_count: GLint,
    pub u_mouse: GLint,
    pub u_pos: GLint,
    pub u_rotation: GLint,
    pub u_frame_mix: GLint,
    pub u_unlit: GLint,

    // vertex attribute location for mixing
    pub va_p2: GLint,
    pub va_n2: GLint,

    pub default_fbo: GLint,
    pub offscreen_fbo: GLuint,
    pub offscreen_color_tex: GLuint,
    pub offscreen_depth_tex: GLuint,
}

impl RenderGod {
    pub fn get() -> Result<&'static mut RenderGod, NUError> {
        unsafe {
            RENDER_GOD
                .as_mut()
                .ok_or_else(|| NUError::MiscError("RENDER_GOD uninit".to_string()))
        }
    }
}

// this could probably be a refcell inside some NUGod struct
// that's just passed everywhere, maybe this is less annoying though
static mut RENDER_GOD: Option<RenderGod> = None;

pub fn compile_shader(shader_type: GLenum, shader_str: &str) -> Result<GLuint, NUError> {
    // https://dev.to/samkevich/learn-opengl-with-rust-shaders-28i3

    let mut shader: GLuint = 0;
    let _ = shader;
    let shader_cstr = CString::new(shader_str).map_err(|_| NUError::ShaderLoadError)?;

    unsafe {
        shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &shader_cstr.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success: GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != 1 {
            let mut error_log_size: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetShaderInfoLog(
                shader,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );
            let e = NUError::ShaderCompilationError(String::from_utf8(error_log)?);
            return Err(e);
        }
    }

    Ok(shader as GLuint)
}

pub fn create_program(vert: GLuint, frag: GLuint) -> Result<GLuint, NUError> {
    let mut program: GLuint = 0;
    let _ = program;

    unsafe {
        program = gl::CreateProgram();
        if program == 0 {
            return Err(NUError::ShaderProgramCreateError);
        }
        gl::AttachShader(program, vert);
        gl::AttachShader(program, frag);
        gl::LinkProgram(program);

        let mut success: GLint = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success != 1 {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let e = NUError::ShaderLinkError(String::from_utf8(error_log)?);
            eprintln!("{}", e);
            return Err(e);
        }
    }

    Ok(program)
}

fn vertex_attribute(
    shader_program: GLuint,
    attrib_name: &str,
    count: isize,
    vertex_size: isize,
    offset: isize,
) -> Result<GLint, NUError> {
    let mut location: GLint = 0;
    _ = location;

    unsafe {
        location = gl::GetAttribLocation(shader_program, attrib_name.as_ptr() as *const i8);
        gl::EnableVertexAttribArray(location as u32);
        gl::VertexAttribPointer(
            location as GLuint,
            count as GLint,
            gl::FLOAT,
            gl::FALSE,
            (vertex_size * 4) as GLsizei,
            (offset * 4) as *const GLvoid,
        );
    }

    match location {
        0 => {
            let e = NUError::VertexAttribError;
            eprintln!("{}", e);
            Err(e)
        }
        _ => Ok(location),
    }
}

pub fn init() -> Result<(), NUError> {
    let rg = RenderGod {
        // OU-global collections
        draw_calls: vec![],
        textures: vec![],

        // shader stuff
        shader_program: 0,
        vertex_buffer: 0,

        // uniforms
        u_camera: 0,
        u_lights: 0,
        u_light_count: 0,
        u_mouse: 0,
        u_pos: 0,
        u_rotation: 0,
        u_frame_mix: 0,
        u_unlit: 0,

        // vertex attribute location for mixing
        va_p2: 0,
        va_n2: 0,

        default_fbo: 0,
        offscreen_fbo: 0,
        offscreen_color_tex: 0,
        offscreen_depth_tex: 0,
    };

    unsafe {
        RENDER_GOD = Some(rg);
    }

    let rg = RenderGod::get()?;

    // compile and set shader
    unsafe {
        rg.shader_program = create_program(
            compile_shader(gl::VERTEX_SHADER, V_SHADER_STR)?,
            compile_shader(gl::FRAGMENT_SHADER, F_SHADER_STR)?,
        )?;
        gl::UseProgram(rg.shader_program);
    }

    // set up uniforms
    unsafe {
        // todo, map these characters to the reference of these static muts
        // also change these names in the shader
        rg.u_camera = gl::GetUniformLocation(rg.shader_program, "c".as_ptr() as *const i8);
        rg.u_lights = gl::GetUniformLocation(rg.shader_program, "l".as_ptr() as *const i8);
        rg.u_light_count =
            gl::GetUniformLocation(rg.shader_program, "light_count".as_ptr() as *const i8);
        rg.u_mouse = gl::GetUniformLocation(rg.shader_program, "m".as_ptr() as *const i8);
        // i think mp and mr are matrix_pos and matrix_rotation
        rg.u_pos = gl::GetUniformLocation(rg.shader_program, "mp".as_ptr() as *const i8);
        rg.u_rotation = gl::GetUniformLocation(rg.shader_program, "mr".as_ptr() as *const i8);
        rg.u_frame_mix = gl::GetUniformLocation(rg.shader_program, "f".as_ptr() as *const i8);
        rg.u_unlit = gl::GetUniformLocation(rg.shader_program, "unlit".as_ptr() as *const i8);
    }

    // vertex buffer
    unsafe {
        // fuck if I know what addr_of_mut does
        gl::GenBuffers(1, addr_of_mut!(rg.vertex_buffer));
        gl::BindBuffer(gl::ARRAY_BUFFER, rg.vertex_buffer);
    }

    // vertex attribute initialization
    unsafe {
        // I don't remember why these first 3 don't get assigned to something
        vertex_attribute(rg.shader_program, "p", 3, 8, 0)?;
        vertex_attribute(rg.shader_program, "t", 2, 8, 3)?;
        vertex_attribute(rg.shader_program, "n", 3, 8, 5)?;
        rg.va_p2 = vertex_attribute(rg.shader_program, "p2", 3, 8, 0)?;
        rg.va_n2 = vertex_attribute(rg.shader_program, "n2", 3, 8, 5)?;
    }

    // gl extras
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::CULL_FACE);
    }

    // create viewport and clear
    unsafe {
        gl::Viewport(0, 0, INTERNAL_W, INTERNAL_H);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // save default fbo
    unsafe {
        gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, addr_of_mut!(rg.default_fbo));
    }

    // initialize offscreen fbo
    unsafe {
        gl::GenFramebuffers(1, addr_of_mut!(rg.offscreen_fbo));
        gl::BindFramebuffer(gl::FRAMEBUFFER, rg.offscreen_fbo);
    }

    // initialize backing texture for offscreen fbo
    unsafe {
        gl::GenTextures(1, addr_of_mut!(rg.offscreen_color_tex));
        gl::BindTexture(gl::TEXTURE_2D, rg.offscreen_color_tex);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            INTERNAL_W,
            INTERNAL_H,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            0 as *const GLvoid,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            rg.offscreen_color_tex,
            0,
        );
    }

    // initialize depth texture for offscreen fbo
    unsafe {
        gl::GenTextures(1, addr_of_mut!(rg.offscreen_depth_tex));
        gl::BindTexture(gl::TEXTURE_2D, rg.offscreen_depth_tex);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT as GLint,
            INTERNAL_W,
            INTERNAL_H,
            0,
            gl::DEPTH_COMPONENT,
            gl::UNSIGNED_INT,
            0 as *const GLvoid,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::TEXTURE_2D,
            rg.offscreen_depth_tex,
            0,
        );
    }

    // restore the vertex buffer and attributes
    unsafe {
        // i don't remember why this happens either, maybe unnecessary?
        // is this because of the other shader and switching?
        gl::BindBuffer(gl::ARRAY_BUFFER, rg.vertex_buffer);
        vertex_attribute(rg.shader_program, "p", 3, 8, 0)?;
        vertex_attribute(rg.shader_program, "t", 2, 8, 3)?;
        vertex_attribute(rg.shader_program, "n", 3, 8, 5)?;
        rg.va_p2 = vertex_attribute(rg.shader_program, "p2", 3, 8, 0)?;
        rg.va_n2 = vertex_attribute(rg.shader_program, "n2", 3, 8, 5)?;
    }

    Ok(())
}

struct PngBin {}

pub fn create_texture(p: PngBin) -> Result<usize, NUError> {
    Ok(0)
}

pub fn prepare_frame(r: f32, g: f32, b: f32) -> Result<(), NUError> {
    Ok(())
}

pub fn end_frame() -> Result<(), NUError> {
    Ok(())
}

struct DrawCall {}

pub fn draw(d: DrawCall) -> Result<(), NUError> {
    Ok(())
}

pub fn submit_buffer() -> Result<(), NUError> {
    Ok(())
}

pub fn push_vert(pos: Vec3, normal: Vec3, u: f32, v: f32) -> Result<(), NUError> {
    Ok(())
}

pub fn push_quad(v0: Vec3, v1: Vec3, v2: Vec3, v3: Vec3, u: f32, v: f32) -> Result<(), NUError> {
    Ok(())
}

pub fn push_block(
    x: f32,
    y: f32,
    z: f32,
    sx: f32,
    sy: f32,
    sz: f32,
    texture: isize,
) -> Result<(), NUError> {
    Ok(())
}

pub fn push_light(pos: Vec3, intensity: f32, r: f32, g: f32, b: f32) -> Result<(), NUError> {
    Ok(())
}

pub fn quit() {
    unsafe {
        RENDER_GOD = None;
    }
}
