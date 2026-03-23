use core::f32;
use std::ffi::CString;
use std::ptr::addr_of_mut;

use minipng;

use gl;
use gl::types::*;
use raymath::{Matrix, vector3_distance, vector3_length};

use crate::math::{self, Vector3};
use crate::nuerror::NUError;
use crate::text;

// draw texture res, default window res
pub const INTERNAL_W: i32 = 640;
pub const INTERNAL_H: i32 = 360;
pub const D_WINDOW_W: u32 = 640;
pub const D_WINDOW_H: u32 = 360;

const V_SHADER_STR: &str = include_str!("game_vert.glsl");
const F_SHADER_STR: &str = include_str!("game_frag.glsl");
const DEBUG_V_SHADER_STR: &str = include_str!("debug_vert.glsl");
const DEBUG_F_SHADER_STR: &str = include_str!("debug_frag.glsl");

// magic numbers
// MAX_LIGHT_V3S game_frag.glsl
const MAX_VERTS: usize = 1024 * 128 * 64;
const MAX_LIGHT_V3S: usize = 32 * 2;

pub const PLACEHOLDER_PNG: &[u8; 69] = include_bytes!("placeholder.png");

// We collect all draw calls in an array and draw them all at once at the end
// the frame. This way the lights buffer will be completely filled and we
// only need to set it once for all geometry
#[derive(Clone, Copy)]
pub struct DrawCall {
    pub matrix: Matrix,
    pub texture: GLuint,
    pub f1: GLint, // todo, first frame of interpolation
    pub f2: GLint, // second frame of interpolation
    pub mix: f32,
    pub glow: Option<Vector3>,
    pub num_verts: usize,
}

struct DebugDrawCmd {
    primitive: GLenum,
    start: GLint,
    count: GLsizei,
    color: [f32; 3],
}

struct MetaTex {
    texture: GLuint,
    width: u32,
    height: u32,
}

struct RenderGod {
    // global collections
    pub draw_calls: Vec<DrawCall>,
    pub textures: Vec<MetaTex>,
    // 8 properties per vert [x,y,z, u,v, nx,ny,nz]
    // rename, vert_array_buffer_data
    pub r_buffer: Box<[f32; MAX_VERTS * 8]>,
    pub r_num_verts: usize,
    pub verts_changed: bool,
    // 2 vec3 per light [(x,y,z), [r,g,b], ...]
    // rename, light_array_buffer_data
    pub r_light_buffer: Box<[f32; MAX_LIGHT_V3S * 3]>,
    pub r_num_lights: usize,

    // shader stuff
    pub shader_program: GLuint,
    pub vertex_buffer: GLuint,
    pub vao: GLuint,

    // camera bits
    pub camera_position: Vector3,
    pub camera_pitch: GLfloat,
    pub camera_yaw: GLfloat,

    // uniforms
    pub u_camera_pos: GLint,
    pub u_lights: GLint,
    pub u_light_count: GLint,
    pub u_mouse: GLint,
    pub u_model_mat_v1: GLint,
    pub u_model_mat_v2: GLint,
    pub u_model_mat_v3: GLint,
    pub u_model_mat_v4: GLint,
    pub u_blend: GLint,
    pub u_glow: GLint,

    // vertex attribute location for mixing
    pub va_p2: GLint,
    pub va_n2: GLint,

    pub default_fbo: GLint,
    pub offscreen_fbo: GLuint,
    pub offscreen_color_tex: GLuint,
    pub offscreen_depth_tex: GLuint,

    pub pad_x: i32,
    pub pad_y: i32,
    pub current_window_width: i32,
    pub current_window_height: i32,

    pub placeholder_tex_id: usize,

    // debug drawing
    pub debug_shader_program: GLuint,
    pub debug_vbo: GLuint,
    pub debug_vao: GLuint,
    pub debug_u_camera_pos: GLint,
    pub debug_u_mouse: GLint,
    pub debug_u_color: GLint,
    pub debug_verts: Vec<f32>,
    pub debug_cmds: Vec<DebugDrawCmd>,
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

pub fn change_window_size(w: i32, h: i32) -> Result<(), NUError> {
    let rg = RenderGod::get()?;
    rg.current_window_width = w;
    rg.current_window_height = h;

    let fw = w as f32;
    let fh = h as f32;
    let dw = INTERNAL_W as f32;
    let dh = INTERNAL_H as f32;
    let ratio: f32;
    rg.pad_x = 0;
    rg.pad_y = 0;
    if fw / dw >= fh / dh {
        ratio = fh / dh;
        rg.pad_x = ((fw - (dw * ratio)) / 2.0) as i32;
    } else {
        ratio = fw / dw;
        rg.pad_y = ((fh - (dh * ratio)) / 2.0) as i32;
    }
    Ok(())
}

pub fn placeholder_tex_id() -> Result<isize, NUError> {
    Ok(RenderGod::get()?.placeholder_tex_id as isize)
}

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

        if success == gl::FALSE.into() {
            let mut log_length: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut log = Vec::with_capacity(log_length as usize);
            log.set_len((log_length as usize) - 1); // subtract 1 to skip the null terminator

            gl::GetShaderInfoLog(
                shader,
                log_length,
                std::ptr::null_mut(),
                log.as_mut_ptr() as *mut GLchar,
            );

            let estr = std::ffi::CStr::from_ptr(log.as_ptr() as *const i8)
                .to_string_lossy()
                .into_owned();

            return Err(NUError::ShaderCompilationError(estr));
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
            let mut log_length: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut log = Vec::with_capacity(log_length as usize);
            log.set_len((log_length as usize) - 1); // subtract 1 to skip the null terminator

            gl::GetProgramInfoLog(
                program,
                log_length,
                std::ptr::null_mut(),
                log.as_mut_ptr() as *mut GLchar,
            );

            let estr = std::ffi::CStr::from_ptr(log.as_ptr() as *const i8)
                .to_string_lossy()
                .into_owned();

            return Err(NUError::ShaderCompilationError(estr));
        }
    }

    Ok(program)
}

fn vertex_attribute(
    shader_program: GLuint,
    attrib_name: CString,
    count: isize,
    vertex_size: isize,
    offset: isize,
) -> Result<GLint, NUError> {
    let location: GLint;

    unsafe {
        location = gl::GetAttribLocation(shader_program, attrib_name.as_ptr() as *const i8);
    }

    // this kinda sucks but,
    // if a parameter gets optimized away by the compiler,
    // just leave it. maybe only do this on debug build?
    if location == -1 {
        return Ok(-1);
    }

    unsafe {
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

    loop {
        let s = unsafe { gl::GetError() };
        if s == gl::NO_ERROR {
            break;
        }
        eprintln!("glerror: {:x}", s);
        panic!();
    }

    Ok(location)
}

pub fn init() -> Result<(), NUError> {
    if RenderGod::get().is_ok() {
        return Err(NUError::MiscError("RENDER_GOD already init".to_string()));
    }

    let rg = RenderGod {
        // global collections
        draw_calls: vec![],
        textures: vec![],

        // 8 properties per vert [x,y,z, u,v, nx,ny,nz]
        r_buffer: vec![0.; MAX_VERTS * 8]
            .into_boxed_slice()
            .try_into()
            .unwrap(),
        r_num_verts: 0,
        verts_changed: true,
        // 2 vec3 per light [(x,y,z), [r,g,b], ...]
        r_light_buffer: vec![0.; MAX_LIGHT_V3S * 3]
            .into_boxed_slice()
            .try_into()
            .unwrap(),
        r_num_lights: 0,

        // shader stuff
        shader_program: 0,
        vertex_buffer: 0,
        vao: 0,

        // camera bits
        camera_position: Vector3 {
            x: 0.,
            y: 0.,
            z: -50.,
        },
        camera_pitch: 0.,
        camera_yaw: 0.,

        // uniforms
        u_camera_pos: 0,
        u_lights: 0,
        u_light_count: 0,
        u_mouse: 0,
        u_model_mat_v1: 0,
        u_model_mat_v2: 0,
        u_model_mat_v3: 0,
        u_model_mat_v4: 0,
        u_blend: 0,
        u_glow: 0,

        // vertex attribute location for mixing
        va_p2: 0,
        va_n2: 0,

        default_fbo: 0,
        offscreen_fbo: 0,
        offscreen_color_tex: 0,
        offscreen_depth_tex: 0,

        pad_x: 0,
        pad_y: 0,
        current_window_width: D_WINDOW_W as i32,
        current_window_height: D_WINDOW_H as i32,

        placeholder_tex_id: 0,

        debug_shader_program: 0,
        debug_vbo: 0,
        debug_vao: 0,
        debug_u_camera_pos: 0,
        debug_u_mouse: 0,
        debug_u_color: 0,
        debug_verts: vec![],
        debug_cmds: vec![],
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
        rg.u_camera_pos =
            gl::GetUniformLocation(rg.shader_program, CString::new("camera_pos")?.as_ptr());
        rg.u_lights = gl::GetUniformLocation(rg.shader_program, CString::new("lights")?.as_ptr());
        rg.u_light_count =
            gl::GetUniformLocation(rg.shader_program, CString::new("light_count")?.as_ptr());
        rg.u_mouse = gl::GetUniformLocation(rg.shader_program, CString::new("mouse")?.as_ptr());
        rg.u_model_mat_v1 =
            gl::GetUniformLocation(rg.shader_program, CString::new("model_mat_v1")?.as_ptr());
        rg.u_model_mat_v2 =
            gl::GetUniformLocation(rg.shader_program, CString::new("model_mat_v2")?.as_ptr());
        rg.u_model_mat_v3 =
            gl::GetUniformLocation(rg.shader_program, CString::new("model_mat_v3")?.as_ptr());
        rg.u_model_mat_v4 =
            gl::GetUniformLocation(rg.shader_program, CString::new("model_mat_v4")?.as_ptr());
        rg.u_blend = gl::GetUniformLocation(rg.shader_program, CString::new("blend")?.as_ptr());
        rg.u_glow = gl::GetUniformLocation(rg.shader_program, CString::new("glow")?.as_ptr());
    }

    // vertex array object - captures attribute state so we don't re-set it each frame
    unsafe {
        gl::GenVertexArrays(1, addr_of_mut!(rg.vao));
        gl::BindVertexArray(rg.vao);
    }

    // vertex buffer
    unsafe {
        // fuck if I know what addr_of_mut does
        gl::GenBuffers(1, addr_of_mut!(rg.vertex_buffer));
        gl::BindBuffer(gl::ARRAY_BUFFER, rg.vertex_buffer);
    }

    // vertex attribute initialization
    // I don't remember why these first 3 don't get assigned to something
    vertex_attribute(rg.shader_program, CString::new("p")?, 3, 8, 0)?;
    vertex_attribute(rg.shader_program, CString::new("t")?, 2, 8, 3)?;
    vertex_attribute(rg.shader_program, CString::new("n")?, 3, 8, 5)?;
    rg.va_p2 = vertex_attribute(rg.shader_program, CString::new("p2")?, 3, 8, 0)?;
    rg.va_n2 = vertex_attribute(rg.shader_program, CString::new("n2")?, 3, 8, 5)?;

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

    rg.placeholder_tex_id = create_texture(PngBin {
        data: PLACEHOLDER_PNG.to_vec(),
    })?;

    // debug shader
    rg.debug_shader_program = create_program(
        compile_shader(gl::VERTEX_SHADER, DEBUG_V_SHADER_STR)?,
        compile_shader(gl::FRAGMENT_SHADER, DEBUG_F_SHADER_STR)?,
    )?;
    unsafe {
        rg.debug_u_camera_pos =
            gl::GetUniformLocation(rg.debug_shader_program, CString::new("camera_pos")?.as_ptr());
        rg.debug_u_mouse =
            gl::GetUniformLocation(rg.debug_shader_program, CString::new("mouse")?.as_ptr());
        rg.debug_u_color =
            gl::GetUniformLocation(rg.debug_shader_program, CString::new("color")?.as_ptr());
    }

    // debug VAO/VBO
    unsafe {
        gl::GenVertexArrays(1, addr_of_mut!(rg.debug_vao));
        gl::BindVertexArray(rg.debug_vao);
        gl::GenBuffers(1, addr_of_mut!(rg.debug_vbo));
        gl::BindBuffer(gl::ARRAY_BUFFER, rg.debug_vbo);
        let loc = gl::GetAttribLocation(rg.debug_shader_program, CString::new("p")?.as_ptr());
        gl::EnableVertexAttribArray(loc as u32);
        gl::VertexAttribPointer(loc as u32, 3, gl::FLOAT, gl::FALSE, 3 * 4, 0 as *const GLvoid);
        // restore main VAO
        gl::BindVertexArray(rg.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, rg.vertex_buffer);
    }

    Ok(())
}

pub fn get_r_num_verts() -> Result<usize, NUError> {
    Ok(RenderGod::get()?.r_num_verts)
}

pub struct PngBin {
    pub data: Vec<u8>,
}

pub fn create_texture(p: PngBin) -> Result<usize, NUError> {
    let header =
        minipng::decode_png_header(&p.data).map_err(|e| NUError::MiniPNGError(e.to_string()))?;
    let mut buffer = vec![0; header.required_bytes()];
    let (width, height) = match minipng::decode_png(&p.data, &mut buffer)
        .map_err(|e| NUError::MiniPNGError(e.to_string()))
    {
        Ok(i) => (i.width(), i.height()),
        Err(e) => {
            eprintln!(
                "create_texture failed: {} {} {}",
                e.to_string(),
                p.data.len(),
                buffer.len()
            );
            buffer = PLACEHOLDER_PNG.to_vec();
            (1, 1)
        }
    };

    let mut texture: GLuint = 0;

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            buffer.as_ptr() as *const GLvoid,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_NEAREST as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    let ts: &mut Vec<_> = RenderGod::get()?.textures.as_mut();

    ts.push(MetaTex {
        texture,
        width,
        height,
    });

    Ok(ts.len() - 1)
}

pub fn prepare_frame() -> Result<(), NUError> {
    let num_lights = &mut RenderGod::get()?.r_num_lights;
    *num_lights = 0;

    unsafe {
        gl::ClearColor(0., 0., 0., 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    text::prepare_frame()
}

pub fn end_frame() -> Result<(), NUError> {
    let rg: &mut RenderGod = RenderGod::get()?;

    // todo -- works, buuuuut, only do it on r_num_verts change
    // otherwise, wasteful copy to gpu
    let verts_changed = &mut RenderGod::get()?.verts_changed;
    if *verts_changed {
        submit_buffer()?;
        *verts_changed = false;
    }

    unsafe {
        gl::UseProgram(rg.shader_program);
        gl::BindVertexArray(rg.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, rg.vertex_buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, rg.offscreen_fbo);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    unsafe {
        gl::Uniform4f(
            rg.u_camera_pos,
            rg.camera_position.x,
            rg.camera_position.y,
            rg.camera_position.z,
            16.0 / 9.0,
        );
        gl::Uniform2f(rg.u_mouse, rg.camera_yaw, rg.camera_pitch);
        gl::Uniform3fv(
            rg.u_lights,
            (rg.r_num_lights * 3 * 2) as i32,
            rg.r_light_buffer.as_ptr() as *const GLfloat,
        );
        gl::Uniform1i(rg.u_light_count, (rg.r_num_lights * 2) as i32);
    }

    // have to initialize to non-zero
    // for vo != (c.f2 - c.f1) check later
    let mut vo: GLint = GLint::MIN;
    let mut last_texture: u32 = u32::MAX - 1;

    // sort draw calls by texture to reduce texture
    // bind time
    // ===============================================
    // rg.draw_calls.sort_by(|a, b| a.texture.cmp(&b.texture));

    // todo, try some optimizations here?
    // sort by distance to cam?
    // discard by distance to cam?
    // ===============================================
    // let cam = rg.camera_position;
    // let max_dist_sq = 50. * 50.;
    // let draw_calls: Vec<&DrawCall> = rg.draw_calls.iter()
    //     .map(|c| {
    //         (c, (c.matrix.m12 - cam.x).powi(2) + (c.matrix.m13 - cam.y).powi(2) + (c.matrix.m14 - cam.z).powi(2))
    //     })
    //     .filter(|(_, dist_sq)| *dist_sq <= max_dist_sq)
    //     .map(|(c, _)| c)
    //     .collect();

    // todo, add a flag to draw calls, that the texture
    // has transparency, then put those last?

    for c in &rg.draw_calls {
        if last_texture != c.texture {
            last_texture = c.texture;
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, rg.textures[last_texture as usize].texture);
            }
        }

        let model_f16: [f32; 16] = c.matrix.into();

        unsafe {
            gl::Uniform4f(
                rg.u_model_mat_v1,
                model_f16[0x0],
                model_f16[0x1],
                model_f16[0x2],
                model_f16[0x3],
            );
            gl::Uniform4f(
                rg.u_model_mat_v2,
                model_f16[0x4],
                model_f16[0x5],
                model_f16[0x6],
                model_f16[0x7],
            );
            gl::Uniform4f(
                rg.u_model_mat_v3,
                model_f16[0x8],
                model_f16[0x9],
                model_f16[0xA],
                model_f16[0xB],
            );
            gl::Uniform4f(
                rg.u_model_mat_v4,
                model_f16[0xC],
                model_f16[0xD],
                model_f16[0xE],
                model_f16[0xF],
            );
            gl::Uniform1f(rg.u_blend, c.mix);

            let glow = match c.glow {
                None => Vector3::new(f32::NAN, f32::NAN, f32::NAN),
                Some(v) => v,
            };

            gl::Uniform3f(rg.u_glow, glow.x, glow.y, glow.z);
        }

        if vo != (c.f2 - c.f1) {
            vo = c.f2 - c.f1;
            unsafe {
                if rg.va_p2 != -1 {
                    gl::VertexAttribPointer(
                        rg.va_p2 as u32,
                        3,
                        gl::FLOAT,
                        gl::FALSE,
                        8 * 4,
                        (vo * 8 * 4) as *const GLvoid,
                    );
                }
                if rg.va_n2 != -1 {
                    gl::VertexAttribPointer(
                        rg.va_n2 as u32,
                        3,
                        gl::FLOAT,
                        gl::FALSE,
                        8 * 4,
                        ((vo * 8 + 5) * 4) as *const GLvoid,
                    );
                }
            }
        }

        unsafe {
            // todo, use EBO, and switch to
            // gl::DrawElements()
            // this should save ram, vram, and hopefully also gpu vertex data copy time
            gl::DrawArrays(gl::TRIANGLES, c.f1, c.num_verts as i32);
        }
    }

    // debug draws
    if !rg.debug_cmds.is_empty() {
        unsafe {
            gl::UseProgram(rg.debug_shader_program);
            gl::BindVertexArray(rg.debug_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, rg.debug_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (rg.debug_verts.len() * std::mem::size_of::<f32>()) as isize,
                rg.debug_verts.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW,
            );
            gl::Uniform4f(
                rg.debug_u_camera_pos,
                rg.camera_position.x,
                rg.camera_position.y,
                rg.camera_position.z,
                16.0 / 9.0,
            );
            gl::Uniform2f(rg.debug_u_mouse, rg.camera_yaw, rg.camera_pitch);
            gl::Disable(gl::CULL_FACE);
        }
        for cmd in &rg.debug_cmds {
            unsafe {
                gl::Uniform3f(rg.debug_u_color, cmd.color[0], cmd.color[1], cmd.color[2]);
                gl::DrawArrays(cmd.primitive, cmd.start, cmd.count);
            }
        }
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::UseProgram(rg.shader_program);
            gl::BindVertexArray(rg.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, rg.vertex_buffer);
        }
        rg.debug_verts.clear();
        rg.debug_cmds.clear();
    }

    // todo works here, but eeeeggghhhh
    // would rather put it after the blit
    text::end_frame()?;

    unsafe {
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, rg.offscreen_fbo);
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, rg.default_fbo as u32);
        gl::BlitFramebuffer(
            0,
            0,
            INTERNAL_W,
            INTERNAL_H,
            rg.pad_x,
            rg.pad_y,
            rg.current_window_width - rg.pad_x,
            rg.current_window_height - rg.pad_y,
            gl::COLOR_BUFFER_BIT,
            gl::NEAREST,
        );
    }

    rg.draw_calls = Vec::new();

    Ok(())
}

pub fn draw(d: DrawCall) -> Result<(), NUError> {
    RenderGod::get()?.draw_calls.push(d);
    Ok(())
}

pub fn submit_buffer() -> Result<(), NUError> {
    let rb = &RenderGod::get()?.r_buffer;
    let vb = &RenderGod::get()?.vertex_buffer;
    let nv = RenderGod::get()?.r_num_verts;
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, *vb);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (nv * 8 * std::mem::size_of::<f32>()) as isize,
            rb.as_ptr() as *const GLvoid,
            gl::DYNAMIC_DRAW,
        );
    }
    Ok(())
}

pub fn push_vert(pos: Vector3, normal: Vector3, u: f32, v: f32) -> Result<usize, NUError> {
    let num_verts = RenderGod::get()?.r_num_verts;

    let vindex = num_verts * 8;

    let r_buffer = &mut RenderGod::get()?.r_buffer;

    r_buffer[vindex + 0] = pos.x;
    r_buffer[vindex + 1] = pos.y;
    r_buffer[vindex + 2] = pos.z;
    r_buffer[vindex + 3] = u;
    r_buffer[vindex + 4] = v;
    r_buffer[vindex + 5] = normal.x;
    r_buffer[vindex + 6] = normal.y;
    r_buffer[vindex + 7] = normal.z;

    RenderGod::get()?.r_num_verts += 1;
    RenderGod::get()?.verts_changed = true;

    Ok(num_verts)
}

pub fn _push_quad(
    v0: Vector3,
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
    u: f32,
    v: f32,
) -> Result<usize, NUError> {
    let num_verts = RenderGod::get()?.r_num_verts;

    let n = math::vec3_face_normal(v0, v1, v2);
    push_vert(v0, n, u, 0.)?;
    push_vert(v1, n, 0., 0.)?;
    push_vert(v2, n, u, v)?;
    push_vert(v3, n, 0., v)?;
    push_vert(v2, n, u, v)?;
    push_vert(v1, n, 0., 0.)?;

    Ok(num_verts)
}

pub fn _push_block(
    x: f32,
    y: f32,
    z: f32,
    sx: f32,
    sy: f32,
    sz: f32,
    texture: isize,
) -> Result<usize, NUError> {
    let tex = &RenderGod::get()?.textures[texture as usize];
    let tex_w = tex.width;
    let tex_h = tex.height;
    let index = RenderGod::get()?.r_num_verts;

    let tx = sx / tex_w as f32;
    let ty = sy / tex_h as f32;
    let tz = sz / tex_w as f32;

    // top
    let v0 = Vector3::new(x, y + sy, z);
    let v1 = Vector3::new(x + sx, y + sy, z);
    let v2 = Vector3::new(x, y + sy, z + sz);
    let v3 = Vector3::new(x + sx, y + sy, z + sz);

    // bottom
    let v4 = Vector3::new(x, y, z + sz);
    let v5 = Vector3::new(x + sx, y, z + sz);
    let v6 = Vector3::new(x, y, z);
    let v7 = Vector3::new(x + sx, y, z);

    _push_quad(v0, v1, v2, v3, tx, tz)?; // top
    _push_quad(v4, v5, v6, v7, tx, tz)?; // bottom
    _push_quad(v2, v3, v4, v5, tx, ty)?; // front
    _push_quad(v1, v0, v7, v6, tx, ty)?; // back
    _push_quad(v3, v1, v5, v7, tz, ty)?; // right
    _push_quad(v0, v2, v6, v4, tz, ty)?; // left

    Ok(index)
}

// todo, find intensity & rgb upper bounds
//
// todo, there are no upper bounds, it's all normalized on the gpu,
// I don't really even get what the intensity is for here, except that it's
// not effected by the fade.
//
// todo, it's honestly so fucking stupid. intensity can here be 0-255, but then we should
// do `let fade = math::scale([...]) * (intensity as f32 / 255) * 100`
//
// this latter * 100 can actually be put in the shader as
// (l[i+1] * vec3(100,100,100));

// also here's this shit --
/*
    // Calculate light contribution
    for (int i = 0; i < R_MAX_LIGHT_V3; i += 2) {
        // Light direction vector
        vec3 lightDir = normalize(l[i] - vp);

        // Angle to normal
        float angle = max(dot(vn, lightDir), 0.0);

        // Inverse distance squared attenuation
        float attenuation = 1.0 / pow(length(l[i] - vp), 2.0);

        // Accumulate light contribution
        vl += angle * attenuation * l[i + 1];
    }
*/
pub fn push_light(pos: Vector3, intensity: u8, r: u8, g: u8, b: u8) -> Result<(), NUError> {
    let cam_pos = RenderGod::get()?.camera_position;
    let r_num_lights = &mut RenderGod::get()?.r_num_lights;
    let r_light_buffer = &mut RenderGod::get()?.r_light_buffer;

    // Calculate the distance to the light, fade it out between 16--32
    let start_fade_dist = 16.;
    let end_fade_dist = 32.;
    let cam_light_dist = vector3_distance(pos, cam_pos);

    // past max fade distance, skip the push entirely
    if cam_light_dist >= end_fade_dist {
        return Ok(());
    }

    let fade = math::scale(cam_light_dist, start_fade_dist, end_fade_dist, 1., 0.).clamp(0., 1.)
        * intensity as f32
        / 255.;

    if *r_num_lights * 2 >= MAX_LIGHT_V3S {
        eprintln!("max lights reached");
        return Ok(());
    }

    let lindex = *r_num_lights * 6;
    r_light_buffer[lindex + 0] = pos.x;
    r_light_buffer[lindex + 1] = pos.y;
    r_light_buffer[lindex + 2] = pos.z;
    r_light_buffer[lindex + 3] = r as f32 * fade;
    r_light_buffer[lindex + 4] = g as f32 * fade;
    r_light_buffer[lindex + 5] = b as f32 * fade;

    *r_num_lights += 1;

    Ok(())
}

pub fn quit() {
    unsafe {
        RENDER_GOD = None;
    }
}

pub fn get_camera_pos() -> Result<Vector3, NUError> {
    let rg = RenderGod::get()?;
    Ok(rg.camera_position)
}

pub fn set_camera_pos(pos: Vector3) -> Result<(), NUError> {
    let rg = RenderGod::get()?;
    rg.camera_position = pos;
    Ok(())
}

pub fn set_camera_pitch(f: f32) -> Result<(), NUError> {
    let rg = RenderGod::get()?;
    rg.camera_pitch = f;
    Ok(())
}

pub fn get_camera_pitch() -> Result<f32, NUError> {
    let rg = RenderGod::get()?;
    Ok(rg.camera_pitch)
}

pub fn set_camera_yaw(f: f32) -> Result<(), NUError> {
    let rg = RenderGod::get()?;
    rg.camera_yaw = f;
    Ok(())
}

pub fn get_camera_yaw() -> Result<f32, NUError> {
    let rg = RenderGod::get()?;
    Ok(rg.camera_yaw)
}

pub fn push_debug_point(pos: Vector3, r: f32, g: f32, b: f32) -> Result<(), NUError> {
    let rg = RenderGod::get()?;
    let start = (rg.debug_verts.len() / 3) as GLint;
    // actually a small line
    rg.debug_verts.extend_from_slice(&[pos.x, pos.y, pos.z, pos.x + 0.01, pos.y + 0.01, pos.z + 0.01]);
    rg.debug_cmds.push(DebugDrawCmd {
        primitive: gl::LINES,
        start,
        count: 2,
        color: [r, g, b],
    });
    Ok(())
}

pub fn push_debug_line(v1: Vector3, v2: Vector3, r: f32, g: f32, b: f32) -> Result<(), NUError> {
    let rg = RenderGod::get()?;
    let start = (rg.debug_verts.len() / 3) as GLint;
    rg.debug_verts.extend_from_slice(&[v1.x, v1.y, v1.z, v2.x, v2.y, v2.z]);
    rg.debug_cmds.push(DebugDrawCmd {
        primitive: gl::LINES,
        start,
        count: 2,
        color: [r, g, b],
    });
    Ok(())
}

pub fn push_debug_circle(center: Vector3, radius: f32, rotation_axis: Vector3, rotation_angle: f32, color: [f32; 3]) -> Result<(), NUError>
{
    let rot = raymath::matrix_rotate(rotation_axis, rotation_angle);
    let segments = 36;
    let step = 360.0 / segments as f32;

    for i in 0..segments {
        let a0 = (i as f32 * step).to_radians();
        let a1 = ((i + 1) as f32 * step).to_radians();

        let p0 = math::vector3_transform(
            Vector3::new(a0.sin() * radius, a0.cos() * radius, 0.0), rot,
        );
        let p1 = math::vector3_transform(
            Vector3::new(a1.sin() * radius, a1.cos() * radius, 0.0), rot,
        );

        let v0 = math::vector3_add(center, p0);
        let v1 = math::vector3_add(center, p1);

        push_debug_line(v0, v1, color[0], color[1], color[2])?;
    }

    Ok(())
}

pub fn push_debug_triangle(
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
    r: f32,
    g: f32,
    b: f32,
) -> Result<(), NUError> {
    let rg = RenderGod::get()?;
    let start = (rg.debug_verts.len() / 3) as GLint;
    rg.debug_verts.extend_from_slice(&[v1.x, v1.y, v1.z, v2.x, v2.y, v2.z, v3.x, v3.y, v3.z]);
    rg.debug_cmds.push(DebugDrawCmd {
        primitive: gl::TRIANGLES,
        start,
        count: 3,
        color: [r, g, b],
    });
    Ok(())
}

pub fn push_debug_cube(position: Vector3, width: f32, height: f32, length: f32, color: [f32; 3]) -> Result<(), NUError> {
    let p = position;
    let hw = width / 2.0;
    let hh = height / 2.0;
    let hl = length / 2.0;

    let tri = |a: Vector3, b: Vector3, c: Vector3| -> Result<(), NUError> {
        push_debug_triangle(a, b, c, color[0], color[1], color[2])?;
        Ok(())
    };

    // front
    tri(Vector3::new(p.x - hw, p.y - hh, p.z + hl), Vector3::new(p.x + hw, p.y - hh, p.z + hl), Vector3::new(p.x - hw, p.y + hh, p.z + hl))?;
    tri(Vector3::new(p.x + hw, p.y + hh, p.z + hl), Vector3::new(p.x - hw, p.y + hh, p.z + hl), Vector3::new(p.x + hw, p.y - hh, p.z + hl))?;
    // back
    tri(Vector3::new(p.x - hw, p.y - hh, p.z - hl), Vector3::new(p.x - hw, p.y + hh, p.z - hl), Vector3::new(p.x + hw, p.y - hh, p.z - hl))?;
    tri(Vector3::new(p.x + hw, p.y + hh, p.z - hl), Vector3::new(p.x + hw, p.y - hh, p.z - hl), Vector3::new(p.x - hw, p.y + hh, p.z - hl))?;
    // top
    tri(Vector3::new(p.x - hw, p.y + hh, p.z - hl), Vector3::new(p.x - hw, p.y + hh, p.z + hl), Vector3::new(p.x + hw, p.y + hh, p.z + hl))?;
    tri(Vector3::new(p.x + hw, p.y + hh, p.z - hl), Vector3::new(p.x - hw, p.y + hh, p.z - hl), Vector3::new(p.x + hw, p.y + hh, p.z + hl))?;
    // bottom
    tri(Vector3::new(p.x - hw, p.y - hh, p.z - hl), Vector3::new(p.x + hw, p.y - hh, p.z + hl), Vector3::new(p.x - hw, p.y - hh, p.z + hl))?;
    tri(Vector3::new(p.x + hw, p.y - hh, p.z - hl), Vector3::new(p.x + hw, p.y - hh, p.z + hl), Vector3::new(p.x - hw, p.y - hh, p.z - hl))?;
    // right
    tri(Vector3::new(p.x + hw, p.y - hh, p.z - hl), Vector3::new(p.x + hw, p.y + hh, p.z - hl), Vector3::new(p.x + hw, p.y + hh, p.z + hl))?;
    tri(Vector3::new(p.x + hw, p.y - hh, p.z + hl), Vector3::new(p.x + hw, p.y - hh, p.z - hl), Vector3::new(p.x + hw, p.y + hh, p.z + hl))?;
    // left
    tri(Vector3::new(p.x - hw, p.y - hh, p.z - hl), Vector3::new(p.x - hw, p.y + hh, p.z + hl), Vector3::new(p.x - hw, p.y + hh, p.z - hl))?;
    tri(Vector3::new(p.x - hw, p.y - hh, p.z + hl), Vector3::new(p.x - hw, p.y + hh, p.z + hl), Vector3::new(p.x - hw, p.y - hh, p.z - hl))
}

pub fn push_debug_cube_wires(position: Vector3, width: f32, height: f32, length: f32, color: [f32; 3]) -> Result<(), NUError> {
    let p = position;
    let hw = width / 2.0;
    let hh = height / 2.0;
    let hl = length / 2.0;

    let line = |a: Vector3, b: Vector3| -> Result<(), NUError> {
        push_debug_line(a, b, color[0], color[1], color[2])
    };

    // front
    line(Vector3::new(p.x - hw, p.y - hh, p.z + hl), Vector3::new(p.x + hw, p.y - hh, p.z + hl))?;
    line(Vector3::new(p.x + hw, p.y - hh, p.z + hl), Vector3::new(p.x + hw, p.y + hh, p.z + hl))?;
    line(Vector3::new(p.x + hw, p.y + hh, p.z + hl), Vector3::new(p.x - hw, p.y + hh, p.z + hl))?;
    line(Vector3::new(p.x - hw, p.y + hh, p.z + hl), Vector3::new(p.x - hw, p.y - hh, p.z + hl))?;
    // back
    line(Vector3::new(p.x - hw, p.y - hh, p.z - hl), Vector3::new(p.x + hw, p.y - hh, p.z - hl))?;
    line(Vector3::new(p.x + hw, p.y - hh, p.z - hl), Vector3::new(p.x + hw, p.y + hh, p.z - hl))?;
    line(Vector3::new(p.x + hw, p.y + hh, p.z - hl), Vector3::new(p.x - hw, p.y + hh, p.z - hl))?;
    line(Vector3::new(p.x - hw, p.y + hh, p.z - hl), Vector3::new(p.x - hw, p.y - hh, p.z - hl))?;
    // edges
    line(Vector3::new(p.x - hw, p.y + hh, p.z + hl), Vector3::new(p.x - hw, p.y + hh, p.z - hl))?;
    line(Vector3::new(p.x + hw, p.y + hh, p.z + hl), Vector3::new(p.x + hw, p.y + hh, p.z - hl))?;
    line(Vector3::new(p.x - hw, p.y - hh, p.z + hl), Vector3::new(p.x - hw, p.y - hh, p.z - hl))?;
    line(Vector3::new(p.x + hw, p.y - hh, p.z + hl), Vector3::new(p.x + hw, p.y - hh, p.z - hl))
}

pub fn push_debug_sphere(center: Vector3, radius: f32, rings: i32, slices: i32, color: [f32; 3]) -> Result<(), NUError> {
    let ringangle = (180.0f32 / (rings + 1) as f32).to_radians();
    let sliceangle = (360.0f32 / slices as f32).to_radians();

    let cosring = ringangle.cos();
    let sinring = ringangle.sin();
    let cosslice = sliceangle.cos();
    let sinslice = sliceangle.sin();

    let mut verts = [Vector3::new(0., 0., 0.); 4];
    verts[2] = Vector3::new(0., 1., 0.);
    verts[3] = Vector3::new(sinring, cosring, 0.);

    for _ in 0..rings + 1 {
        for _ in 0..slices {
            verts[0] = verts[2];
            verts[1] = verts[3];
            verts[2] = Vector3::new(cosslice * verts[2].x - sinslice * verts[2].z, verts[2].y, sinslice * verts[2].x + cosslice * verts[2].z);
            verts[3] = Vector3::new(cosslice * verts[3].x - sinslice * verts[3].z, verts[3].y, sinslice * verts[3].x + cosslice * verts[3].z);

            let v = |i: usize| Vector3::new(center.x + verts[i].x * radius, center.y + verts[i].y * radius, center.z + verts[i].z * radius);
            push_debug_triangle(v(0), v(3), v(1), color[0], color[1], color[2])?;
            push_debug_triangle(v(0), v(2), v(3), color[0], color[1], color[2])?;
        }
        verts[2] = verts[3];
        verts[3] = Vector3::new(cosring * verts[3].x + sinring * verts[3].y, -sinring * verts[3].x + cosring * verts[3].y, verts[3].z);
    }

    Ok(())
}

pub fn push_debug_sphere_wires(center: Vector3, radius: f32, rings: i32, slices: i32, color: [f32; 3])-> Result<(), NUError> {
    for i in 0..rings + 2 {
        for j in 0..slices {
            let ring_a = (270.0 + (180.0 / (rings + 1) as f32) * i as f32).to_radians();
            let ring_b = (270.0 + (180.0 / (rings + 1) as f32) * (i + 1) as f32).to_radians();
            let slice_a = (360.0 * j as f32 / slices as f32).to_radians();
            let slice_b = (360.0 * (j + 1) as f32 / slices as f32).to_radians();

            let vert = |ring: f32, slice: f32| Vector3::new(
                center.x + ring.cos() * slice.sin() * radius,
                center.y + ring.sin() * radius,
                center.z + ring.cos() * slice.cos() * radius,
            );

            // diagonal
            push_debug_line(vert(ring_a, slice_a), vert(ring_b, slice_b), color[0], color[1], color[2])?;
            // horizontal
            push_debug_line(vert(ring_b, slice_b), vert(ring_b, slice_a), color[0], color[1], color[2])?;
            // vertical
            push_debug_line(vert(ring_b, slice_a), vert(ring_a, slice_a), color[0], color[1], color[2])?;
        }
    }
    Ok(())
}

pub fn push_debug_cylinder(start_pos: Vector3, end_pos: Vector3, start_radius: f32, end_radius: f32, sides: i32, color: [f32; 3])-> Result<(), NUError> {
    let sides = sides.max(3);
    let direction = math::vector3_subtract(end_pos, start_pos);
    if vector3_length(direction) == 0. { 
        return Err(NUError::MiscError("cylinder has no direction".to_string()));
    }

    let b1 = raymath::vector3_normalize(raymath::vector3_perpendicular(direction));
    let b2 = raymath::vector3_normalize(raymath::vector3_cross_product(b1, direction));
    let base_angle = std::f32::consts::TAU / sides as f32;

    for i in 0..sides {
        let s1 = (base_angle * i as f32).sin() * start_radius;
        let c1 = (base_angle * i as f32).cos() * start_radius;
        let w1 = Vector3::new(start_pos.x + s1*b1.x + c1*b2.x, start_pos.y + s1*b1.y + c1*b2.y, start_pos.z + s1*b1.z + c1*b2.z);
        let s2 = (base_angle * (i+1) as f32).sin() * start_radius;
        let c2 = (base_angle * (i+1) as f32).cos() * start_radius;
        let w2 = Vector3::new(start_pos.x + s2*b1.x + c2*b2.x, start_pos.y + s2*b1.y + c2*b2.y, start_pos.z + s2*b1.z + c2*b2.z);
        let s3 = (base_angle * i as f32).sin() * end_radius;
        let c3 = (base_angle * i as f32).cos() * end_radius;
        let w3 = Vector3::new(end_pos.x + s3*b1.x + c3*b2.x, end_pos.y + s3*b1.y + c3*b2.y, end_pos.z + s3*b1.z + c3*b2.z);
        let s4 = (base_angle * (i+1) as f32).sin() * end_radius;
        let c4 = (base_angle * (i+1) as f32).cos() * end_radius;
        let w4 = Vector3::new(end_pos.x + s4*b1.x + c4*b2.x, end_pos.y + s4*b1.y + c4*b2.y, end_pos.z + s4*b1.z + c4*b2.z);

        if start_radius > 0.0 {
            push_debug_triangle(start_pos, w2, w1, color[0], color[1], color[2])?;
        }
        push_debug_triangle(w1, w2, w3, color[0], color[1], color[2])?;
        push_debug_triangle(w2, w4, w3, color[0], color[1], color[2])?;
        if end_radius > 0.0 {
            push_debug_triangle(end_pos, w3, w4, color[0], color[1], color[2])?;
        }
    }

    Ok(())
}

pub fn push_debug_cylinder_wires(start_pos: Vector3, end_pos: Vector3, start_radius: f32, end_radius: f32, sides: i32, color: [f32; 3]) -> Result<(), NUError>  {
    let sides = sides.max(3);
    let direction = math::vector3_subtract(end_pos, start_pos);
    if vector3_length(direction) == 0. { 
        return Err(NUError::MiscError("cylinder has no direction".to_string()));
    }

    let b1 = raymath::vector3_normalize(raymath::vector3_perpendicular(direction));
    let b2 = raymath::vector3_normalize(raymath::vector3_cross_product(b1, direction));
    let base_angle = std::f32::consts::TAU / sides as f32;

    for i in 0..sides {
        let s1 = (base_angle * i as f32).sin() * start_radius;
        let c1 = (base_angle * i as f32).cos() * start_radius;
        let w1 = Vector3::new(start_pos.x + s1*b1.x + c1*b2.x, start_pos.y + s1*b1.y + c1*b2.y, start_pos.z + s1*b1.z + c1*b2.z);
        let s2 = (base_angle * (i+1) as f32).sin() * start_radius;
        let c2 = (base_angle * (i+1) as f32).cos() * start_radius;
        let w2 = Vector3::new(start_pos.x + s2*b1.x + c2*b2.x, start_pos.y + s2*b1.y + c2*b2.y, start_pos.z + s2*b1.z + c2*b2.z);
        let s3 = (base_angle * i as f32).sin() * end_radius;
        let c3 = (base_angle * i as f32).cos() * end_radius;
        let w3 = Vector3::new(end_pos.x + s3*b1.x + c3*b2.x, end_pos.y + s3*b1.y + c3*b2.y, end_pos.z + s3*b1.z + c3*b2.z);
        let s4 = (base_angle * (i+1) as f32).sin() * end_radius;
        let c4 = (base_angle * (i+1) as f32).cos() * end_radius;
        let w4 = Vector3::new(end_pos.x + s4*b1.x + c4*b2.x, end_pos.y + s4*b1.y + c4*b2.y, end_pos.z + s4*b1.z + c4*b2.z);

        push_debug_line(w1, w2, color[0], color[1], color[2])?;
        push_debug_line(w1, w3, color[0], color[1], color[2])?;
        push_debug_line(w3, w4, color[0], color[1], color[2])?;
    }

    Ok(())
}