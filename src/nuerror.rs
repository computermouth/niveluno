use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
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
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
}
