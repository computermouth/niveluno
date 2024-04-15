use std::ffi::CString;

use gl;
use gl::types::*;

use crate::nuerror::NUError;

const V_SHADER_STR: &str = include_str!("vert.glsl");
const F_SHADER_STR: &str = include_str!("frag.glsl");

fn compile_shader(shader_type: GLenum, shader_str: &str) -> Result<GLuint, NUError> {
    // https://dev.to/samkevich/learn-opengl-with-rust-shaders-28i3

    let mut shader: GLuint = 0;
    let _ = shader;
    let shader_cstr = CString::new(shader_str).map_err(|_| NUError::ShaderLoadError)?;

    unsafe {
        shader = gl::CreateShader(shader_type);
        if shader == 0 {
            return Err(NUError::ShaderCreateError);
        }
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

            error_log.set_len(error_log_size as usize);
            let e = NUError::ShaderCompilationError(String::from_utf8(error_log)?);
            eprintln!("{}", e);
            return Err(e);
        }
    }

    Ok(shader)
}

fn create_program(vert: GLuint, frag: GLuint) -> Result<GLuint, NUError> {
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

pub fn init() -> Result<(), NUError> {
    let program = create_program(
        compile_shader(gl::VERTEX_SHADER, V_SHADER_STR)?,
        compile_shader(gl::FRAGMENT_SHADER, F_SHADER_STR)?,
    )?;

    unsafe {
        gl::UseProgram(program);
    }

    Ok(())
}
