use std::fmt;

#[derive(Debug)]
pub enum TextureLoadError {
    DecodeHires {
        path: String,
        source: image::ImageError,
    },
    DecodeMip {
        path: String,
        source: image::ImageError,
    },
}

impl fmt::Display for TextureLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextureLoadError::DecodeHires { path, source } => {
                write!(f, "failed to decode texture '{path}': {source}")
            }
            TextureLoadError::DecodeMip { path, source } => {
                write!(f, "failed to decode @2x mipmap for texture '{path}': {source}")
            }
        }
    }
}

impl std::error::Error for TextureLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TextureLoadError::DecodeHires { source, .. }
            | TextureLoadError::DecodeMip { source, .. } => Some(source),
        }
    }
}
