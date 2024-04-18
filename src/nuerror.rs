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
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    #[error{"Error building sdl2 window"}]
    WindowBuildError,
    #[error("Error initializing sdl2: {0}")]
    SDLError(String),
}

impl From<NUError> for String {
    fn from(e: NUError) -> String {
        e.to_string()
    }
}
