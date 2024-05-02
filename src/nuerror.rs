use minipng;
use std::ffi::NulError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
#[repr(u8)]
pub enum NUError {
    #[error{"Error loading shader"}]
    ShaderLoadError,
    #[error{"Error creating shader"}]
    ShaderCreateError,
    #[error("Error compiling shader: {0}")]
    ShaderCompilationError(String),
    #[error("Error linking shader: {0}")]
    ShaderLinkError(String),
    #[error{"Error creating shader program"}]
    ShaderProgramCreateError,
    #[error{"Error setting up vertex attribute data"}]
    VertexAttribError,
    #[error{"Error {0}"}]
    NulError(String),
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    #[error{"Error building sdl2 window"}]
    WindowBuildError,
    #[error("Error initializing sdl2: {0}")]
    SDLError(String),
    #[error("MiniPNG error: {0}")]
    MiniPNGError(String),
    #[error("Miscellaneous error: {0}")]
    MiscError(String),
}

impl From<NulError> for NUError {
    fn from(e: NulError) -> NUError {
        NUError::NulError(e.to_string())
    }
}

impl From<NUError> for String {
    fn from(e: NUError) -> String {
        e.to_string()
    }
}
