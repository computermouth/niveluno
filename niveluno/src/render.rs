use std::ffi::CString;
use std::ptr::addr_of_mut;

use minipng;

use gl;
use gl::types::*;

use crate::math::{self, Vec3};
use crate::nuerror::NUError;
use crate::text;

// draw texture res, default window res
pub const INTERNAL_W: i32 = 320;
pub const INTERNAL_H: i32 = 180;
pub const D_WINDOW_W: u32 = 640;
pub const D_WINDOW_H: u32 = 360;

const V_SHADER_STR: &str = include_str!("game_vert.glsl");
const F_SHADER_STR: &str = include_str!("game_frag.glsl");

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
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub texture: GLuint,
    pub f1: GLint, // todo, first frame of interpolation
    pub f2: GLint, // second frame of interpolation
    pub mix: f32,
    pub unlit: bool,
    pub num_verts: usize,
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
    // 2 vec3 per light [(x,y,z), [r,g,b], ...]
    // rename, light_array_buffer_data
    pub r_light_buffer: Box<[f32; MAX_LIGHT_V3S * 3]>,
    pub r_num_lights: usize,

    // shader stuff
    pub shader_program: GLuint,
    pub vertex_buffer: GLuint,

    // camera bits
    pub camera_position: Vec3,
    pub camera_pitch: GLfloat,
    pub camera_yaw: GLfloat,

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

    pub pad_x: i32,
    pub pad_y: i32,
    pub current_window_width: i32,
    pub current_window_height: i32,

    pub placeholder_tex_id: usize,
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
    attrib_name: CString,
    count: isize,
    vertex_size: isize,
    offset: isize,
) -> Result<GLint, NUError> {
    let mut location: GLint = -1;
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
        -1 => {
            let e = NUError::VertexAttribError;
            eprintln!("{}", e);
            Err(e)
        }
        _ => Ok(location),
    }
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
        // 2 vec3 per light [(x,y,z), [r,g,b], ...]
        r_light_buffer: vec![0.; MAX_LIGHT_V3S * 3]
            .into_boxed_slice()
            .try_into()
            .unwrap(),
        r_num_lights: 0,

        // shader stuff
        shader_program: 0,
        vertex_buffer: 0,

        // camera bits
        camera_position: Vec3 {
            x: 0.,
            y: 0.,
            z: -50.,
        },
        camera_pitch: 0.,
        camera_yaw: 0.,

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

        pad_x: 0,
        pad_y: 0,
        current_window_width: D_WINDOW_W as i32,
        current_window_height: D_WINDOW_H as i32,

        placeholder_tex_id: 0,
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
        rg.u_camera = gl::GetUniformLocation(rg.shader_program, CString::new("c")?.as_ptr());
        rg.u_lights = gl::GetUniformLocation(rg.shader_program, CString::new("l")?.as_ptr());
        rg.u_light_count =
            gl::GetUniformLocation(rg.shader_program, CString::new("light_count")?.as_ptr());
        rg.u_mouse = gl::GetUniformLocation(rg.shader_program, CString::new("m")?.as_ptr());
        // i think mp and mr are matrix_pos and matrix_rotation
        rg.u_pos = gl::GetUniformLocation(rg.shader_program, CString::new("mp")?.as_ptr());
        rg.u_rotation = gl::GetUniformLocation(rg.shader_program, CString::new("mr")?.as_ptr());
        rg.u_frame_mix = gl::GetUniformLocation(rg.shader_program, CString::new("f")?.as_ptr());
        rg.u_unlit = gl::GetUniformLocation(rg.shader_program, CString::new("unlit")?.as_ptr());
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

    // restore the vertex buffer and attributes
    unsafe {
        // i don't remember why this happens either, maybe unnecessary?
        // is this because of the other shader and switching?
        gl::BindBuffer(gl::ARRAY_BUFFER, rg.vertex_buffer);
        vertex_attribute(rg.shader_program, CString::new("p")?, 3, 8, 0)?;
        vertex_attribute(rg.shader_program, CString::new("t")?, 2, 8, 3)?;
        vertex_attribute(rg.shader_program, CString::new("n")?, 3, 8, 5)?;
        rg.va_p2 = vertex_attribute(rg.shader_program, CString::new("p2")?, 3, 8, 0)?;
        rg.va_n2 = vertex_attribute(rg.shader_program, CString::new("n2")?, 3, 8, 5)?;
    }

    rg.placeholder_tex_id = create_texture(PngBin {
        data: PLACEHOLDER_PNG.to_vec(),
    })?;

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

    // wohooo works, buuuuut, only do it on r_num_verts change
    // otherwise, wasteful copy to gpu
    submit_buffer()?;

    unsafe {
        gl::UseProgram(rg.shader_program);
        gl::BindBuffer(gl::ARRAY_BUFFER, rg.vertex_buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, rg.offscreen_fbo);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    vertex_attribute(rg.shader_program, CString::new("p")?, 3, 8, 0)?;
    vertex_attribute(rg.shader_program, CString::new("t")?, 2, 8, 3)?;
    vertex_attribute(rg.shader_program, CString::new("n")?, 3, 8, 5)?;
    rg.va_p2 = vertex_attribute(rg.shader_program, CString::new("p2")?, 3, 8, 0)?;
    rg.va_n2 = vertex_attribute(rg.shader_program, CString::new("n2")?, 3, 8, 5)?;

    unsafe {
        gl::Uniform4f(
            rg.u_camera,
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

    let mut vo: GLint = 0;
    let mut last_texture: u32 = u32::MAX - 1;

    let len = rg.draw_calls.len();
    // draw_call_t * draw_calls = vector_begin(r_draw_calls);
    // meta_tex_t * meta_texts = vector_begin(r_textures);
    for i in 0..len {
        // todo, raw index
        let c = rg.draw_calls[i];

        if last_texture != c.texture {
            last_texture = c.texture;
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, rg.textures[last_texture as usize].texture);
            }
        }

        unsafe {
            gl::Uniform3f(rg.u_pos, c.pos.x, c.pos.y, c.pos.z);
            gl::Uniform2f(rg.u_rotation, c.yaw, c.pitch);
            gl::Uniform1f(rg.u_frame_mix, c.mix);
            gl::Uniform1i(rg.u_unlit, c.unlit as i32);
        }

        if vo != (c.f2 - c.f1) {
            vo = c.f2 - c.f1;
            unsafe {
                gl::VertexAttribPointer(
                    rg.va_p2 as u32,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    8 * 4,
                    (vo * 8 * 4) as *const GLvoid,
                );
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

        unsafe {
            // todo, use EBO, and switch to
            // gl::DrawElements()
            // this should save ram, vram, and hopefully also gpu vertex data copy time
            gl::DrawArrays(gl::TRIANGLES, c.f1, c.num_verts as i32);
        }
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
            gl::STATIC_DRAW,
        );
    }
    Ok(())
}

pub fn push_vert(pos: Vec3, normal: Vec3, u: f32, v: f32) -> Result<usize, NUError> {
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

    Ok(num_verts)
}

pub fn push_quad(v0: Vec3, v1: Vec3, v2: Vec3, v3: Vec3, u: f32, v: f32) -> Result<usize, NUError> {
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

pub fn push_block(
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
    let v0 = Vec3::new(x, y + sy, z);
    let v1 = Vec3::new(x + sx, y + sy, z);
    let v2 = Vec3::new(x, y + sy, z + sz);
    let v3 = Vec3::new(x + sx, y + sy, z + sz);

    // bottom
    let v4 = Vec3::new(x, y, z + sz);
    let v5 = Vec3::new(x + sx, y, z + sz);
    let v6 = Vec3::new(x, y, z);
    let v7 = Vec3::new(x + sx, y, z);

    push_quad(v0, v1, v2, v3, tx, tz)?; // top
    push_quad(v4, v5, v6, v7, tx, tz)?; // bottom
    push_quad(v2, v3, v4, v5, tx, ty)?; // front
    push_quad(v1, v0, v7, v6, tx, ty)?; // back
    push_quad(v3, v1, v5, v7, tz, ty)?; // right
    push_quad(v0, v2, v6, v4, tz, ty)?; // left

    Ok(index)
}

pub fn push_light(pos: Vec3, intensity: f32, r: f32, g: f32, b: f32) -> Result<(), NUError> {
    let cam_pos = RenderGod::get()?.camera_position;
    let r_num_lights = &mut RenderGod::get()?.r_num_lights;
    let r_light_buffer = &mut RenderGod::get()?.r_light_buffer;

    // Calculate the distance to the light, fade it out between 768--1024
    let fade = math::scale(math::vec3_dist(pos, cam_pos), 768., 1024., 1., 0.).clamp(0., 1.)
        * intensity
        * 10.;

    // epsilon thing??
    // -MIN_POSITIVE < fade < MIN_POSITIVE ??
    if fade == 0. {
        return Ok(());
    }

    if *r_num_lights * 2 >= MAX_LIGHT_V3S {
        eprintln!("max lights reached");
        return Ok(());
    }

    let lindex = *r_num_lights * 6;
    r_light_buffer[lindex + 0] = pos.x;
    r_light_buffer[lindex + 1] = pos.y;
    r_light_buffer[lindex + 2] = pos.z;
    r_light_buffer[lindex + 3] = r * fade;
    r_light_buffer[lindex + 4] = g * fade;
    r_light_buffer[lindex + 5] = b * fade;

    *r_num_lights += 1;

    Ok(())
}

pub fn quit() {
    unsafe {
        RENDER_GOD = None;
    }
}
