
use std::{error::Error, ffi::CString};

use gl;

const V_SHADER_STR: &str = include_str!("vert.glsl");
const F_SHADER_STR: &str = include_str!("frag.glsl");

// todo, return result
fn compile_shader(shader_type: gl::types::GLenum, shader_str: &str) -> Option<gl::types::GLuint> {

    let mut shader = 0;
    let shader_cstr = CString::new(shader_str).ok();
    if shader_cstr.is_none() {
        return None;
    }
    let shader_cstr = shader_cstr.unwrap();

    unsafe {
        shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &shader_cstr.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    // todo, check compilation
    // https://dev.to/samkevich/learn-opengl-with-rust-shaders-28i3

    Some(shader)
}

fn init(){

    compile_shader(gl::VERTEX_SHADER, V_SHADER_STR);
    compile_shader(gl::FRAGMENT_SHADER, F_SHADER_STR);

}